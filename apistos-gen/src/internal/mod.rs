use crate::internal::components::Components;
use crate::internal::operation::Operation;
use crate::operation_attr::OperationAttr;
use proc_macro_error::{abort, emit_error};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

use syn::{
  Expr, FnArg, Ident, ImplGenerics, ItemFn, Lit, Meta, Pat, ReturnType, Token, Type, TypeGenerics, TypeTraitObject,
  WhereClause,
};

mod components;
mod operation;

pub(crate) mod schemas;
pub(crate) mod security;
pub(crate) mod utils;

#[allow(clippy::too_many_arguments)]
pub(crate) fn gen_open_api_impl(
  item_ast: &ItemFn,
  operation_attribute: OperationAttr,
  openapi_struct: &Ident,
  openapi_struct_def: &TokenStream2,
  impl_generics: &ImplGenerics,
  ty_generics: &TypeGenerics,
  where_clause: Option<&WhereClause>,
  responder_wrapper: &TokenStream2,
) -> TokenStream2 {
  let path_item_def_impl = if operation_attribute.skip {
    quote!(
      fn is_visible() -> bool {
        false
      }
    )
  } else {
    let args = extract_fn_arguments_types(item_ast, &operation_attribute.skip_args);

    let deprecated = item_ast.attrs.iter().find_map(|attr| {
      if !matches!(attr.path().get_ident(), Some(ident) if &*ident.to_string() == "deprecated") {
        None
      } else {
        Some(true)
      }
    });

    let doc_comments: Vec<String> = item_ast
      .attrs
      .iter()
      .filter(|attr| match attr.path().get_ident() {
        None => false,
        Some(attr) => attr == "doc",
      })
      .filter_map(|attr| match &attr.meta {
        Meta::NameValue(nv) => {
          if let Expr::Lit(ref doc_comment) = nv.value {
            if let Lit::Str(ref comment) = doc_comment.lit {
              Some(comment.value().trim().to_string())
            } else {
              None
            }
          } else {
            None
          }
        }
        Meta::Path(_) | Meta::List(_) => None,
      })
      .collect();
    let description = &*doc_comments
      .clone()
      .into_iter()
      .skip(1)
      .collect::<Vec<String>>()
      .join("\\\n");

    let operation = Operation {
      args: &args,
      responder_wrapper,
      operation_id: operation_attribute.operation_id.as_ref(),
      deprecated: Some(operation_attribute.deprecated || deprecated.unwrap_or_default()),
      summary: operation_attribute.summary.as_ref().or_else(|| doc_comments.first()),
      description: operation_attribute.description.as_deref().or({
        if description.is_empty() {
          None
        } else {
          Some(description)
        }
      }),
      tags: &operation_attribute.tags,
      scopes: operation_attribute.scopes,
      error_codes: &operation_attribute.error_codes,
      consumes: operation_attribute.consumes.as_ref(),
      produces: operation_attribute.produces.as_ref(),
    };
    let components = Components {
      args: &args,
      responder_wrapper,
      error_codes: &operation_attribute.error_codes,
    };

    quote!(
      fn is_visible() -> bool {
        true
      }
      #operation
      #components
    )
  };

  quote! {
    #[allow(non_camel_case_types)]
    #[doc(hidden)]
    #openapi_struct_def
    #[automatically_derived]
    impl #impl_generics apistos::PathItemDefinition for #openapi_struct #ty_generics #where_clause {
      #path_item_def_impl
    }
  }
}

pub(crate) fn gen_item_ast(
  default_span: Span,
  mut item_ast: ItemFn,
  openapi_struct: &Ident,
  ty_generics: &TypeGenerics,
  generics_call: &TokenStream2,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
  // Remove async prefix if any. This macro generates an impl Future
  if item_ast.sig.asyncness.is_some() {
    item_ast.sig.asyncness = None;
  } else {
    emit_error!(default_span, "Operation must be an async function.");
    return (quote!(), quote!());
  }

  let mut is_impl_trait = false;
  let mut is_responder = false;
  let mut responder_wrapper =
    quote!(apistos::actix::ResponseWrapper<actix_web::HttpResponse, #openapi_struct #ty_generics>);
  match &mut item_ast.sig.output {
    ReturnType::Default => {}
    ReturnType::Type(_, _type) => {
      if let Type::ImplTrait(_) = (**_type).clone() {
        let string_type = quote!(#_type).to_string();
        is_impl_trait = true;

        if string_type == "impl Responder" {
          is_responder = true;

          *_type = Box::new(
            match syn::parse2(quote!(
              impl std::future::Future<Output=apistos::actix::ResponderWrapper<#_type>>
            )) {
              Ok(parsed) => parsed,
              Err(e) => abort!("parsing impl trait: {:?}", e),
            },
          );
        }
      } else {
        // Any handler that's not returning an impl trait should return an `impl Future`
        *_type = Box::new(match syn::parse2(quote!(impl std::future::Future<Output=#_type>)) {
          Ok(parsed) => parsed,
          Err(e) => abort!("parsing impl trait: {:?}", e),
        });
      }

      // should be an impl trait here for sure because if it was not initially
      if let Type::ImplTrait(imp) = &**_type {
        let obj = TypeTraitObject {
          dyn_token: Some(Token![dyn](default_span)),
          bounds: imp.bounds.clone(),
        };
        *_type = Box::new(match syn::parse2(quote!(#_type + apistos::PathItemDefinition)) {
          Ok(parsed) => parsed,
          Err(e) => abort!("parsing impl trait: {:?}", e),
        });

        if !is_responder {
          responder_wrapper =
            quote!(apistos::actix::ResponseWrapper<Box<#obj + std::marker::Unpin>, #openapi_struct #ty_generics>);
        }
      }
    }
  }

  let block = item_ast.block;
  let inner_handler = if is_responder {
    quote!({
      use apistos::FutureExt;
      #[expect(async_yields_async)]
      (move || { async move #block })().map(apistos::actix::ResponderWrapper)
    })
  } else if is_impl_trait {
    quote!((move || #block)())
  } else {
    quote!((move || async move #block)())
  };

  item_ast.block = Box::new(
    match syn::parse2(quote!(
        {
            let inner = #inner_handler;
            apistos::actix::ResponseWrapper {
                inner,
                path_item: #openapi_struct #generics_call,
            }
        }
    )) {
      Ok(parsed) => parsed,
      Err(e) => abort!("parsing wrapped block: {:?}", e),
    },
  );

  let responder_wrapper = if is_responder {
    quote! { apistos::actix::ResponderWrapper::<actix_web::HttpResponse> }
  } else {
    quote! { #responder_wrapper }
  };
  (responder_wrapper, quote!(#item_ast))
}

fn extract_fn_arguments_types(item_ast: &ItemFn, skipped_args: &[Ident]) -> Vec<Type> {
  item_ast
    .sig
    .inputs
    .iter()
    .filter_map(|inp| match inp {
      FnArg::Receiver(_) => None,
      FnArg::Typed(t) => match *t.pat.clone() {
        Pat::Ident(pi) => {
          if skipped_args.contains(&pi.ident) {
            None
          } else {
            Some(*t.ty.clone())
          }
        }
        _ => Some(*t.ty.clone()),
      },
    })
    .collect()
}
