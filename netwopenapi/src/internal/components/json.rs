use crate::internal::components::ApiComponent;
use actix_web::web::Json;
use netwopenapi_models::reference_or::ReferenceOr;
use netwopenapi_models::Schema;

impl<T> ApiComponent for Json<T>
where
  T: ApiComponent,
{
  fn required() -> bool {
    T::required()
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    T::schema()
  }
}

#[cfg(feature = "garde")]
impl<T> ApiComponent for garde_actix_web::web::Json<T>
where
  T: ApiComponent,
{
  fn required() -> bool {
    T::required()
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    T::schema()
  }
}
