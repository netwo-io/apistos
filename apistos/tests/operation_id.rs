#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use actix_web::http::StatusCode;
use actix_web::test::{TestRequest, call_service, init_service, try_read_body_json};
use actix_web::web::{Json, Path};
use actix_web::{App, ResponseError};
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos::web::{get, resource, scope};
use apistos_gen::{ApiComponent, ApiErrorComponent, api_operation};
use apistos_models::OpenApi;
use apistos_models::info::Info;
use apistos_models::paths::OperationType;
use apistos_models::tag::Tag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[actix_web::test]
async fn default_operation_id() {
  #[derive(Serialize, Deserialize, Debug, Clone, ApiErrorComponent)]
  #[openapi_error(status(code = 405, description = "Invalid input"))]
  pub(crate) enum ErrorResponse {
    MethodNotAllowed(String),
  }

  impl Display for ErrorResponse {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
      panic!()
    }
  }

  impl ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
      panic!()
    }
  }

  #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
  struct Test {
    id_number: u32,
    id_string: String,
  }

  #[api_operation(tag = "pet")]
  pub(crate) async fn test(_params: Path<(u32, String)>) -> Result<Json<Test>, ErrorResponse> {
    panic!()
  }

  #[api_operation(tag = "pet")]
  pub(crate) async fn test2(_params: Path<u32>) -> Result<Json<Test>, ErrorResponse> {
    panic!()
  }

  #[api_operation(tag = "pet", operation_id = "test3")]
  pub(crate) async fn test3(_params: Path<u32>) -> Result<Json<Test>, ErrorResponse> {
    panic!()
  }

  let openapi_path = "/test.json";
  let operation_path = "/test/{plop_id}/{clap_name}";
  let operation_path2 = "/test/line/{plop_id}";
  let operation_path3 = "/test/line2/{plop_id}";

  let info = Info {
    title: "A well documented API".to_string(),
    description: Some("Really well document I mean it".to_string()),
    terms_of_service: Some("https://terms.com".to_string()),
    ..Default::default()
  };
  let tags = vec![Tag {
    name: "A super tag".to_owned(),
    ..Default::default()
  }];
  let spec = Spec {
    info: info.clone(),
    tags: tags.clone(),
    ..Default::default()
  };
  let app = App::new()
    .document(spec)
    .service(
      scope("test")
        .service(resource("/{plop_id}/{clap_name}").route(get().to(test)))
        .service(resource("/line/{plop_id}").route(get().to(test2)))
        .service(resource("/line2/{plop_id}").route(get().to(test3))),
    )
    .build(openapi_path);
  let app = init_service(app).await;

  let req = TestRequest::get().uri(openapi_path).to_request();
  let resp = call_service(&app, req).await;
  assert!(resp.status().is_success());

  let body: OpenApi = try_read_body_json(resp).await.expect("Unable to read body");
  let paths = body.paths.paths;

  let operation = paths.get(&operation_path.to_string()).cloned();
  assert!(operation.is_some());
  let operation = operation
    .unwrap_or_default()
    .operations
    .get(&OperationType::Get)
    .cloned()
    .unwrap_or_default();
  let operation_id = operation.operation_id;
  assert_eq!(
    operation_id,
    Some("get_test-6dcecae37dd6df4024e8ec3a32ca81ea".to_string())
  );

  let operation2 = paths.get(&operation_path2.to_string()).cloned();
  assert!(operation2.is_some());
  let operation2 = operation2
    .unwrap_or_default()
    .operations
    .get(&OperationType::Get)
    .cloned()
    .unwrap_or_default();
  let operation_id2 = operation2.operation_id;
  assert_eq!(
    operation_id2,
    Some("get_test-line-e6d32c785aeb754bd68fcc6878de32d6".to_string())
  );

  let operation3 = paths.get(&operation_path3.to_string()).cloned();
  assert!(operation3.is_some());
  let operation3 = operation3
    .unwrap_or_default()
    .operations
    .get(&OperationType::Get)
    .cloned()
    .unwrap_or_default();
  let operation_id3 = operation3.operation_id;
  assert_eq!(operation_id3, Some("test3".to_string()))
}

// Imports bellow aim at making clippy happy. Those dependencies are necessary for integration-test.
use actix_service as _;
use actix_web_lab as _;
use apistos_core as _;
use apistos_plugins as _;
use apistos_rapidoc as _;
use apistos_redoc as _;
use apistos_scalar as _;
use apistos_swagger_ui as _;
use assert_json_diff as _;
use futures_util as _;
use garde_actix_web as _;
use indexmap as _;
use log as _;
use md5 as _;
use once_cell as _;
use regex as _;
use serde_json as _;
