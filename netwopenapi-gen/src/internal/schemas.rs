use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub struct Schemas;

impl ToTokens for Schemas {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    tokens.extend(quote! {
      fn child_schemas() -> Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)> {
        let settings = schemars::gen::SchemaSettings::openapi3();
        let mut gen = settings.into_generator();
        let schema: schemars::schema::RootSchema = gen.into_root_schema_for::<Self>();

        let mut schemas: Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)> = vec![];
        for (def_name, def) in schema.definitions {
          schemas.push((def_name, netwopenapi::json_schema_to_schemas(def.into_object())));
        }
        schemas
      }

      fn schema() -> Option<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)> {
        let (name, schema) = {
          let schema_name = <Self as schemars::JsonSchema>::schema_name();
          let settings = schemars::gen::SchemaSettings::openapi3();
          let mut gen = settings.into_generator();
          let schema = <Self as schemars::JsonSchema>::json_schema(&mut gen);
          (schema_name, netwopenapi::json_schema_to_schemas(schema.into_object()))
        };
        Some((name.to_string(), schema))
      }
    });
  }
}
