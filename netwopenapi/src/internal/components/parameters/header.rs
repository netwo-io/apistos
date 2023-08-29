use crate::ApiComponent;
use actix_web::web::Header;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn, ParameterStyle};
use utoipa::openapi::request_body::RequestBody;
use utoipa::openapi::{Deprecated, RefOr, Required, Schema};

pub trait ApiHeader {
  fn name() -> String;
  fn description() -> Option<String> {
    None
  }
  fn required() -> Required {
    Default::default()
  }
  fn deprecated() -> Deprecated {
    Default::default()
  }
}

impl<T> ApiComponent for Header<T>
where
  T: ApiComponent + ApiHeader,
{
  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    T::schema()
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    vec![ParameterBuilder::new()
      .parameter_in(ParameterIn::Header)
      .name(T::name())
      .description(T::description())
      .required(<T as ApiHeader>::required())
      .deprecated(Some(<T as ApiHeader>::deprecated()))
      .schema(Self::schema().map(|(_, schema)| schema).or_else(|| Self::raw_schema()))
      .style(Some(ParameterStyle::Simple))
      .build()]
  }
}
