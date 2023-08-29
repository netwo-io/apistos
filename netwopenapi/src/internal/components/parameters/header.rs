use crate::ApiComponent;
use actix_web::web::Header;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn, ParameterStyle};
use utoipa::openapi::{RefOr, Schema};

pub trait ApiHeader {
  fn name() -> String;
  fn description() -> Option<String> {
    None
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

  fn parameters() -> Vec<Parameter> {
    vec![ParameterBuilder::new()
      .parameter_in(ParameterIn::Header)
      .name(T::name())
      .description(T::description())
      .schema(Self::schema().map(|(_, schema)| schema).or_else(|| Self::raw_schema()))
      .style(Some(ParameterStyle::Simple))
      .build()]
  }
}
