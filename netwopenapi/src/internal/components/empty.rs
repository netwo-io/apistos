use super::ApiComponent;
use actix_web::dev::Payload;
use actix_web::{HttpRequest, HttpResponse};
use netwopenapi_models::reference_or::ReferenceOr;
use netwopenapi_models::Schema;

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
