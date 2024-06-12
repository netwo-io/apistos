use apistos::OpenApiVersion;
use assert_json_diff::assert_json_eq;
use schemars::JsonSchema;
use serde_json::json;

use crate::utils::assert_schema;
use apistos_core::ApiComponent;
use apistos_gen::ApiCookie;

#[test]
#[allow(dead_code)]
fn api_cookie_derive() {
  #[derive(JsonSchema, ApiCookie)]
  #[openapi_cookie(
    name = "X-Organization-Slug",
    description = "Organization of the current caller",
    required = true
  )]
  struct OrganizationSlugCookie(String);

  let schema = <OrganizationSlugCookie as ApiComponent>::schema(OpenApiVersion::OAS3_1);
  let child_schemas = <OrganizationSlugCookie as ApiComponent>::child_schemas(OpenApiVersion::OAS3_1);
  let header_parameter = <OrganizationSlugCookie as ApiComponent>::parameters(OpenApiVersion::OAS3_1);
  assert!(schema.is_some());
  assert!(child_schemas.is_empty());
  assert!(!header_parameter.is_empty());
  let (schema_name, schema) = schema.expect("schema should be defined");
  assert_eq!(schema_name, "OrganizationSlugCookie");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "$schema": "https://json-schema.org/draft/2020-12/schema",
      "title": "OrganizationSlugCookie",
      "type": "string"
    })
  );
  assert_eq!(header_parameter.len(), 1);
  let header_parameter = header_parameter.first().expect("missing parameter");
  let json = serde_json::to_value(header_parameter).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "deprecated": false,
      "description": "Organization of the current caller",
      "in": "cookie",
      "name": "X-Organization-Slug",
      "required": true,
      "schema": {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "OrganizationSlugCookie",
        "type": "string"
      }
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_cookie_derive_deprecated() {
  #[derive(JsonSchema, ApiCookie)]
  #[openapi_cookie(
    name = "X-Organization-Slug",
    description = "Organization of the current caller",
    required = true
  )]
  #[deprecated]
  struct OrganizationSlugCookie(String);

  let schema = <OrganizationSlugCookie as ApiComponent>::schema(OpenApiVersion::OAS3_1);
  let child_schemas = <OrganizationSlugCookie as ApiComponent>::child_schemas(OpenApiVersion::OAS3_1);
  let header_parameter = <OrganizationSlugCookie as ApiComponent>::parameters(OpenApiVersion::OAS3_1);
  assert!(schema.is_some());
  assert!(child_schemas.is_empty());
  assert!(!header_parameter.is_empty());
  let (schema_name, schema) = schema.expect("schema should be defined");
  assert_eq!(schema_name, "OrganizationSlugCookie");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "$schema": "https://json-schema.org/draft/2020-12/schema",
      "title": "OrganizationSlugCookie",
      "type": "string",
      "deprecated": true
    })
  );
  assert_eq!(header_parameter.len(), 1);
  let header_parameter = header_parameter.first().expect("missing parameter");
  let json = serde_json::to_value(header_parameter).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "deprecated": true,
      "description": "Organization of the current caller",
      "in": "cookie",
      "name": "X-Organization-Slug",
      "required": true,
      "schema": {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "OrganizationSlugCookie",
        "type": "string",
        "deprecated": true
      }
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_cookie_derive_deprecated_attribute() {
  #[derive(JsonSchema, ApiCookie)]
  #[openapi_cookie(
    name = "X-Organization-Slug",
    description = "Organization of the current caller",
    required = true,
    deprecated = true
  )]
  struct OrganizationSlugCookie2(String);

  let schema = <OrganizationSlugCookie2 as ApiComponent>::schema(OpenApiVersion::OAS3_1);
  let child_schemas = <OrganizationSlugCookie2 as ApiComponent>::child_schemas(OpenApiVersion::OAS3_1);
  let header_parameter = <OrganizationSlugCookie2 as ApiComponent>::parameters(OpenApiVersion::OAS3_1);
  assert!(schema.is_some());
  assert!(child_schemas.is_empty());
  assert!(!header_parameter.is_empty());
  let (schema_name, schema) = schema.expect("schema should be defined");
  assert_eq!(schema_name, "OrganizationSlugCookie2");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "$schema": "https://json-schema.org/draft/2020-12/schema",
      "title": "OrganizationSlugCookie2",
      "type": "string",
      "deprecated": true
    })
  );
  assert_eq!(header_parameter.len(), 1);
  let header_parameter = header_parameter.first().expect("missing parameter");
  let json = serde_json::to_value(header_parameter).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "deprecated": true,
      "description": "Organization of the current caller",
      "in": "cookie",
      "name": "X-Organization-Slug",
      "required": true,
      "schema": {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "OrganizationSlugCookie2",
        "type": "string",
        "deprecated": true
      }
    })
  );
}
