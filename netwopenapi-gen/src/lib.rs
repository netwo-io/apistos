use crate::internal::{extract_generics_params, gen_item_ast, gen_open_api_impl};
use crate::operation::OperationAttr;
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{Data, DeriveInput, Ident, ItemFn, Type};

mod internal;
mod operation;
mod path;

const OPENAPI_STRUCT_PREFIX: &str = "__openapi_";

#[proc_macro_error]
#[proc_macro_derive(ApiComponent)]
pub fn derive_api_component(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as DeriveInput);
  let DeriveInput {
    attrs: _attrs,
    ident,
    data,
    generics,
    vis: _vis,
  } = input;

  let childs: Vec<Type> = match data {
    Data::Struct(s) => s.fields.into_iter().map(|f| f.ty).collect(),
    Data::Enum(e) => e
      .variants
      .into_iter()
      .map(|v| v.fields.into_iter().map(|f| f.ty))
      .flatten()
      .collect(),
    Data::Union(u) => u.fields.named.into_iter().map(|f| f.ty).collect(),
  };

  let (_, ty_generics, where_clause) = generics.split_for_impl();
  quote!(
    impl #generics netwopenapi::ApiComponent for #ident #ty_generics #where_clause {
      fn child_schemas() -> Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)> {
        let mut schemas: Vec<Option<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)>> = vec![];
        #(
          schemas.push(<#childs>::schema());
        )*
        let mut schemas = schemas.into_iter().flatten().collect::<Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)>>();
        #(
          schemas.append(&mut <#childs>::child_schemas());
        )*
        schemas
      }

      fn schema() -> Option<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)> {
        let (name, schema) = <Self as utoipa::ToSchema<'_>>::schema();
        Some((name.to_string(), schema))
      }
    }
  ).into()
}

/// Todo: doc
#[proc_macro_error]
#[proc_macro_attribute]
pub fn api_operation(attr: TokenStream, item: TokenStream) -> TokenStream {
  let operation_attribute = syn::parse_macro_input!(attr as OperationAttr);

  let default_span = proc_macro2::Span::call_site();
  let item_ast = match syn::parse::<ItemFn>(item) {
    Ok(v) => v,
    Err(e) => abort!(e.span(), format!("{e}")),
  };

  let s_name = format!("{OPENAPI_STRUCT_PREFIX}{}", item_ast.sig.ident);
  let openapi_struct = Ident::new(&s_name, default_span);

  let generics = &item_ast.sig.generics.clone();
  let mut generics_call = quote!();
  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
  let openapi_struct_def = if !generics.params.is_empty() {
    let turbofish = ty_generics.as_turbofish();
    generics_call = quote!(#turbofish { p: std::marker::PhantomData });
    let generics_params = extract_generics_params(&item_ast);
    quote!(struct #openapi_struct #ty_generics { p: std::marker::PhantomData<(#generics_params)> } )
  } else {
    quote!(struct #openapi_struct;)
  };

  let (responder_wrapper, generated_item_ast) =
    gen_item_ast(default_span, item_ast, &openapi_struct, &ty_generics, generics_call);
  let generated_item_fn = match syn::parse::<ItemFn>(generated_item_ast.clone().into()) {
    Ok(v) => v,
    Err(e) => abort!(e.span(), format!("{e}")),
  };
  let open_api_def = gen_open_api_impl(
    &generated_item_fn,
    operation_attribute,
    &openapi_struct,
    openapi_struct_def,
    impl_generics,
    &ty_generics,
    where_clause,
    responder_wrapper,
  );

  let res = quote!(
    #open_api_def

    #generated_item_ast
  );

  // eprintln!("{:#}", res.to_string());
  res.into()
}
