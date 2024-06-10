use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub(crate) struct Schemas {
  pub(crate) deprecated: bool,
}

impl ToTokens for Schemas {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let deprecated = if self.deprecated {
      quote!(
        obj.insert("deprecated".to_owned(), true.into());
      )
    } else {
      quote!()
    };

    let update_metadata_title = quote!(
      sch_obj.entry("title").or_insert_with(|| prop_name.clone().into());
    );
    let update_single_enum_value = quote!(if enum_values.len() == 1 {
      if let Some(schemars::_serde_json::Value::String(prop_name)) = enum_values.first() {
        #update_metadata_title
      }
    });
    let update_one_of_title = quote!(for s in one_of {
      if let Some(sch_obj) = s.as_object_mut() {
        if let Some(props) = sch_obj.clone().get("properties").and_then(|v| v.as_object()) {
          if props.len() == 1 {
            if let Some((prop_name, _)) = props.iter().next() {
              #update_metadata_title;
            }
          } else if let Some(enum_values) = props.iter().find_map(|(_, p)| p.as_object().and_then(|sch_obj| sch_obj.get("enum").and_then(|v| v.as_array()))) {
            #update_single_enum_value
          }
        } else if let Some(enum_values) = sch_obj.clone().get_mut("enum").and_then(|v| v.as_array_mut()) {
          #update_single_enum_value
        }
      }
    });

    tokens.extend(quote! {
      fn child_schemas(oas_version: apistos::OpenApiVersion) -> Vec<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)> {
        let settings = match oas_version {
          apistos::OpenApiVersion::OAS3_0 => schemars::gen::SchemaSettings::openapi3(),
          apistos::OpenApiVersion::OAS3_1 => schemars::gen::SchemaSettings::draft2020_12(),
        };
        let mut gen = settings.into_generator();
        let mut schema: apistos::Schema = gen.into_root_schema_for::<Self>();

        let mut schemas: Vec<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)> = vec![];
        let obj = schema.ensure_object();
        for (def_name, mut def) in obj
          .get("components")
          .and_then(|v| v.as_object())
          .and_then(|v| v.get("schemas"))
          .and_then(|v| v.as_object())
          .cloned()
          .unwrap_or_default()
        {
          if let Some(schema) = def.as_object_mut() {
            if let Some(one_of) = schema.get_mut("oneOf").and_then(|v| v.as_array_mut()) {
              #update_one_of_title;
            }
          }
          let schema = apistos::Schema::try_from(def)
                    .map_err(|err| {
                      apistos::log::warn!("Error generating json schema: {err:?}");
                      err
                    }).unwrap_or_default();
          schemas.push((def_name, apistos::reference_or::ReferenceOr::Object(schema)));
        }
        schemas
      }

      fn schema(oas_version: apistos::OpenApiVersion) -> Option<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)> {
        let (name, schema) = {
          let schema_name = <Self as schemars::JsonSchema>::schema_name();
          let settings = match oas_version {
            apistos::OpenApiVersion::OAS3_0 => schemars::gen::SchemaSettings::openapi3(),
            apistos::OpenApiVersion::OAS3_1 => schemars::gen::SchemaSettings::draft2020_12(),
          };
          let mut gen = settings.into_generator();
          let definition_path = gen.settings().definitions_path.clone();
          let definition_path = definition_path
            .trim_start_matches('/')
            .split('/')
            .next()
            .unwrap_or_default();
          let mut schema: apistos::Schema = gen.into_root_schema_for::<Self>();

          let obj = schema.ensure_object();
          if let Some(mut one_of) = obj.get_mut("oneOf").and_then(|v| v.as_array_mut()) {
            #update_one_of_title;
          }
          obj.remove(definition_path);
          #deprecated
          let schema = apistos::Schema::from(obj.clone());
          (
            schema_name,
            apistos::reference_or::ReferenceOr::Object(schema)
          )
        };
        Some((name.to_string(), schema))
      }
    });
  }
}
