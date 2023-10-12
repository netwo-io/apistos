use actix_web::http::StatusCode;
use actix_web::ResponseError;
use apistos::ApiErrorComponent;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Serialize, Deserialize, Clone, ApiErrorComponent)]
#[openapi_error(
  status(code = 403),
  status(code = 404),
  status(code = 405, description = "Invalid input"),
  status(code = 409)
)]
pub enum ErrorResponse {
  MethodNotAllowed(String),
  NotFound(String),
  Conflict(String),
  Unauthorized(String),
}

impl Debug for ErrorResponse {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ErrorResponse::NotFound(str)
      | ErrorResponse::Conflict(str)
      | ErrorResponse::Unauthorized(str)
      | ErrorResponse::MethodNotAllowed(str) => {
        write!(f, "{str}")
      }
    }
  }
}

impl Display for ErrorResponse {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ErrorResponse::NotFound(str)
      | ErrorResponse::Conflict(str)
      | ErrorResponse::Unauthorized(str)
      | ErrorResponse::MethodNotAllowed(str) => {
        write!(f, "{str}")
      }
    }
  }
}

impl ResponseError for ErrorResponse {
  fn status_code(&self) -> StatusCode {
    match self {
      ErrorResponse::MethodNotAllowed(_) => StatusCode::METHOD_NOT_ALLOWED,
      ErrorResponse::NotFound(_) => StatusCode::NOT_FOUND,
      ErrorResponse::Conflict(_) => StatusCode::CONFLICT,
      ErrorResponse::Unauthorized(_) => StatusCode::UNAUTHORIZED,
    }
  }
}
