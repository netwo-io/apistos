use crate::OpenApiVersion;
use log::warn;
use schemars::gen::SchemaSettings;
use schemars::{Schema, SchemaGenerator};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Clone, Debug)]
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
        match oas_version {
          OpenApiVersion::OAS3_0 => {
            // remove definitions from schema
            Self::remove_definition_from_schema(obj, SchemaSettings::openapi3().into_generator());
            // remove $schema property
            obj.remove("$schema");
            Self(schemars::Schema::from(obj.clone()))
          }
          OpenApiVersion::OAS3_1 => {
            // remove definitions from schema
            Self::remove_definition_from_schema(obj, SchemaSettings::draft2020_12().into_generator());
            Self(schemars::Schema::from(obj.clone()))
          }
        }
      }
    }
  }

  pub fn from_value(value: Value, oas_version: OpenApiVersion) -> Self {
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

  fn remove_definition_from_schema(obj: &mut serde_json::Map<String, Value>, gen: SchemaGenerator) {
    let definition_path = gen.settings().definitions_path.clone();
    let definition_path = definition_path
      .trim_start_matches('/')
      .split('/')
      .next()
      .unwrap_or_default();
    obj.remove(definition_path);
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
