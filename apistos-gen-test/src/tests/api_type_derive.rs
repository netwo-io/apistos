use assert_json_diff::assert_json_eq;
use schemars::schema::InstanceType;
use serde_json::json;

use crate::utils::assert_schema;
use apistos_core::{ApiComponent, TypedSchema};
use apistos_gen::ApiType;

#[test]
#[allow(dead_code)]
fn api_type_derive() {
  #[derive(ApiType)]
  struct Name(String);

  impl TypedSchema for Name {
    fn schema_type() -> InstanceType {
      InstanceType::String
    }

    fn format() -> Option<String> {
      None
    }
  }

  let name_schema = <Name as ApiComponent>::schema();
  let name_child_schemas = <Name as ApiComponent>::child_schemas();
  assert!(name_schema.is_some());
  assert!(name_child_schemas.is_empty());
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Name");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "type": "string"
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_type_derive_with_format() {
  #[derive(ApiType)]
  struct Name(String);

  impl TypedSchema for Name {
    fn schema_type() -> InstanceType {
      InstanceType::String
    }

    fn format() -> Option<String> {
      // not a real format but not a problem
      Some("lastname".to_string())
    }
  }

  let name_schema = <Name as ApiComponent>::schema();
  let name_child_schemas = <Name as ApiComponent>::child_schemas();
  assert!(name_schema.is_some());
  assert!(name_child_schemas.is_empty());
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Name");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "type": "string",
      "format": "lastname"
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_type_derive_with_format_complex_struct() {
  #[derive(ApiType)]
  struct Name {
    last_name: String,
    first_name: String,
    id: u32,
  }

  impl TypedSchema for Name {
    fn schema_type() -> InstanceType {
      InstanceType::String
    }

    fn format() -> Option<String> {
      None
    }
  }

  let name_schema = <Name as ApiComponent>::schema();
  let name_child_schemas = <Name as ApiComponent>::child_schemas();
  assert!(name_schema.is_some());
  assert!(name_child_schemas.is_empty());
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Name");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "type": "string"
    })
  );
}
