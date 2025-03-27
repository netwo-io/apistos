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
