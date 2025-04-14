use proc_macro_error2::{abort, emit_error};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

use syn::{FnArg, Ident, ItemFn, Pat, ReturnType, Token, Type, TypeGenerics, TypeTraitObject};

mod components;
mod operation;

#[cfg(feature = "actix-web-macros")]
pub(crate) mod actix_macros;
pub(crate) mod path_item;
pub(crate) mod schemas;
pub(crate) mod security;
pub(crate) mod utils;

pub(crate) fn gen_item_ast(
  default_span: Span,
  mut item_ast: ItemFn,
  openapi_struct: &Ident,
  ty_generics: Option<&TypeGenerics>,
  generics_call: &TokenStream2,
  with_actix_macros: bool,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
  // Remove async prefix if any. This macro generates an impl Future
  if item_ast.sig.asyncness.is_some() {
    item_ast.sig.asyncness = None;
  } else if !with_actix_macros {
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

  let openapi_struct = if with_actix_macros {
    quote! {Self}
  } else {
    quote! {#openapi_struct}
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
