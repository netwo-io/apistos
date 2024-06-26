use actix_web::dev::Payload;
use actix_web::error::ParseError;
use actix_web::http::header::{Header, HeaderName, HeaderValue, InvalidHeaderValue, TryIntoHeaderValue};
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use apistos::{ApiComponent, ApiCookie, ApiHeader, ApiType, ApiWebhookComponent, TypedSchema};
use num_traits::Float;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future::Ready;
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub(crate) struct Pet {
  pub(crate) id: Option<i64>,
  pub(crate) name: String,
  pub(crate) category: Option<Category>,
  pub(crate) photo_urls: Vec<String>,
  pub(crate) tags: Option<Vec<Tag>>,
  /// pet status in the store
  pub(crate) status: Option<Status>,
}

/// A category for a pet
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub(crate) struct Category {
  pub(crate) id: Option<Finite<f32>>,
  pub(crate) name: Option<Name>,
  pub(crate) docs: Option<Url>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ApiType)]
#[serde(transparent)]
#[allow(clippy::useless_vec)]
pub struct Finite<N: Float> {
  inner: N,
}

impl<N: Float> TypedSchema for Finite<N> {
  fn schema_type() -> String {
    "number".to_string()
  }

  fn format() -> Option<String> {
    Some("float".to_string())
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, ApiType)]
pub struct Name(String);

impl TypedSchema for Name {
  fn schema_type() -> String {
    "string".to_string()
  }

  fn format() -> Option<String> {
    None
  }
}

/// pet status in the store
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Status {
  Available,
  Pending,
  Sold,
}

/// A tag for a pet
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub(crate) struct Tag {
  pub(crate) id: Option<Decimal>,
  pub(crate) name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub(crate) struct QueryStatus {
  pub(crate) status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub(crate) struct QueryTag {
  pub(crate) tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub(crate) struct PetUpdatesQuery {
  pub(crate) name: String,
  pub(crate) status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiHeader)]
#[openapi_header(
  name = "X-Organization-Slug",
  description = "Organization of the current caller",
  required = true
)]
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
      .map_err(|_e| ParseError::Header)?
      .map(|value| OrganizationSlug(value.to_string()))
      .ok_or_else(|| ParseError::Header)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiCookie)]
#[openapi_cookie(name = "X-Realm", description = "Realm of the current caller", deprecated = true)]
pub(crate) struct Realm(String);

impl FromRequest for Realm {
  type Error = Error;
  type Future = Ready<Result<Self, Self::Error>>;

  fn from_request(_req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
    todo!()
  }
}

#[derive(ApiWebhookComponent)]
pub(crate) enum WebhookHolder {
  #[openapi_webhook(name = "PetCreated", response(code = 200))]
  PetCreated,
  #[openapi_webhook(skip)]
  AnotherWebhook,
}
