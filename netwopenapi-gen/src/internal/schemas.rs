use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub(crate) struct Schemas {
  pub(crate) deprecated: bool,
}

impl ToTokens for Schemas {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let deprecated = if self.deprecated {
      quote!(
        let schema = {
          let mut schema = schema;
          schema.schema.metadata.as_mut().map(|mut m| m.deprecated = true);
          schema
        };
      )
    } else {
      quote!()
    };
    tokens.extend(quote! {
      fn child_schemas() -> Vec<(String, netwopenapi::reference_or::ReferenceOr<netwopenapi::Schema>)> {
        let settings = schemars::gen::SchemaSettings::openapi3();
        let mut gen = settings.into_generator();
        let schema: netwopenapi::RootSchema = gen.into_root_schema_for::<Self>();

        let mut schemas: Vec<(String, netwopenapi::reference_or::ReferenceOr<netwopenapi::Schema>)> = vec![];
        for (def_name, def) in schema.definitions {
          schemas.push((def_name, netwopenapi::reference_or::ReferenceOr::Object(def)));
        }
        schemas
      }

      fn schema() -> Option<(String, netwopenapi::reference_or::ReferenceOr<netwopenapi::Schema>)> {
        let (name, schema) = {
          let schema_name = <Self as schemars::JsonSchema>::schema_name();
          let settings = schemars::gen::SchemaSettings::openapi3();
          let mut gen = settings.into_generator();
          let schema: netwopenapi::RootSchema = gen.into_root_schema_for::<Self>();
          #deprecated
          (
            schema_name,
            netwopenapi::reference_or::ReferenceOr::Object(schemars::schema::Schema::Object(schema.schema))
          )
        };
        Some((name, schema))
      }
    });
  }
}
