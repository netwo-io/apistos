use crate::ApiComponent;
use actix_web::dev::Payload;
use actix_web::{HttpRequest, HttpResponse};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::ApistosSchema;

macro_rules! empty_component_impl {
  ($($ty:ty),+) => {
    $(impl ApiComponent for $ty {
      fn child_schemas(_: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
        vec![]
      }
      fn schema(_: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
        None
      }
    })+
  };
}

empty_component_impl!(HttpRequest, HttpResponse, Payload, ());

#[cfg(feature = "actix-web-grants")]
impl<T> ApiComponent for actix_web_grants::authorities::AuthDetails<T>
where
  T: Eq + std::hash::Hash,
{
  fn child_schemas(_oas_version: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    vec![]
  }
  fn schema(_oas_version: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    None
  }
}
