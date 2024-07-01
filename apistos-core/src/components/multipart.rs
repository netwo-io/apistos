use crate::ApiComponent;
use actix_multipart::form::text::Text;
use actix_multipart::form::{MultipartCollect, MultipartForm};
use actix_multipart::Multipart;
use apistos_models::paths::{MediaType, RequestBody};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::ApistosSchema;
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;

impl<T> ApiComponent for MultipartForm<T>
where
  T: MultipartCollect + ApiComponent,
{
  fn content_type() -> String {
    "multipart/form-data".to_string()
  }

  fn child_schemas(oas_version: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn schema(oas_version: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version)
  }
}

impl<T> ApiComponent for Text<T>
where
  T: DeserializeOwned + ApiComponent,
{
  fn content_type() -> String {
    "multipart/form-data".to_string()
  }

  fn child_schemas(oas_version: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn schema(oas_version: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version)
  }
}

impl<T> ApiComponent for actix_multipart::form::json::Json<T>
where
  T: DeserializeOwned + ApiComponent,
{
  fn content_type() -> String {
    "multipart/form-data".to_string()
  }

  fn child_schemas(oas_version: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn schema(oas_version: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version)
  }
}

impl ApiComponent for Multipart {
  fn content_type() -> String {
    "multipart/form-data".to_string()
  }

  fn child_schemas(_: apistos_models::OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    vec![]
  }

  fn raw_schema(_: apistos_models::OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
    None
  }

  fn schema(_: apistos_models::OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    None
  }

  fn request_body(_: apistos_models::OpenApiVersion) -> Option<RequestBody> {
    Some(RequestBody {
      content: BTreeMap::from_iter(vec![(Self::content_type(), MediaType::default())]),
      required: Some(Self::required()),
      ..Default::default()
    })
  }
}
