use crate::ApiComponent;
use actix_web::web::Header;
use apistos_models::Schema;
use apistos_models::paths::{Parameter, ParameterDefinition, ParameterIn, ParameterStyle, RequestBody};
use apistos_models::reference_or::ReferenceOr;

pub trait ApiHeader {
  fn name() -> String;
  fn description() -> Option<String> {
    None
  }
  fn required() -> bool {
    Default::default()
  }
  fn deprecated() -> bool {
    Default::default()
  }
}

impl<T> ApiComponent for Header<T>
where
  T: ApiComponent + ApiHeader,
{
  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    vec![Parameter {
      name: T::name(),
      _in: ParameterIn::Header,
      description: T::description(),
      required: Some(<T as ApiHeader>::required()),
      deprecated: Some(<T as ApiHeader>::deprecated()),
      style: Some(ParameterStyle::Simple),
      definition: T::schema()
        .map(|(_, schema)| schema)
        .or_else(Self::raw_schema)
        .map(ParameterDefinition::Schema),
      ..Default::default()
    }]
  }
}

#[cfg(feature = "garde")]
impl<T> ApiComponent for garde_actix_web::web::Header<T>
where
  T: ApiComponent + ApiHeader,
{
  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    vec![Parameter {
      name: T::name(),
      _in: ParameterIn::Header,
      description: T::description(),
      required: Some(<T as ApiHeader>::required()),
      deprecated: Some(<T as ApiHeader>::deprecated()),
      style: Some(ParameterStyle::Simple),
      definition: T::schema()
        .map(|(_, schema)| schema)
        .or_else(Self::raw_schema)
        .map(ParameterDefinition::Schema),
      ..Default::default()
    }]
  }
}
