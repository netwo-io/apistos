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

    let update_metadata_title = quote!(match sch_obj.metadata.as_mut() {
      None => {
        sch_obj.metadata = Some(Box::new(schemars::schema::Metadata {
          title: Some(prop_name.clone()),
          ..Default::default()
        }));
      }
      Some(m) => m.title = m.title.clone().or_else(|| Some(prop_name.clone())),
    });
    let update_one_of_title = quote!(for s in &mut *one_of {
      match s {
        schemars::schema::Schema::Bool(_) => {}
        schemars::schema::Schema::Object(sch_obj) => {
          if let Some(obj) = sch_obj.object.as_mut() {
            if obj.properties.len() == 1 {
              if let Some((prop_name, _)) = obj.properties.first_key_value() {
                #update_metadata_title;
              }
            } else if let Some(enum_values) = obj.properties.iter_mut().find_map(|(_, p)| match p {
              schemars::schema::Schema::Bool(_) => None,
              schemars::schema::Schema::Object(sch_obj) => sch_obj.enum_values.as_mut(),
            }) {
              if enum_values.len() == 1 {
                if let Some(serde_json::Value::String(prop_name)) = enum_values.first() {
                  #update_metadata_title
                }
              }
            }
          };
        }
      }
    });

    tokens.extend(quote! {
      fn child_schemas() -> Vec<(String, netwopenapi::reference_or::ReferenceOr<netwopenapi::Schema>)> {
        let settings = schemars::gen::SchemaSettings::openapi3();
        let mut gen = settings.into_generator();
        let schema: netwopenapi::RootSchema = gen.into_root_schema_for::<Self>();

        let mut schemas: Vec<(String, netwopenapi::reference_or::ReferenceOr<netwopenapi::Schema>)> = vec![];
        for (def_name, mut def) in schema.definitions {
          match &mut def {
            schemars::schema::Schema::Bool(_) => {}
            schemars::schema::Schema::Object(schema) => {
              if let Some(one_of) = schema.subschemas.as_mut().and_then(|s| s.one_of.as_mut()) {
                #update_one_of_title;
              }
            }
          }
          schemas.push((def_name, netwopenapi::reference_or::ReferenceOr::Object(def)));
        }
        schemas
      }

      fn schema() -> Option<(String, netwopenapi::reference_or::ReferenceOr<netwopenapi::Schema>)> {
        let (name, schema) = {
          let schema_name = <Self as schemars::JsonSchema>::schema_name();
          let settings = schemars::gen::SchemaSettings::openapi3();
          let mut gen = settings.into_generator();
          let mut schema: netwopenapi::RootSchema = gen.into_root_schema_for::<Self>();
          if let Some(one_of) = schema.schema.subschemas.as_mut().and_then(|s| s.one_of.as_mut()) {
            #update_one_of_title
          }
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
