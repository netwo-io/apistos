use actix_web::web::Header;
use assert_json_diff::assert_json_eq;
use schemars::JsonSchema;
use serde_json::json;

use crate::utils::assert_schema;
use apistos_core::ApiComponent;
use apistos_gen::ApiHeader;

#[test]
#[allow(dead_code)]
fn api_header_derive() {
  #[derive(JsonSchema, ApiHeader)]
  #[openapi_header(
    name = "X-Organization-Slug",
    description = "Organization of the current caller",
    required = true
  )]
  struct OrganizationSlug(String);

  let schema = <OrganizationSlug as ApiComponent>::schema();
  let child_schemas = <OrganizationSlug as ApiComponent>::child_schemas();
  let header_parameter = <Header<OrganizationSlug> as ApiComponent>::parameters();
  assert!(schema.is_some());
  assert!(child_schemas.is_empty());
  assert!(!header_parameter.is_empty());
  let (schema_name, schema) = schema.expect("schema should be defined");
  assert_eq!(schema_name, "OrganizationSlug");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "$schema": "https://spec.openapis.org/oas/3.0/schema/2021-09-28#/definitions/Schema",
      "title": "OrganizationSlug",
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
      "in": "header",
      "name": "X-Organization-Slug",
      "required": true,
      "schema": {
        "$schema": "https://spec.openapis.org/oas/3.0/schema/2021-09-28#/definitions/Schema",
        "title": "OrganizationSlug",
        "type": "string"
      },
      "style": "simple"
    })
  );
}
