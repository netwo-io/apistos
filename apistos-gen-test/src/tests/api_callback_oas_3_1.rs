use actix_web::web::Json;
use assert_json_diff::assert_json_eq;
use schemars::_serde_json::json;
use serde::{Deserialize, Serialize};

use apistos::{JsonSchema, OpenApiVersion};
use apistos_core::PathItemDefinition;
use apistos_gen::{api_callback, api_operation, ApiComponent};

#[allow(clippy::todo)]
mod test_models {
  use std::fmt::{Display, Formatter};

  use actix_web::http::StatusCode;
  use actix_web::ResponseError;
  use schemars::JsonSchema;
  use serde::{Deserialize, Serialize};

  use apistos_gen::{ApiComponent, ApiErrorComponent};

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
}

#[test]
#[allow(dead_code)]
fn api_callback() {
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
  #[api_callback(response(code = 200, component = TestCallbackResult))]
  pub(crate) async fn test_callback(
    _body: Json<test_models::Test>,
  ) -> Result<Json<test_models::TestResult>, test_models::ErrorResponse> {
    Ok(Json(test_models::TestResult { id: 0 }))
  }

  let components = __openapi_test::components(OpenApiVersion::OAS3_1);
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation(OpenApiVersion::OAS3_1);
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(components, json!([{}]));
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
              "$schema": "https://json-schema.org/draft/2020-12/schema",
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
                "$schema": "https://json-schema.org/draft/2020-12/schema",
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
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "callbacks": {
        "onData": {
          "{$request.body.test}/data": {
            "post": {
              "responses": {
                "200": {
                  "description": "",
                  "content": {
                    "application/json": {
                      "schema": {
                        "$schema": "https://json-schema.org/draft/2020-12/schema",
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
