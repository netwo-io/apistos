use crate::operation_attr::OperationCallbacks;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Type;

pub(crate) struct Components<'a> {
  pub(crate) args: &'a [Type],
  pub(crate) responder_wrapper: &'a TokenStream,
  pub(crate) error_codes: &'a [u16],
  pub(crate) callbacks: &'a [OperationCallbacks],
}

impl<'a> ToTokens for Components<'a> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let args = self.args;
    let responder_wrapper = self.responder_wrapper;
    let callback_operations_types: Vec<Ident> = self
      .callbacks
      .iter()
      .flat_map(|c| {
        c.callbacks
          .iter()
          .flat_map(|(_, ops)| ops.values().cloned().collect::<Vec<Ident>>())
          .collect::<Vec<_>>()
      })
      .collect();
    let error_codes_filter = if self.error_codes.is_empty() {
      quote!()
    } else {
      let error_codes = self.error_codes;
      quote! {
        let available_error_codes = [#(#error_codes,)*];
        for (status, s) in error_schemas {
          let status = status.parse::<u16>();
          if let Ok(status) = status {
            if status >= 400 &&available_error_codes.contains(&status) {
              schemas.push(s);
            }
          }
        }
      }
    };
    tokens.extend(quote!(
      fn components(oas_version: apistos::OpenApiVersion) -> Vec<apistos::components::Components> {
        use apistos::ApiComponent;
        let mut component_builder = apistos::components::Components::default();

        #(
          for (name, security) in <#args>::securities() {
            component_builder.security_schemes.insert(
              name, apistos::reference_or::ReferenceOr::Object(security)
            );
          }
        )*

        let mut schemas = vec![];
        let mut schemas = if oas_version == apistos::OpenApiVersion::OAS3_0 {
          #(
            schemas.push(<#args>::schema(oas_version));
          )*
          schemas.push(<#responder_wrapper>::schema(oas_version));
          schemas.into_iter().flatten().collect::<Vec<(String, apistos::reference_or::ReferenceOr<apistos::ApistosSchema>)>>()
        } else {
          vec![]
        };
        #(
          schemas.append(&mut <#args>::child_schemas(oas_version));
        )*

        schemas.append(&mut <#responder_wrapper>::child_schemas(oas_version));
        let error_schemas = <#responder_wrapper>::error_schemas(oas_version);
        #error_codes_filter

        let mut schemas = std::collections::BTreeMap::from_iter(schemas);
        #(
          schemas.append(&mut <#callback_operations_types>::components(oas_version).into_iter().map(|c| c.schemas).flatten().collect());
        )*
        component_builder.schemas = schemas;

        vec![component_builder]
      }
    ))
  }
}
