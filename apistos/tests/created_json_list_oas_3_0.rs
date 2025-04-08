use actix_web::App;
use actix_web::ResponseError;
use actix_web::http::StatusCode;
use actix_web::test::{TestRequest, call_service, init_service, try_read_body_json};
use apistos::ApiComponent;
use apistos::ApiErrorComponent;
use apistos::OpenApiVersion;
use apistos::actix::CreatedJson;
use apistos::api_operation;
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos::web::{get, scope};
use apistos_models::OpenApi;
use core::fmt::Formatter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    .document(Spec {
      openapi: OpenApiVersion::OAS3_0,
      ..Default::default()
    })
    .service(scope("test").route("test2", get().to(get_array)))
    .build("/openapi.json");
  let app = init_service(app).await;

  let req = TestRequest::get().uri("/openapi.json").to_request();
  let resp = call_service(&app, req).await;
  assert!(resp.status().is_success());

  let body: OpenApi = try_read_body_json(resp).await.expect("Unable to read body");
  let json_body = serde_json::to_value(&body).unwrap();

  let schema = json_body
    .get("paths")
    .unwrap()
    .get("/test/test2")
    .unwrap()
    .get("get")
    .unwrap()
    .get("responses")
    .unwrap()
    .get("201")
    .unwrap()
    .get("content")
    .unwrap()
    .get("application/json")
    .unwrap()
    .get("schema")
    .unwrap();
  let Value::String(schema_type) = schema.get("type").unwrap() else {
    panic!();
  };
  assert_eq!(schema_type, "array");
  let Value::String(schema_vec_ref) = schema.get("items").unwrap().get("$ref").unwrap() else {
    panic!();
  };
  assert_eq!(schema_vec_ref, "#/components/schemas/Test");
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
use serde_json::{self as _, Value};
