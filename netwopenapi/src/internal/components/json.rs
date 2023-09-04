use crate::internal::components::ApiComponent;
use actix_web::web::Json;
use utoipa::openapi::{RefOr, Required, Schema};

impl<T> ApiComponent for Json<T>
where
  T: ApiComponent,
{
  fn required() -> Required {
    T::required()
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    T::schema()
  }
}

#[cfg(feature = "garde")]
impl<T> ApiComponent for garde_actix_web::web::Json<T>
where
  T: ApiComponent,
{
  fn required() -> Required {
    T::required()
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    T::schema()
  }
}
