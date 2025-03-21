use crate::ApiComponent;
use actix_web::web::Form;
use apistos_models::Schema;
use apistos_models::reference_or::ReferenceOr;

impl<T> ApiComponent for Form<T>
where
  T: ApiComponent,
{
  fn content_type() -> String {
    "application/x-www-form-urlencoded".to_string()
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    T::schema()
  }
}

#[cfg(feature = "garde")]
impl<T> ApiComponent for garde_actix_web::web::Form<T>
where
  T: ApiComponent,
{
  fn content_type() -> String {
    "application/x-www-form-urlencoded".to_string()
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    T::schema()
  }
}
