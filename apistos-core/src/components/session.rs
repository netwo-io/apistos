use crate::ApiComponent;
use apistos_models::ApistosSchema;
use apistos_models::paths::{Parameter, ParameterDefinition, ParameterIn, RequestBody};
use apistos_models::reference_or::ReferenceOr;
use schemars::json_schema;

impl ApiComponent for actix_session::Session {
  fn required() -> bool {
    true
  }

  fn child_schemas(_oas_version: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    vec![]
  }

  fn raw_schema(oas_version: apistos_models::OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
    Some(
      ApistosSchema::new(
        json_schema!({
          "type": "string",
        }),
        oas_version,
      )
      .into(),
    )
  }

  fn schema(_oas_version: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    None
  }

  fn request_body(_oas_version: apistos_models::OpenApiVersion, _description: Option<String>) -> Option<RequestBody> {
    None
  }

  fn parameters(oas_version: apistos_models::OpenApiVersion, description: Option<String>) -> Vec<Parameter> {
    vec![Parameter {
      name: "id".to_string(), // from default actix-session's CookieConfiguration
      _in: ParameterIn::Cookie,
      required: Some(true),
      definition: Self::raw_schema(oas_version).map(ParameterDefinition::Schema),
      description,
      ..Default::default()
    }]
  }
}
