use crate::ApiComponent;
use actix_web::web::Header;
use apistos_models::paths::{Parameter, ParameterDefinition, ParameterIn, ParameterStyle, RequestBody};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::{ApistosSchema, OpenApiVersion};

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
  fn child_schemas(oas_version: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn raw_schema(oas_version: OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
    T::raw_schema(oas_version)
  }

  fn schema(_: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    None
  }

  fn request_body(_: OpenApiVersion) -> Option<RequestBody> {
    None
  }

  fn parameters(oas_version: OpenApiVersion) -> Vec<Parameter> {
    vec![Parameter {
      name: T::name(),
      _in: ParameterIn::Header,
      description: T::description(),
      required: Some(<T as ApiHeader>::required()),
      deprecated: Some(<T as ApiHeader>::deprecated()),
      style: Some(ParameterStyle::Simple),
      definition: T::schema(oas_version)
        .map(|(_, schema)| schema)
        .or_else(|| Self::raw_schema(oas_version))
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
  fn child_schemas(oas_version: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn raw_schema(oas_version: OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
    T::raw_schema(oas_version)
  }

  fn schema(_: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    None
  }

  fn request_body(_: OpenApiVersion) -> Option<RequestBody> {
    None
  }

  fn parameters(oas_version: OpenApiVersion) -> Vec<Parameter> {
    vec![Parameter {
      name: T::name(),
      _in: ParameterIn::Header,
      description: T::description(),
      required: Some(<T as ApiHeader>::required()),
      deprecated: Some(<T as ApiHeader>::deprecated()),
      style: Some(ParameterStyle::Simple),
      definition: T::schema(oas_version)
        .map(|(_, schema)| schema)
        .or_else(|| Self::raw_schema(oas_version))
        .map(ParameterDefinition::Schema),
      ..Default::default()
    }]
  }
}
