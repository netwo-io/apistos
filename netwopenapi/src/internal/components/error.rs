use actix_web::error::Error;
use std::collections::BTreeMap;
use utoipa::openapi::{RefOr, Response, Schema};

pub trait ApiErrorComponent {
  fn schemas_by_status_code() -> BTreeMap<String, (String, RefOr<Schema>)>;
  fn error_responses() -> Vec<(String, Response)>;
}

impl ApiErrorComponent for Error {
  fn schemas_by_status_code() -> BTreeMap<String, (String, RefOr<Schema>)> {
    BTreeMap::default()
  }

  fn error_responses() -> Vec<(String, Response)> {
    vec![]
  }
}
