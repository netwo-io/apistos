use apistos::OpenApiVersion;
use assert_json_diff::assert_json_eq;
use serde_json::json;
use std::str::FromStr;

use crate::utils::assert_schema;
use apistos_core::{ApiComponent, TypedSchema};
use apistos_gen::ApiType;

#[test]
#[expect(dead_code)]
fn api_type_derive() {
  #[derive(ApiType)]
  struct Name(String);

  impl TypedSchema for Name {
    fn schema_type() -> String {
      "string".to_owned()
    }

    fn format() -> Option<String> {
      None
    }
  }

  #[derive(ApiType)]
  #[openapi_type(schema_type = "string")]
  struct Name2(String);

  let name_schema = <Name as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
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
    })
  );

  let name_schema = <Name2 as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name2 as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
  assert!(name_schema.is_some());
  assert!(name_child_schemas.is_empty());
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Name2");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "type": "string",
    })
  );
}

#[test]
#[expect(dead_code)]
fn api_type_derive_with_format() {
  #[derive(ApiType)]
  struct Name(String);

  impl TypedSchema for Name {
    fn schema_type() -> String {
      "string".to_owned()
    }

    fn format() -> Option<String> {
      // not a real format but not a problem
      Some("lastname".to_string())
    }
  }

  #[derive(ApiType)]
  #[openapi_type(schema_type = "string", format = "lastname")]
  struct Name2(String);

  let name_schema = <Name as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
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

  let name_schema = <Name2 as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name2 as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
  assert!(name_schema.is_some());
  assert!(name_child_schemas.is_empty());
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Name2");
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
#[expect(dead_code)]
fn api_type_derive_with_default_type_parameter() {
  struct GenericHolder<T = String> {
    inner: T,
  }

  #[derive(ApiType)]
  struct Name<T = String>(GenericHolder<T>);

  impl<T> TypedSchema for Name<T> {
    fn schema_type() -> String {
      "string".to_owned()
    }

    fn format() -> Option<String> {
      // not a real format but not a problem
      Some("lastname".to_string())
    }
  }

  #[derive(ApiType)]
  #[openapi_type(schema_type = "string", format = "lastname")]
  struct Name2<T = String>(GenericHolder<T>);

  let name_schema = <Name as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
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

  let name_schema = <Name2 as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name2 as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
  assert!(name_schema.is_some());
  assert!(name_child_schemas.is_empty());
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Name2");
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
#[expect(dead_code)]
fn api_type_derive_with_generic_type_parameter() {
  struct GenericHolder<T: FromStr> {
    inner: T,
  }

  #[derive(ApiType)]
  struct Name<T: FromStr>(GenericHolder<T>);

  impl<T: FromStr> TypedSchema for Name<T> {
    fn schema_type() -> String {
      "string".to_owned()
    }

    fn format() -> Option<String> {
      // not a real format but not a problem
      Some("lastname".to_string())
    }
  }

  #[derive(ApiType)]
  #[openapi_type(schema_type = "string", format = "lastname")]
  struct Name2<T: FromStr>(GenericHolder<T>);

  let name_schema = <Name<String> as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name<String> as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
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

  let name_schema = <Name2<String> as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name2<String> as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
  assert!(name_schema.is_some());
  assert!(name_child_schemas.is_empty());
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Name2");
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
#[expect(dead_code)]
fn api_type_derive_with_format_complex_struct() {
  #[derive(ApiType)]
  struct Name {
    last_name: String,
    first_name: String,
    id: u32,
  }

  impl TypedSchema for Name {
    fn schema_type() -> String {
      "string".to_owned()
    }

    fn format() -> Option<String> {
      None
    }
  }

  #[derive(ApiType)]
  #[openapi_type(schema_type = "string")]
  struct Name2 {
    last_name: String,
    first_name: String,
    id: u32,
  }

  let name_schema = <Name as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
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
    })
  );

  let name_schema = <Name2 as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name2 as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
  assert!(name_schema.is_some());
  assert!(name_child_schemas.is_empty());
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Name2");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "type": "string",
    })
  );
}