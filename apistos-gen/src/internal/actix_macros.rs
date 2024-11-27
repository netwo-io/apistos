#[cfg(feature = "actix-web-macros")]
use crate::actix_operation_attr::ActixOperationAttr;
use crate::internal::gen_item_ast;
use crate::internal::path_item::{PathItem, SourceDefinitionKind};
use crate::operation_attr::{OperationAttr, OperationType};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_error2::abort;
use quote::{format_ident, quote};
use syn::{GenericParam, ItemFn};

pub(crate) fn gen_open_api_def_actix_routes_macro(
  operations: Vec<(OperationType, ActixOperationAttr)>,
  item_ast: &ItemFn,
) -> proc_macro2::TokenStream {
  let mut res = quote!();

  let item_ident = &item_ast.sig.ident;

  let generics_call = quote!();

  let mut openapi_structs = vec![];
  let mut operation_defs = quote!();
  for (operation_type, operation_attr) in operations {
    if matches!(operation_type, OperationType::Custom(_) | OperationType::Connect) {
      continue;
    }

    let operation_type_str = operation_type.to_string();
    let path = operation_attr.path;
    let normalized_path = path.replace("/", "_").replace("{", "").replace("}", "");
    let openapi_struct = format_ident!("{item_ident}{operation_type_str}{normalized_path}");

    let (responder_wrapper, _) = gen_item_ast(
      Span::call_site(),
      item_ast.clone(),
      &openapi_struct,
      None,
      &generics_call,
      true,
    );

    let open_api_def = PathItem {
      source: SourceDefinitionKind::ActixMacros,
      openapi_struct: &openapi_struct,
      item_ast,
      operation_attribute: operation_attr.operation,
      responder_wrapper: &responder_wrapper,
    };

    res.extend(quote! {
      #open_api_def
    });

    operation_defs.extend(quote!(
      (#path.to_string(), apistos::IndexMap::from_iter(vec![(#operation_type, #openapi_struct::operation(oas_version))])),
    ));

    openapi_structs.push(openapi_struct);
  }

  let definition_holder_impl = quote!(
    impl ::apistos::DefinitionHolder for #item_ident {
      fn operations(&mut self, oas_version: apistos::OpenApiVersion) -> apistos::IndexMap<String, apistos::IndexMap<apistos::paths::OperationType, apistos::paths::Operation>> {
        use ::apistos::PathItemDefinition;
        apistos::IndexMap::from_iter(vec![#operation_defs])
      }
      fn components(&mut self, oas_version: apistos::OpenApiVersion) -> Vec<apistos::components::Components> {
        use ::apistos::PathItemDefinition;
        let components: Vec<Vec<apistos::components::Components>> = vec![#(<#openapi_structs as ::apistos::PathItemDefinition>::components(oas_version),)*];
        components.into_iter().flatten().collect()
      }
    }
  );

  res.extend(quote!(
    #(
      struct #openapi_structs;
    )*
    #definition_holder_impl
  ));

  res
}

pub(crate) fn gen_open_api_def_actix_macro(
  operation_attribute: OperationAttr,
  item: TokenStream,
  actix_proc_macro: &proc_macro2::TokenStream,
  path: &str,
  operation_type: &OperationType,
) -> proc_macro2::TokenStream {
  let default_span = Span::call_site();
  let item_ast = match syn::parse::<ItemFn>(item) {
    Ok(v) => v,
    Err(e) => abort!(e.span(), format!("{e}")),
  };

  let openapi_struct = &item_ast.sig.ident;

  let generics = &item_ast.sig.generics.clone();
  let mut generics_call = quote!();
  let (_impl_generics, ty_generics, _where_clause) = generics.split_for_impl();
  if !generics.params.is_empty() {
    let mut generic_types_idents = vec![];
    for param in &generics.params {
      match param {
        GenericParam::Lifetime(_) => {}
        GenericParam::Const(_) => {}
        GenericParam::Type(_type) => generic_types_idents.push(_type.ident.clone()),
      }
    }
    let turbofish = ty_generics.as_turbofish();
    let mut phantom_params_names = quote!();
    for generic_types_ident in generic_types_idents {
      let param_name = Ident::new(
        &format_ident!("p_{}", generic_types_ident).to_string().to_lowercase(),
        Span::call_site(),
      );
      phantom_params_names.extend(quote!(#param_name: std::marker::PhantomData,));
    }
    generics_call = quote!(#turbofish { #phantom_params_names });
  };

  let (responder_wrapper, _) = gen_item_ast(
    default_span,
    item_ast.clone(),
    openapi_struct,
    None,
    &generics_call,
    true,
  );

  let open_api_def = PathItem {
    source: SourceDefinitionKind::ActixMacros,
    openapi_struct,
    item_ast: &item_ast,
    operation_attribute,
    responder_wrapper: &responder_wrapper,
  };

  let definition_holder_impl = quote!(
    #[automatically_derived]
    impl ::apistos::DefinitionHolder for #openapi_struct {
      fn operations(&mut self, oas_version: apistos::OpenApiVersion) -> apistos::IndexMap<String, apistos::IndexMap<apistos::paths::OperationType, apistos::paths::Operation>> {
        use ::apistos::PathItemDefinition;
        apistos::IndexMap::from_iter(
          vec![(
            #path.to_string(),
            apistos::IndexMap::from_iter(vec![(#operation_type, #openapi_struct::operation(oas_version))])
          )]
        )
      }
      fn components(&mut self, oas_version: apistos::OpenApiVersion) -> Vec<apistos::components::Components> {
        use ::apistos::PathItemDefinition;
        <#openapi_struct as ::apistos::PathItemDefinition>::components(oas_version)
      }
    }
  );

  quote!(
    #open_api_def

    #definition_holder_impl
    #actix_proc_macro
    #item_ast
  )
}
