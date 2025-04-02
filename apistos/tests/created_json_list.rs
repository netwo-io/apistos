use actix_web::App;
use actix_web::ResponseError;
use actix_web::http::StatusCode;
use actix_web::test::{TestRequest, call_service, init_service, try_read_body_json};
use apistos::ApiComponent;
use apistos::ApiErrorComponent;
use apistos::actix::CreatedJson;
use apistos::api_operation;
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos::web::{get, scope};
use apistos_models::OpenApi;
use assert_json_diff::assert_json_eq;
use core::fmt::Formatter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;

#[actix_web::test]
#[allow(clippy::panic)]
async fn created_json_list() {
  #[derive(Serialize, Deserialize, Debug, Clone, ApiErrorComponent)]
  #[openapi_error(status(code = 403))]
  pub(crate) enum ErrorResponse {
    Fobidden(String),
  }

  impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      f.write_fmt(format_args!("forbidden"))
    }
  }

  impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
      StatusCode::FORBIDDEN
    }
  }

  #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
  pub(crate) struct Test(String);

  #[api_operation(tag = "pet")]
  async fn get_array() -> Result<CreatedJson<Vec<Test>>, ErrorResponse> {
    Ok(CreatedJson(vec![Test("1".to_owned()), Test("2".to_owned())]))
  }

  let app = App::new()
    .document(Spec::default())
    .service(scope("test").route("test2", get().to(get_array)))
    .build("/openapi.json");
  let app = init_service(app).await;

  let req = TestRequest::get().uri("/openapi.json").to_request();
  let resp = call_service(&app, req).await;
  assert!(resp.status().is_success());

  let body: OpenApi = try_read_body_json(resp).await.expect("Unable to read body");
  let json = serde_json::to_value(&body).unwrap();

  assert_json_eq!(
    json,
    json!({
      "openapi": "3.0.3",
      "info": {
        "title": "",
        "version": ""
      },
      "servers": [],
      "paths": {
        "/test/test2": {
          "get": {
            "tags": [
              "pet"
            ],
            "operationId": "get_test-test2-82ec344dad380013ef60df45e872a141",
            "responses": {
              "201": {
                "description": "",
                "content": {
                  "application/json": {
                    "schema": {
                      "type": "array",
                      "items": {
                        "$ref": "#/components/schemas/Test"
                      }
                    }
                  }
                }
              },
              "403": {
                "description": "Forbidden"
              }
            },
            "deprecated": false
          }
        }
      },
      "components": {
        "schemas": {
          "Test": {
            "title": "Test",
            "type": "string"
          }
        }
      }
    })
  );
}

// Imports bellow aim at making clippy happy. Those dependencies are necessary for integration-test.
use actix_service as _;
use actix_web_lab as _;
use apistos_core as _;
use apistos_gen as _;
use apistos_plugins as _;
use apistos_rapidoc as _;
use apistos_redoc as _;
use apistos_scalar as _;
use apistos_swagger_ui as _;
use futures_util as _;
use garde_actix_web as _;
use indexmap as _;
use log as _;
use md5 as _;
use once_cell as _;
use regex as _;
