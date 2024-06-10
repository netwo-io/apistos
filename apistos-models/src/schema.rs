use crate::OpenApiVersion;
use log::warn;
use schemars::gen::SchemaSettings;
use schemars::{Schema, SchemaGenerator};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(transparent)]
pub struct ApistosSchema(Schema);

impl Default for ApistosSchema {
  fn default() -> Self {
    Self(Schema::default())
  }
}

impl ApistosSchema {
  pub fn new(mut schema: Schema, oas_version: OpenApiVersion) -> Self {
    let obj = schema.as_object_mut();

    match obj {
      None => Self(schema),
      Some(obj) => {
        // set title for each one_of if not set
        if let Some(one_of) = obj.get_mut("oneOf").and_then(|v| v.as_array_mut()) {
          Self::set_title_for_enum_variants(one_of);
        }

        // remove definitions from schema
        Self::remove_definition_from_schema(obj, &oas_version.get_schema_settings().into_generator());
        match oas_version {
          OpenApiVersion::OAS3_0 => {
            // remove definitions from schema
            Self::remove_definition_from_schema(obj, &SchemaSettings::openapi3().into_generator());
            // remove $schema property
            obj.remove("$schema");
            Self(schemars::Schema::from(obj.clone()))
          }
          OpenApiVersion::OAS3_1 => Self(schemars::Schema::from(obj.clone())),
        }
      }
    }
  }

  pub fn from_value(value: &Value, oas_version: OpenApiVersion) -> Self {
    let schema = schemars::Schema::try_from(value.clone());
    match schema {
      Ok(sch) => Self::new(sch, oas_version),
      Err(e) => {
        warn!("Error converting value to schema: get {e:?} for value {value:?}");
        Self::new(Schema::default(), oas_version)
      }
    }
  }

  pub fn into_inner(self) -> Schema {
    self.0
  }

  pub fn inner(&self) -> &Schema {
    &self.0
  }

  pub fn inner_mut(&mut self) -> &mut Schema {
    &mut self.0
  }

  fn remove_definition_from_schema(obj: &mut serde_json::Map<String, Value>, gen: &SchemaGenerator) {
    let definition_path = gen.settings().definitions_path.clone();
    let definition_path = definition_path
      .trim_start_matches('/')
      .split('/')
      .next()
      .unwrap_or_default();
    obj.remove(definition_path);
  }

  fn set_title_for_enum_variants(one_of: &mut Vec<Value>) {
    for s in one_of {
      if let Some(sch_obj) = s.as_object_mut() {
        if let Some(props) = sch_obj.clone().get("properties").and_then(|v| v.as_object()) {
          if props.len() == 1 {
            if let Some((prop_name, _)) = props.iter().next() {
              sch_obj.entry("title").or_insert_with(|| prop_name.clone().into());
            }
          } else if let Some(enum_values) = props.iter().find_map(|(_, p)| {
            p.as_object()
              .and_then(|sch_obj| sch_obj.get("enum").and_then(|v| v.as_array()))
          }) {
            if enum_values.len() == 1 {
              if let Some(Value::String(prop_name)) = enum_values.first() {
                sch_obj.entry("title").or_insert_with(|| prop_name.clone().into());
              }
            }
          }
        } else if let Some(enum_values) = sch_obj.clone().get_mut("enum").and_then(|v| v.as_array_mut()) {
          if enum_values.len() == 1 {
            if let Some(Value::String(prop_name)) = enum_values.first() {
              sch_obj.entry("title").or_insert_with(|| prop_name.clone().into());
            }
          }
        }
      }
    }
  }
}

#[cfg(any(feature = "test", test))]
impl From<Schema> for ApistosSchema {
  fn from(value: Schema) -> Self {
    Self(value)
  }
}

#[cfg(any(feature = "test", test))]
impl From<Schema> for crate::reference_or::ReferenceOr<ApistosSchema> {
  fn from(value: Schema) -> Self {
    Self::Object(value.into())
  }
}
