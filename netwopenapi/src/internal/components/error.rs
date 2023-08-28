use actix_web::error::Error;
use utoipa::openapi::{RefOr, Response, Schema};

pub trait ApiErrorComponent {
  fn schemas() -> Vec<(String, RefOr<Schema>)>;
  fn error_responses() -> Vec<(String, Response)>;
}

impl ApiErrorComponent for Error {
  fn schemas() -> Vec<(String, RefOr<Schema>)> {
    vec![]
  }

  fn error_responses() -> Vec<(String, Response)> {
    vec![]
  }
}
