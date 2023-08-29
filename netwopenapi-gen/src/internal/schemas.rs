use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Type;

pub struct Schemas<'a> {
  pub childs: &'a [Type],
}

impl<'a> ToTokens for Schemas<'a> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let childs = self.childs;
    tokens.extend(quote! {
      fn child_schemas() -> Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)> {
        let mut schemas: Vec<Option<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)>> = vec![];
        #(
          schemas.push(<#childs as ApiComponent>::schema());
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
    });
  }
}
