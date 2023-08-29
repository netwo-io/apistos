use actix_web::error::ParseError;
use actix_web::http::header::{Header, HeaderName, HeaderValue, InvalidHeaderValue, TryIntoHeaderValue};
use actix_web::{FromRequest, HttpMessage};
use netwopenapi::{ApiComponent, ApiHeader};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, ApiComponent)]
pub(crate) struct Pet {
  // #[openapi(example = 10)]
  pub(crate) id: Option<i64>,
  // #[openapi(example = "doggie")]
  pub(crate) name: String,
  pub(crate) category: Option<Category>,
  // #[openapi(rename = "photoUrls")]
  pub(crate) photo_urls: Vec<String>,
  pub(crate) tags: Option<Vec<Tag>>,
  /// pet status in the store
  pub(crate) status: Option<Status>,
}

/// A category for a pet
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, ApiComponent)]
pub(crate) struct Category {
  // #[openapi(example = 1)]
  pub(crate) id: Option<i64>,
  // #[openapi(example = "Dogs")]
  pub(crate) name: Option<String>,
}

/// pet status in the store
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, ApiComponent)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Status {
  Available,
  Pending,
  Sold,
}

/// A tag for a pet
// #[openapi(rename = "Pet tag")]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, ApiComponent)]
pub(crate) struct Tag {
  pub(crate) id: Option<i64>,
  pub(crate) name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, ApiComponent)]
pub(crate) struct QueryStatus {
  pub(crate) status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, ApiComponent)]
pub(crate) struct QueryTag {
  pub(crate) tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, ApiComponent)]
pub(crate) struct PetUpdatesQuery {
  pub(crate) name: String,
  pub(crate) status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, ApiComponent, ApiHeader)]
#[openapi_header(name = "X-Organization-Slug", description = "Organization of the current caller")]
pub(crate) struct OrganizationSlug(String);

impl TryIntoHeaderValue for OrganizationSlug {
  type Error = InvalidHeaderValue;

  fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
    HeaderValue::from_str(&self.0)
  }
}

impl Header for OrganizationSlug {
  fn name() -> HeaderName {
    HeaderName::from_static("X-Organization-Slug")
  }

  fn parse<M: HttpMessage>(msg: &M) -> Result<Self, ParseError> {
    msg
      .headers()
      .get(<Self as Header>::name())
      .map(|value| value.to_str())
      .transpose()
      .map_err(|e| ParseError::Header)?
      .map(|value| OrganizationSlug(value.to_string()))
      .ok_or_else(|| ParseError::Header)
  }
}
