use actix_web::App;
use apistos::paths::{OperationType, ParameterDefinition};
use apistos::web::redirect;

use actix_web::test::{TestRequest, call_service, init_service, try_read_body_json};
use apistos::OpenApiVersion;
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos_models::{OpenApi, VersionSpecificSchema};
use serde_json::Value;

#[actix_web::test]
async fn actix_redirect_oas_3_0() {
  let app = App::new()
    .document(Spec {
      openapi: OpenApiVersion::OAS3_0,
      ..Default::default()
    })
    .service(redirect("/duck", "https://duck.com"))
    .build("/openapi.json");
  let app = init_service(app).await;

  let req = TestRequest::get().uri("/openapi.json").to_request();
  let resp = call_service(&app, req).await;
  assert!(resp.status().is_success());

  let body: OpenApi = try_read_body_json(resp).await.expect("Unable to read body");
  println!("{}", serde_json::to_string_pretty(&body).unwrap());
  let mut paths: Vec<&String> = body.paths.paths.keys().collect();
  paths.sort();
  let expected_paths = vec!["/duck"];
  assert_eq!(paths, expected_paths);
  assert_eq!(body.paths.paths.values().flat_map(|v| v.operations.values()).count(), 7);
  let duck = body.paths.paths.get("/duck").unwrap();
  let get = duck.operations.get(&OperationType::Get).unwrap();
  let responses = get.responses.responses.get("307").unwrap();
  let reponse = responses.clone().get_object().unwrap();
  let location = reponse.headers.get("Location").unwrap();
  let location_header = location.clone().get_object().unwrap();
  let location_header_definition = location_header.definition.unwrap();
  let ParameterDefinition::Content(btree_map) = location_header_definition else {
    panic!();
  };
  let section = &btree_map.get("text/plain").unwrap().schema;
  let Some(schema) = section else {
    panic!();
  };
  let VersionSpecificSchema::OAS3_0(inner) = schema else {
    panic!();
  };
  let inner = inner.clone();
  let enum_entry = inner.get_object().unwrap();
  let enum_entry = enum_entry.inner().get("enum").unwrap();
  let Value::Array(list) = enum_entry else {
    panic!();
  };
  let redirect = list.first().unwrap().clone();
  assert_eq!(redirect, Value::String("https://duck.com".to_owned()));
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
use serde_json as _;
