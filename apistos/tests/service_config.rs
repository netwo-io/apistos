use actix_web::web::{Json, Path};
use actix_web::{App, Error};

use actix_web::test::{TestRequest, call_service, init_service, try_read_body_json};
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos::web::{ServiceConfig, get};
use apistos_gen::api_operation;
use apistos_models::OpenApi;

#[actix_web::test]
async fn actix_nested_service_config_configure() {
  #[api_operation(tag = "pet")]
  pub(crate) async fn test(_params: Path<(u32, String)>) -> Result<Json<()>, Error> {
    Ok(Json(()))
  }

  fn cfg_root(cfg: &mut ServiceConfig) {
    cfg.configure(cfg_sub);
  }

  fn cfg_sub(cfg: &mut ServiceConfig) {
    cfg.route("/", get().to(test));
  }

  let app = App::new()
    .document(Spec::default())
    .configure(cfg_root)
    .build("/openapi.json");
  let app = init_service(app).await;

  let req = TestRequest::get().uri("/openapi.json").to_request();
  let resp = call_service(&app, req).await;
  assert!(resp.status().is_success());

  let body: OpenApi = try_read_body_json(resp).await.expect("Unable to read body");
  let mut paths: Vec<&String> = body.paths.paths.keys().collect();
  paths.sort();

  let expected_paths = vec!["/"];

  assert_eq!(paths, expected_paths);

  assert_eq!(body.paths.paths.values().flat_map(|v| v.operations.values()).count(), 1);
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
use schemars as _;
use serde as _;
use serde_json as _;
