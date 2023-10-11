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
          let mut schema: netwopenapi::RootSchema = gen.into_root_schema_for::<Self>();
          if let Some(one_of) = schema.schema.subschemas.as_mut().and_then(|s| s.one_of.as_mut()) {
            one_of.iter_mut().for_each(|s| {
              match s {
                schemars::schema::Schema::Bool(_) => {}
                schemars::schema::Schema::Object(sch_obj) => {
                  if let Some(obj) = sch_obj.object.as_mut() {
                    if obj.properties.len() == 1 {
                      obj
                        .properties
                        .first_key_value()
                        .map(|(prop_name, _)| {
                          match sch_obj.metadata.as_mut() {
                            None => {
                              sch_obj.metadata = Some(Box::new(schemars::schema::Metadata {
                                title: Some(prop_name.clone()),
                                ..Default::default()
                              }));
                            }
                            Some(m) => m.title = m.title.clone().or_else(|| Some(prop_name.clone())),
                          };
                        });
                    };
                  };
                }
              }
            })
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
