use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Type;

pub struct Components<'a> {
  pub args: &'a [Type],
  pub responder_wrapper: &'a TokenStream,
}

impl<'a> ToTokens for Components<'a> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let args = self.args;
    let responder_wrapper = self.responder_wrapper;
    tokens.extend(quote!(
      fn components() -> Vec<utoipa::openapi::Components> {
        use netwopenapi::ApiComponent;
        let mut component_builder = utoipa::openapi::ComponentsBuilder::new();

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
        for (name, schema) in schemas {
          component_builder = component_builder.schema(name, schema);
        }
        vec![component_builder.build()]
      }
    ))
  }
}
