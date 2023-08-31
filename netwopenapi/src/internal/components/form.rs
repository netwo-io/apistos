use crate::ApiComponent;
use actix_web::web::Form;
use utoipa::openapi::{RefOr, Schema};

impl<T> ApiComponent for Form<T>
where
  T: ApiComponent,
{
  fn content_type() -> String {
    "application/x-www-form-urlencoded".to_string()
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    T::schema()
  }
}

#[cfg(feature = "garde")]
impl<T> ApiComponent for actix_web_garde::web::Form<T>
where
  T: ApiComponent,
{
  fn content_type() -> String {
    "application/x-www-form-urlencoded".to_string()
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    T::schema()
  }
}
