use crate::internal::components::Components;
use crate::internal::operation::Operation;
use crate::operation::OperationAttr;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_error::{abort, emit_error, proc_macro_error};
use quote::quote;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
  FnArg, GenericParam, Ident, ImplGenerics, ItemFn, ReturnType, Token, Type, TypeGenerics, TypeTraitObject, WhereClause,
};

mod components;
mod operation;

pub(crate) fn gen_open_api_impl(
  item_ast: &ItemFn,
  operation_attribute: OperationAttr,
  openapi_struct: &Ident,
  openapi_struct_def: TokenStream2,
  impl_generics: ImplGenerics,
  ty_generics: &TypeGenerics,
  where_clause: Option<&WhereClause>,
) -> TokenStream2 {
  let path_item_def_impl = if operation_attribute.skip {
    quote!(
      fn is_visible() -> bool {
        false
      }
    )
  } else {
    let args = extract_fn_arguments_types(item_ast);
    let operation = Operation { args: &args }.to_token_stream();
    let components = Components { args: &args }.to_token_stream();

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
    impl #impl_generics netwopenapi::path_item_definition::PathItemDefinition for #openapi_struct #ty_generics #where_clause {
      #path_item_def_impl
    }
  }
}

pub(crate) fn gen_item_ast(
  default_span: Span,
  mut item_ast: ItemFn,
  openapi_struct: Ident,
  ty_generics: TypeGenerics,
  generics_call: TokenStream2,
) -> TokenStream2 {
  // Remove async prefix if any. This macro generate an impl Future
  if item_ast.sig.asyncness.is_some() {
    item_ast.sig.asyncness = None;
  } else {
    // @todo should we really fail here as the macro doesn't really care about it ?
    emit_error!(default_span, "Operation must be an async function.");
    return quote!().into();
  }

  let mut is_impl_trait = false;
  let mut is_responder = false;
  let mut responder_wrapper =
    quote!(netwopenapi::actix::ResponderWrapper<actix_web::HttpResponse, #openapi_struct #ty_generics>);
  match &mut item_ast.sig.output {
    ReturnType::Default => {}
    ReturnType::Type(_, _type) => {
      if let Type::ImplTrait(_) = (&**_type).clone() {
        let string_type = quote!(#_type).to_string();
        is_impl_trait = true;

        if string_type == "impl Responder" {
          is_responder = true;

          *_type = Box::new(
            syn::parse2(quote!(
              impl std::future::Future<Output=netwopenapi::actix::ResponderWrapper<#_type>>
            ))
            .expect("parsing impl trait"),
          );
        }
      } else {
        // Any handler that's not returning an impl trait should return an `impl Future`
        *_type = Box::new(syn::parse2(quote!(impl std::future::Future<Output=#_type>)).expect("parsing impl trait"));
      }

      // should be an impl trait here for sure because if it was not initially
      if let Type::ImplTrait(imp) = &**_type {
        let obj = TypeTraitObject {
          dyn_token: Some(Token![dyn](default_span)),
          bounds: imp.bounds.clone(),
        };
        *_type = Box::new(
          syn::parse2(quote!(#_type + netwopenapi::path_item_definition::PathItemDefinition))
            .expect("parsing impl trait"),
        );

        if !is_responder {
          responder_wrapper =
            quote!(netwopenapi::actix::ResponderWrapper<Box<#obj + std::marker::Unpin>, #openapi_struct #ty_generics>);
        }
      }
    }
  }

  let block = item_ast.block;
  let inner_handler = if is_responder {
    quote!(core::future::ready::ready(netwopenapi::actix::ResponderWrapper((move || #block)())))
  } else if is_impl_trait {
    quote!((move || #block)())
  } else {
    quote!((move || async move #block)())
  };

  item_ast.block = Box::new(
    syn::parse2(quote!(
        {
            let inner = #inner_handler;
            netwopenapi::actix::ResponderWrapper {
                inner,
                path_item: #openapi_struct #generics_call,
            }
        }
    ))
    .expect("parsing wrapped block"),
  );

  quote!(#item_ast)
}

pub(crate) fn extract_generics_params(item_ast: &ItemFn) -> Punctuated<GenericParam, Comma> {
  item_ast.sig.generics.params.clone()
}

fn extract_fn_arguments_types(item_ast: &ItemFn) -> Vec<Type> {
  item_ast
    .sig
    .inputs
    .iter()
    .filter_map(|inp| match inp {
      FnArg::Receiver(_) => None,
      FnArg::Typed(ref t) => Some(*t.ty.clone()),
    })
    .collect()
}
