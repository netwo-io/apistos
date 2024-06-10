use crate::ApiComponent;
use actix_web::web::Form;
use apistos_models::reference_or::ReferenceOr;
use apistos_models::ApistosSchema;

impl<T> ApiComponent for Form<T>
where
  T: ApiComponent,
{
  fn content_type() -> String {
    "application/x-www-form-urlencoded".to_string()
  }

  fn child_schemas(oas_version: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn schema(oas_version: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version)
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

  fn child_schemas(oas_version: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn schema(oas_version: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version)
  }
}
