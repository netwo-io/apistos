use assert_json_diff::assert_json_eq;
use schemars::_serde_json::json;

use apistos::{ApiWebhook, OpenApiVersion};
use apistos_gen::ApiWebhookComponent;

#[allow(clippy::todo)]
mod test_models {
  use std::fmt::{Display, Formatter};

  use actix_web::error::ParseError;
  use actix_web::http::header::{Header, HeaderName, HeaderValue, InvalidHeaderValue, TryIntoHeaderValue};
  use actix_web::http::StatusCode;
  use actix_web::{HttpMessage, ResponseError};
  use schemars::JsonSchema;
  use serde::{Deserialize, Serialize};

  use apistos_gen::{ApiComponent, ApiErrorComponent, ApiHeader};

  #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
  pub(crate) struct Test {
    pub(crate) test: String,
  }

  #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
  pub(crate) struct TestResult {
    pub(crate) id: u32,
  }

  #[derive(Serialize, Deserialize, Debug, Clone, ApiErrorComponent)]
  #[openapi_error(status(code = 405, description = "Invalid input"))]
  pub(crate) enum ErrorResponse {
    MethodNotAllowed(String),
  }

  impl Display for ErrorResponse {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
      todo!()
    }
  }

  impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
      todo!()
    }
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
}

#[test]
#[allow(dead_code)]
fn api_webhook() {
  use actix_web::web::Header;
  use test_models::OrganizationSlug;

  #[derive(ApiWebhookComponent)]
  #[openapi_webhook(name = "TestWebhook", component = "Header<OrganizationSlug>", response(code = 200))]
  struct TestStruct {}

  let components = TestStruct::components(OpenApiVersion::OAS3_0);
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 0);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let webhooks = TestStruct::webhooks(OpenApiVersion::OAS3_0);
  let webhooks = serde_json::to_value(webhooks).expect("Unable to serialize as Json");

  assert_json_eq!(components, json!([]));
  assert_json_eq!(webhooks, json!({}));
}
