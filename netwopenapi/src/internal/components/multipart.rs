use crate::ApiComponent;
use actix_multipart::form::text::Text;
use actix_multipart::form::{MultipartCollect, MultipartForm};
use actix_multipart::Multipart;
use serde::de::DeserializeOwned;
use utoipa::openapi::request_body::{RequestBody, RequestBodyBuilder};
use utoipa::openapi::{ContentBuilder, RefOr, Schema};

impl<T> ApiComponent for MultipartForm<T>
where
  T: MultipartCollect + ApiComponent,
{
  fn content_type() -> String {
    "multipart/form-data".to_string()
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
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

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
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

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    T::schema()
  }
}

impl ApiComponent for Multipart {
  fn content_type() -> String {
    "multipart/form-data".to_string()
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    vec![]
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    None
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    None
  }

  fn request_body() -> Option<RequestBody> {
    Some(
      RequestBodyBuilder::new()
        .content(Self::content_type(), ContentBuilder::new().build())
        .required(Some(Self::required()))
        .build(),
    )
  }
}
