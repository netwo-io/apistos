use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Type;

pub(crate) struct Components<'a> {
  pub(crate) args: &'a [Type],
  pub(crate) responder_wrapper: &'a TokenStream,
  pub(crate) error_codes: &'a [u16],
}

impl ToTokens for Components<'_> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let args = self.args;
    let responder_wrapper = self.responder_wrapper;
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
      fn components() -> Vec<apistos::components::Components> {
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
        #(
          schemas.push(<#args>::schema());
        )*
        schemas.push(<#responder_wrapper>::schema());
        let mut schemas = schemas.into_iter().flatten().collect::<Vec<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)>>();
        #(
          schemas.append(&mut <#args>::child_schemas());
        )*
        schemas.append(&mut <#responder_wrapper>::child_schemas());
        let error_schemas = <#responder_wrapper>::error_schemas();
        #error_codes_filter
        component_builder.schemas = std::collections::BTreeMap::from_iter(schemas);
        vec![component_builder]
      }
    ))
  }
}
