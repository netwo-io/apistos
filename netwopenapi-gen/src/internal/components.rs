use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Type;

pub struct Components<'a> {
  pub args: &'a [Type],
  pub responder_wrapper: &'a TokenStream,
  pub error_codes: &'a [u16],
}

impl<'a> ToTokens for Components<'a> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let args = self.args;
    let responder_wrapper = self.responder_wrapper;
    let error_codes_filter = if self.error_codes.is_empty() {
      quote!()
    } else {
      let error_codes = self.error_codes;
      quote! {
        let available_error_codes = vec![#(#error_codes)*,];
        error_schemas
          .into_iter()
          .map(|(status, s)| {
            use std::str::FromStr;
            let status = status.parse::<u16>();
            if let Ok(status) = status {
              if status >= 400 &&available_error_codes.contains(&status) {
                schemas.push(s);
              }
            }
          });
      }
    };
    tokens.extend(quote!(
      fn components() -> Vec<utoipa::openapi::Components> {
        use netwopenapi::ApiComponent;
        let mut component_builder = utoipa::openapi::ComponentsBuilder::new();

        #(
          for (name, security) in <#args>::securities() {
            component_builder = component_builder.security_scheme(name, security);
          }
        )*

        let mut schemas = vec![];
        #(
          schemas.push(<#args>::schema());
        )*
        schemas.push(<#responder_wrapper>::schema());
        let mut schemas = schemas.into_iter().flatten().collect::<Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)>>();
        #(
          schemas.append(&mut <#args>::child_schemas());
        )*
        schemas.append(&mut <#responder_wrapper>::child_schemas());
        let error_schemas = <#responder_wrapper>::error_schemas();
        #error_codes_filter
        for (name, schema) in schemas {
          component_builder = component_builder.schema(name, schema);
        }
        vec![component_builder.build()]
      }
    ))
  }
}
