use actix_web::web::Json;
use assert_json_diff::assert_json_eq;
use schemars::_serde_json::json;
use serde::{Deserialize, Serialize};

use apistos::{JsonSchema, OpenApiVersion};
use apistos_core::PathItemDefinition;
use apistos_gen::{api_callback, api_operation, ApiComponent};

#[allow(clippy::todo)]
mod test_models {
  use actix_web::http::header::{Header, HeaderName, HeaderValue, InvalidHeaderValue, TryIntoHeaderValue};
  use std::fmt::{Display, Formatter};

  use actix_web::error::ParseError;
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
fn api_callback() {
  use actix_web::web::Header;
  use test_models::OrganizationSlug;

  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet", callbacks(name = "onData", callback(path = "{$request.body.test}/data", post = test_callback)))]
  pub(crate) async fn test(
    _body: Json<test_models::Test>,
  ) -> Result<Json<test_models::TestResult>, test_models::ErrorResponse> {
    Ok(Json(test_models::TestResult { id: 0 }))
  }

  #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
  pub(crate) struct TestCallbackResult {
    pub(crate) id: u32,
  }

  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_callback(
    component = "Header<OrganizationSlug>",
    response(code = 200, component = "TestCallbackResult")
  )]
  pub(crate) async fn test_callback(
    _body: Json<test_models::Test>,
  ) -> Result<Json<test_models::TestResult>, test_models::ErrorResponse> {
    Ok(Json(test_models::TestResult { id: 0 }))
  }

  let components = __openapi_test::components(OpenApiVersion::OAS3_0);
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation(OpenApiVersion::OAS3_0);
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "title": "Test",
            "type": "object",
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ]
          },
          "TestCallbackResult": {
            "title": "TestCallbackResult",
            "type": "object",
            "properties": {
              "id": {
                "type": "integer",
                "format": "uint32",
                "minimum": 0
              }
            },
            "required": [
              "id"
            ]
          },
          "TestResult": {
            "title": "TestResult",
            "type": "object",
            "properties": {
              "id": {
                "type": "integer",
                "format": "uint32",
                "minimum": 0
              }
            },
            "required": [
              "id"
            ]
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "tags": [
        "pet"
      ],
      "summary": "Add a new pet to the store",
      "description": "Add a new pet to the store\\\nPlop",
      "requestBody": {
        "content": {
          "application/json": {
            "schema": {
              "$ref": "#/components/schemas/Test"
            }
          }
        },
        "required": true
      },
      "responses": {
        "200": {
          "description": "",
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/TestResult"
              }
            }
          }
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "callbacks": {
        "onData": {
          "{$request.body.test}/data": {
            "post": {
              "parameters": [
                {
                  "name": "X-Organization-Slug",
                  "in": "header",
                  "description": "Organization of the current caller",
                  "required": true,
                  "deprecated": false,
                  "style": "simple",
                  "schema": {
                    "title": "OrganizationSlug",
                    "type": "string"
                  }
                }
              ],
              "responses": {
                "200": {
                  "description": "",
                  "content": {
                    "application/json": {
                      "schema": {
                        "$ref": "#/components/schemas/TestCallbackResult"
                      }
                    }
                  }
                }
              }
            }
          }
        }
      },
      "deprecated": false
    })
  );
}