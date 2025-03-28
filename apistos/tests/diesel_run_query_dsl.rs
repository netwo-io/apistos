use apistos::ApiComponent;

#[allow(unused_imports)]
use diesel_async::RunQueryDsl;
use schemars::JsonSchema;

#[derive(Clone, Copy, Debug, JsonSchema, ApiComponent)]
struct ChangeAnalysisMethod {
  #[allow(dead_code)]
  pub obsolete: Option<bool>,
}

#[test]
#[expect(
  clippy::print_stdout,
  reason = "derive(ApiComponent) Should not raise error diesel::query_builder::Query` is not implemented for `&mut std::vec::Vec<serde_json::Value>"
)]
fn apistos_could_build_with_diesel_runquerydsl() {
  println!("ApiComponent can live with diesel_async::RunQueryDsl");
}

// Imports bellow aim at making clippy happy. Those dependencies are necessary for integration-test.
use actix_service as _;
use actix_web as _;
use actix_web_lab as _;
use apistos_core as _;
use apistos_gen as _;
use apistos_models as _;
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
use serde as _;
use serde_json as _;
