use actix_web::App;
use apistos::web::redirect;

use actix_web::test::{TestRequest, call_service, init_service, try_read_body_json};
use apistos::OpenApiVersion;
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos_models::OpenApi;

#[allow(clippy::panic)]
#[actix_web::test]
async fn actix_redirect_oas_3_1() {
  let app = App::new()
    .document(Spec {
      openapi: OpenApiVersion::OAS3_1,
      ..Default::default()
    })
    .service(redirect("/duck", "https://duck.com"))
    .build("/openapi.json");
  let app = init_service(app).await;

  let req = TestRequest::get().uri("/openapi.json").to_request();
  let resp = call_service(&app, req).await;
  assert!(resp.status().is_success());

  let body: OpenApi = try_read_body_json(resp).await.expect("Unable to read body");
  let json_body = serde_json::to_value(&body).unwrap();
  let methods = json_body.get("paths").unwrap().get("/duck").unwrap();
  let Value::Object(methods) = methods else {
    panic!();
  };
  assert_eq!(methods.keys().len(), 7);
  let response_307 = methods
    .get("get")
    .unwrap()
    .get("responses")
    .unwrap()
    .get("307")
    .unwrap();
  let location_header = response_307.get("headers").unwrap().get("Location").unwrap();
  let redirect_value = location_header
    .get("content")
    .unwrap()
    .get("text/plain")
    .unwrap()
    .get("schema")
    .unwrap()
    .get("const")
    .unwrap();
  assert_eq!(redirect_value, &Value::String("https://duck.com".to_owned()))
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
use schemars as _;
use serde as _;
use serde_json::{self as _, Value};
