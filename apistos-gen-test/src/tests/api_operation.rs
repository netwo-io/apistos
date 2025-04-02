use actix_multipart::form::MultipartForm;
use actix_web::dev::ServiceRequest;
use actix_web::http::header::ContentType;
use actix_web::web::Json;
use actix_web::{Error, HttpResponse, Responder};
use assert_json_diff::assert_json_eq;
use schemars::_serde_json::json;
use std::collections::HashSet;
use uuid::Uuid;

use apistos::actix::{AcceptedJson, CreatedJson, NoContent};
use apistos_core::PathItemDefinition;
use apistos_gen::api_operation;

#[allow(clippy::todo)]
mod test_models {
  use std::fmt::{Display, Formatter};
  use std::future::Ready;

  use actix_multipart::form::{Limits, MultipartCollect, State};
  use actix_multipart::{Field, MultipartError};
  use actix_web::dev::Payload;
  use actix_web::http::StatusCode;
  use actix_web::{Error, FromRequest, HttpRequest, ResponseError};
  use schemars::JsonSchema;
  use serde::{Deserialize, Serialize};

  use apistos_gen::{ApiComponent, ApiErrorComponent, ApiSecurity};

  #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
  pub(crate) struct Test {
    pub(crate) test: String,
  }

  #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
  pub(crate) struct TestResult {
    pub(crate) id: u32,
  }

  impl MultipartCollect for Test {
    fn limit(_field_name: &str) -> Option<usize> {
      todo!()
    }

    fn handle_field<'t>(
      _req: &'t HttpRequest,
      _field: Field,
      _limits: &'t mut Limits,
      _state: &'t mut State,
    ) -> futures_core::future::LocalBoxFuture<'t, Result<(), MultipartError>> {
      todo!()
    }

    fn from_state(_state: State) -> Result<Self, MultipartError> {
      todo!()
    }
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

  #[allow(clippy::duplicated_attributes)]
  #[derive(Serialize, Deserialize, Debug, Clone, ApiErrorComponent)]
  #[openapi_error(status(code = 401), status(code = 403), status(code = 404), status(code = 405))]
  pub(crate) enum MultipleErrorResponse {
    MethodNotAllowed(String),
  }

  impl Display for MultipleErrorResponse {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
      todo!()
    }
  }

  impl ResponseError for MultipleErrorResponse {
    fn status_code(&self) -> StatusCode {
      todo!()
    }
  }

  #[derive(ApiSecurity)]
  #[openapi_security(scheme(security_type(oauth2(flows(implicit(
    authorization_url = "https://authorize.com",
    refresh_url = "https://refresh.com",
    scopes(scope = "all:read", description = "Read all the things"),
    scopes(scope = "all:write", description = "Write all the things")
  ))))))]
  pub(crate) struct ApiKey;

  impl FromRequest for ApiKey {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(_req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
      todo!()
    }
  }
}

#[test]
#[allow(dead_code)]
fn api_operation() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  pub(crate) async fn test(
    _body: Json<test_models::Test>,
  ) -> Result<Json<test_models::TestResult>, test_models::ErrorResponse> {
    Ok(Json(test_models::TestResult { id: 0 }))
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          },
          "TestResult": {
            "properties": {
              "id": {
                "format": "uint32",
                "minimum": 0.0,
                "type": "integer"
              }
            },
            "required": [
              "id"
            ],
            "title": "TestResult",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
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
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/TestResult"
              }
            }
          },
          "description": ""
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_impl_responder() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  pub(crate) async fn test(_body: Json<test_models::Test>) -> impl Responder {
    HttpResponse::Ok()
  }

  #[allow(clippy::todo, clippy::unused_async)]
  async fn plop() {
    todo!()
  }

  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  pub(crate) async fn test_async(_body: Json<test_models::Test>) -> impl Responder {
    plop().await;
    HttpResponse::Ok()
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
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
      "responses": {},
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );

  let components = __openapi_test_async::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test_async::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
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
      "responses": {},
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_simple_response() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  pub(crate) async fn test(_body: Json<test_models::Test>) -> Result<Json<Uuid>, test_models::ErrorResponse> {
    Ok(Json(Uuid::new_v4()))
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
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
          "content": {
            "application/json": {
              "schema": {
                "format": "uuid",
                "title": "Uuid",
                "type": "string"
              }
            }
          },
          "description": ""
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_without_parameters() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  pub(crate) async fn test() -> Result<Json<test_models::TestResult>, test_models::ErrorResponse> {
    Ok(Json(test_models::TestResult { id: 0 }))
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "TestResult": {
            "properties": {
              "id": {
                "format": "uint32",
                "minimum": 0.0,
                "type": "integer"
              }
            },
            "required": [
              "id"
            ],
            "title": "TestResult",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
      "description": "Add a new pet to the store\\\nPlop",
      "responses": {
        "200": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "#/components/schemas/TestResult"
              }
            }
          },
          "description": ""
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_no_content() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  pub(crate) async fn test(_body: Json<test_models::Test>) -> Result<NoContent, test_models::ErrorResponse> {
    Ok(NoContent)
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
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
        "204": {
          "description": ""
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_created_json() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  pub(crate) async fn test(
    _body: Json<test_models::Test>,
  ) -> Result<CreatedJson<test_models::TestResult>, test_models::ErrorResponse> {
    Ok(CreatedJson(test_models::TestResult { id: 1 }))
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          },
          "TestResult": {
            "properties": {
              "id": {
                "format": "uint32",
                "minimum": 0.0,
                "type": "integer"
              }
            },
            "required": [
              "id"
            ],
            "title": "TestResult",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
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
        "201": {
          "content": {
            "application/json": {
              "schema": {
                "properties": {
                  "id": {
                    "format": "uint32",
                    "minimum": 0.0,
                    "type": "integer"
                  }
                },
                "required": [
                  "id"
                ],
                "title": "TestResult",
                "type": "object"
              }
            }
          },
          "description": ""
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_created_json_simple_response() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  pub(crate) async fn test(_body: Json<test_models::Test>) -> Result<CreatedJson<Uuid>, test_models::ErrorResponse> {
    Ok(CreatedJson(Uuid::new_v4()))
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
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
        "201": {
          "content": {
            "application/json": {
              "schema": {
                "format": "uuid",
                "title": "Uuid",
                "type": "string"
              }
            }
          },
          "description": ""
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_accepted_json() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  pub(crate) async fn test(
    _body: Json<test_models::Test>,
  ) -> Result<AcceptedJson<test_models::TestResult>, test_models::ErrorResponse> {
    Ok(AcceptedJson(test_models::TestResult { id: 0 }))
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          },
          "TestResult": {
            "properties": {
              "id": {
                "format": "uint32",
                "minimum": 0.0,
                "type": "integer"
              }
            },
            "required": [
              "id"
            ],
            "title": "TestResult",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
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
        "202": {
          "content": {
            "application/json": {
              "schema": {
                "properties": {
                  "id": {
                    "format": "uint32",
                    "minimum": 0.0,
                    "type": "integer"
                  }
                },
                "required": [
                  "id"
                ],
                "title": "TestResult",
                "type": "object"
              }
            }
          },
          "description": ""
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_deprecated() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet", deprecated)]
  pub(crate) async fn test(
    _body: Json<test_models::Test>,
  ) -> Result<CreatedJson<test_models::TestResult>, test_models::ErrorResponse> {
    Ok(CreatedJson(test_models::TestResult { id: 4 }))
  }

  let components = __openapi_test::components();
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          },
          "TestResult": {
            "properties": {
              "id": {
                "format": "uint32",
                "minimum": 0.0,
                "type": "integer"
              }
            },
            "required": [
              "id"
            ],
            "title": "TestResult",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": true,
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
        "201": {
          "content": {
            "application/json": {
              "schema": {
                "properties": {
                  "id": {
                    "format": "uint32",
                    "minimum": 0.0,
                    "type": "integer"
                  }
                },
                "required": [
                  "id"
                ],
                "title": "TestResult",
                "type": "object"
              }
            }
          },
          "description": ""
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );

  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet", operation_id = "test2")]
  #[deprecated]
  pub(crate) async fn test2(
    _body: Json<test_models::Test>,
  ) -> Result<CreatedJson<test_models::TestResult>, test_models::ErrorResponse> {
    Ok(CreatedJson(test_models::TestResult { id: 2 }))
  }

  let components = __openapi_test2::components();
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test2::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          },
          "TestResult": {
            "properties": {
              "id": {
                "format": "uint32",
                "minimum": 0.0,
                "type": "integer"
              }
            },
            "required": [
              "id"
            ],
            "title": "TestResult",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": true,
      "description": "Add a new pet to the store\\\nPlop",
      "operationId": "test2",
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
        "201": {
          "content": {
            "application/json": {
              "schema": {
                "properties": {
                  "id": {
                    "format": "uint32",
                    "minimum": 0.0,
                    "type": "integer"
                  }
                },
                "required": [
                  "id"
                ],
                "title": "TestResult",
                "type": "object"
              }
            }
          },
          "description": ""
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_skip() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet", skip)]
  pub(crate) async fn test(
    _body: Json<test_models::Test>,
  ) -> Result<CreatedJson<test_models::TestResult>, test_models::ErrorResponse> {
    Ok(CreatedJson(test_models::TestResult { id: 6 }))
  }

  let components = __openapi_test::components();
  assert!(components.is_empty());

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    operation,
    json!({
      "responses": {}
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_error() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet", error_code = "404", error_code = "401")]
  pub(crate) async fn test(
    _body: Json<test_models::Test>,
  ) -> Result<CreatedJson<test_models::TestResult>, test_models::MultipleErrorResponse> {
    Ok(CreatedJson(test_models::TestResult { id: 1 }))
  }

  let components = __openapi_test::components();
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          },
          "TestResult": {
            "properties": {
              "id": {
                "format": "uint32",
                "minimum": 0.0,
                "type": "integer"
              }
            },
            "required": [
              "id"
            ],
            "title": "TestResult",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
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
        "201": {
          "content": {
            "application/json": {
              "schema": {
                "properties": {
                  "id": {
                    "format": "uint32",
                    "minimum": 0.0,
                    "type": "integer"
                  }
                },
                "required": [
                  "id"
                ],
                "title": "TestResult",
                "type": "object"
              }
            }
          },
          "description": ""
        },
        "401": {
          "description": "Unauthorized"
        },
        "404": {
          "description": "Not Found"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_security() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(security_scope(name = "api_key", scope = "read:pets"))]
  pub(crate) async fn test(
    _body: Json<test_models::Test>,
    _key: test_models::ApiKey,
  ) -> Result<CreatedJson<test_models::TestResult>, test_models::MultipleErrorResponse> {
    Ok(CreatedJson(test_models::TestResult { id: 0 }))
  }

  let components = __openapi_test::components();
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          },
          "TestResult": {
            "properties": {
              "id": {
                "format": "uint32",
                "minimum": 0.0,
                "type": "integer"
              }
            },
            "required": [
              "id"
            ],
            "title": "TestResult",
            "type": "object"
          }
        },
        "securitySchemes": {
          "api_key": {
            "flows": {
              "implicit": {
                "authorizationUrl": "https://authorize.com",
                "refreshUrl": "https://refresh.com",
                "scopes": {
                  "all:read": "Read all the things",
                  "all:write": "Write all the things"
                }
              }
            },
            "type": "oauth2"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
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
        "201": {
          "content": {
            "application/json": {
              "schema": {
                "properties": {
                  "id": {
                    "format": "uint32",
                    "minimum": 0.0,
                    "type": "integer"
                  }
                },
                "required": [
                  "id"
                ],
                "title": "TestResult",
                "type": "object"
              }
            }
          },
          "description": ""
        },
        "401": {
          "description": "Unauthorized"
        },
        "403": {
          "description": "Forbidden"
        },
        "404": {
          "description": "Not Found"
        },
        "405": {
          "description": "Method Not Allowed"
        }
      },
      "security": [
        {
          "api_key": [
            "read:pets"
          ]
        }
      ],
      "summary": "Add a new pet to the store"
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_multipart() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation()]
  pub(crate) async fn test(
    _payload: MultipartForm<test_models::Test>,
  ) -> Result<HttpResponse, test_models::MultipleErrorResponse> {
    Ok(HttpResponse::Ok().content_type(ContentType::plaintext()).json(""))
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
      "description": "Add a new pet to the store\\\nPlop",
      "requestBody": {
        "content": {
          "multipart/form-data": {
            "schema": {
              "$ref": "#/components/schemas/Test"
            }
          }
        },
        "required": true
      },
      "responses": {
        "200": {
          "description": ""
        },
        "401": {
          "description": "Unauthorized"
        },
        "403": {
          "description": "Forbidden"
        },
        "404": {
          "description": "Not Found"
        },
        "405": {
          "description": "Method Not Allowed"
        }
      },
      "summary": "Add a new pet to the store"
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_consumes_produces() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(consumes = "application/problem+json", produces = "text/plain")]
  pub(crate) async fn test(
    _payload: MultipartForm<test_models::Test>,
  ) -> Result<HttpResponse, test_models::MultipleErrorResponse> {
    Ok(HttpResponse::Ok().content_type(ContentType::plaintext()).json(""))
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
      "description": "Add a new pet to the store\\\nPlop",
      "requestBody": {
        "content": {
          "application/problem+json": {
            "schema": {
              "$ref": "#/components/schemas/Test"
            }
          }
        },
        "required": true
      },
      "responses": {
        "200": {
          "content": {
            "text/plain": {}
          },
          "description": ""
        },
        "401": {
          "description": "Unauthorized"
        },
        "403": {
          "description": "Forbidden"
        },
        "404": {
          "description": "Not Found"
        },
        "405": {
          "description": "Method Not Allowed"
        }
      },
      "summary": "Add a new pet to the store"
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_operation_root_vec() {
  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[api_operation(tag = "pet")]
  pub(crate) async fn test(
    _body: Json<test_models::Test>,
  ) -> Result<Json<Vec<test_models::TestResult>>, test_models::ErrorResponse> {
    Ok(Json(vec![test_models::TestResult { id: 0 }]))
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          },
          "TestResult": {
            "properties": {
              "id": {
                "format": "uint32",
                "minimum": 0.0,
                "type": "integer"
              }
            },
            "required": [
              "id"
            ],
            "title": "TestResult",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
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
          "content": {
            "application/json": {
              "schema": {
                "type": "array",
                "items": {
                  "$ref": "#/components/schemas/TestResult"
                }
              }
            }
          },
          "description": ""
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}

#[test]
fn api_operation_actix_web_grant() {
  #[allow(clippy::unused_async)]
  async fn extract(_req: &ServiceRequest) -> Result<HashSet<String>, Error> {
    Ok(Default::default())
  }

  /// Add a new pet to the store
  /// Add a new pet to the store
  /// Plop
  #[actix_web_grants::protect("ADMIN")]
  #[api_operation(tag = "pet")]
  pub(crate) async fn test(
    _body: Json<test_models::Test>,
  ) -> Result<Json<Vec<test_models::TestResult>>, test_models::ErrorResponse> {
    Ok(Json(vec![test_models::TestResult { id: 0 }]))
  }

  let components = __openapi_test::components();
  // only one component here because: error does not have schema and Test is used both for query and response
  assert_eq!(components.len(), 1);
  let components = serde_json::to_value(components).expect("Unable to serialize as Json");

  let operation = __openapi_test::operation();
  let operation = serde_json::to_value(operation).expect("Unable to serialize as Json");

  assert_json_eq!(
    components,
    json!([
      {
        "schemas": {
          "Test": {
            "properties": {
              "test": {
                "type": "string"
              }
            },
            "required": [
              "test"
            ],
            "title": "Test",
            "type": "object"
          },
          "TestResult": {
            "properties": {
              "id": {
                "format": "uint32",
                "minimum": 0.0,
                "type": "integer"
              }
            },
            "required": [
              "id"
            ],
            "title": "TestResult",
            "type": "object"
          }
        }
      }
    ])
  );
  assert_json_eq!(
    operation,
    json!({
      "deprecated": false,
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
          "content": {
            "application/json": {
              "schema": {
                "type": "array",
                "items": {
                  "$ref": "#/components/schemas/TestResult"
                }
              }
            }
          },
          "description": ""
        },
        "405": {
          "description": "Invalid input"
        }
      },
      "summary": "Add a new pet to the store",
      "tags": [
        "pet"
      ]
    })
  );
}
