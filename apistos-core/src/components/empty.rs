use crate::ApiComponent;
use actix_web::dev::Payload;
use actix_web::{HttpRequest, HttpResponse};
use apistos_models::Schema;
use apistos_models::reference_or::ReferenceOr;

macro_rules! empty_component_impl {
  ($($ty:ty),+) => {
    $(impl ApiComponent for $ty {
      fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
        vec![]
      }
      fn schema() -> Option<(String, ReferenceOr<Schema>)> {
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
  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    vec![]
  }
  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }
}
