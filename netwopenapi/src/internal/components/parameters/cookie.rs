use crate::ApiComponent;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{Deprecated, RefOr, Required, Schema};

pub trait ApiCookie {
  fn child_schemas() -> Vec<(String, RefOr<Schema>)>;
  fn schema() -> Option<(String, RefOr<Schema>)>;
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

impl<T> ApiComponent for T
where
  T: ApiCookie,
{
  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    <T as ApiCookie>::child_schemas()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    <T as ApiCookie>::schema()
  }

  fn parameters() -> Vec<Parameter> {
    vec![ParameterBuilder::new()
      .parameter_in(ParameterIn::Cookie)
      .name(T::name())
      .description(T::description())
      .required(<T as ApiCookie>::required())
      .deprecated(Some(<T as ApiCookie>::deprecated()))
      .schema(Self::schema().map(|(_, schema)| schema).or_else(|| Self::raw_schema()))
      .build()]
  }
}
