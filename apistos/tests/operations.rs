#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use actix_web::test::{call_service, init_service, try_read_body_json, TestRequest};
use actix_web::{App, Error};
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos::web::{put, resource, scope, Scope};
use apistos_gen::api_operation;
use apistos_models::info::Info;
use apistos_models::OpenApi;

#[actix_web::test]
async fn operations() {
  const OAS_PATH: &str = "/api-docs/openapi.json";

  const PATH_PREFIX: &str = "/api/rest/v1/finance";

  fn router_api_v1() -> Scope {
    scope(PATH_PREFIX)
      .service(api_ticket())
      .service(api_account_no_trailing_slash())
  }

  const TICKET_SUB_PATH_PREFIX: &str = "/ticket";
  const ACCOUNT_SUB_PATH_PREFIX: &str = "/account";

  pub(crate) fn api_ticket() -> Scope {
    scope(TICKET_SUB_PATH_PREFIX).service(resource("/").route(put().to(update_ticket)))
  }

  pub(crate) fn api_account_no_trailing_slash() -> Scope {
    scope(ACCOUNT_SUB_PATH_PREFIX).service(resource("").route(put().to(update_account)))
  }

  #[api_operation(tag = "ticket")]
  pub(crate) async fn update_ticket() -> Result<String, Error> {
    Ok("success".to_string())
  }

  #[api_operation(tag = "account")]
  pub(crate) async fn update_account() -> Result<String, Error> {
    Ok("success".to_string())
  }

  let spec = Spec {
    info: Info {
      title: "An API".to_string(),
      version: "1.0.0".to_string(),
      ..Default::default()
    },
    ..Default::default()
  };
  let app = App::new().document(spec).service(router_api_v1()).build(OAS_PATH);
  let app = init_service(app).await;

  let req = TestRequest::get().uri(OAS_PATH).to_request();
  let resp = call_service(&app, req).await;
  assert!(resp.status().is_success());

  let body: OpenApi = try_read_body_json(resp).await.expect("Unable to read body");
  let paths = body.paths.paths;
  assert_eq!(paths.len(), 2);

  let (path, _path_item) = paths.iter().find(|(p, _)| p.contains("ticket")).unwrap();
  assert_eq!(path, "/api/rest/v1/finance/ticket/");

  let (path, _path_item) = paths.iter().find(|(p, _)| p.contains("account")).unwrap();
  assert_eq!(path, "/api/rest/v1/finance/account");
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
use garde_actix_web as _;
use indexmap as _;
use log as _;
use md5 as _;
use once_cell as _;
use regex as _;
use schemars as _;
use serde as _;
use serde_json as _;
