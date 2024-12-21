use crate::ApiComponent;
use apistos_models::paths::{Parameter, ParameterDefinition, ParameterIn, RequestBody};
use apistos_models::reference_or::ReferenceOr;
use schemars::schema::{InstanceType, Schema, SchemaObject, SingleOrVec, StringValidation};

impl ApiComponent for actix_session::Session {
  fn required() -> bool {
    true
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    vec![]
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    Some(ReferenceOr::Object(Schema::Object(SchemaObject {
      instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
      string: Some(Box::new(StringValidation::default())),
      ..Default::default()
    })))
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    vec![Parameter {
      name: "id".to_string(), // from default actix-session's CookieConfiguration
      _in: ParameterIn::Cookie,
      required: Some(true),
      definition: Self::raw_schema().map(ParameterDefinition::Schema),
      ..Default::default()
    }]
  }
}
