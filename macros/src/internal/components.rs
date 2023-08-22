use proc_macro2::TokenStream;
use proc_macro_error::__export::ToTokensAsSpanRange;
use quote::{quote, ToTokens};
use syn::Type;

pub struct Components<'a> {
  pub args: &'a [Type],
}

impl<'a> ToTokens for Components<'a> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let args = self.args;
    tokens.extend(quote!(
      fn components() -> Vec<utoipa::openapi::Components> {
        use netwopenapi::ApiComponent;
        let mut component_builder = utoipa::openapi::ComponentsBuilder::new();

        let mut schemas = vec![];
        #(
          schemas.push(<#args>::schema());
        )*
        //@todo for each property of each modifiers also fetch schema()
        let mut schemas = schemas.into_iter().flatten().collect::<Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)>>();
        #(
          schemas.append(&mut <#args>::child_schemas());
        )*
        for (name, schema) in schemas {
          component_builder = component_builder.schema(name, schema);
        }
        vec![component_builder.build()]
      }
    ))
  }
}
