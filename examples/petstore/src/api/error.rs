use std::fmt::{Debug, Display, Formatter};
use actix_web::ResponseError;
use futures::AsyncWriteExt;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum ErrorResponse {
  NotFound(String),
  Conflict(String),
  Unauthorized(String),
}

impl Debug for ErrorResponse {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ErrorResponse::NotFound(str) |
      ErrorResponse::Conflict(str) |
      ErrorResponse::Unauthorized(str) => write!(f, "{str}")
    }
  }
}

impl Display for ErrorResponse {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ErrorResponse::NotFound(str) |
      ErrorResponse::Conflict(str) |
      ErrorResponse::Unauthorized(str) => write!(f, "{str}")
    }
  }
}

impl ResponseError for ErrorResponse {}
