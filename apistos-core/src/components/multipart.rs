use crate::ApiComponent;
use actix_multipart::Multipart;
use actix_multipart::form::text::Text;
use actix_multipart::form::{MultipartCollect, MultipartForm};
use apistos_models::Schema;
use apistos_models::paths::{MediaType, RequestBody};
use apistos_models::reference_or::ReferenceOr;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;

impl<T> ApiComponent for MultipartForm<T>
where
  T: MultipartCollect + ApiComponent,
{
  fn content_type() -> String {
    "multipart/form-data".to_string()
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    T::schema()
  }
}

impl<T> ApiComponent for Text<T>
where
  T: DeserializeOwned + ApiComponent,
{
  fn content_type() -> String {
    "multipart/form-data".to_string()
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    T::schema()
  }
}

impl<T> ApiComponent for actix_multipart::form::json::Json<T>
where
  T: DeserializeOwned + ApiComponent,
{
  fn content_type() -> String {
    "multipart/form-data".to_string()
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    T::schema()
  }
}

impl ApiComponent for Multipart {
  fn content_type() -> String {
    "multipart/form-data".to_string()
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    vec![]
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    None
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }

  fn request_body() -> Option<RequestBody> {
    Some(RequestBody {
      content: BTreeMap::from_iter(vec![(Self::content_type(), MediaType::default())]),
      required: Some(Self::required()),
      ..Default::default()
    })
  }
}
