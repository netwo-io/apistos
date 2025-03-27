use actix_web::App;

use apistos::paths::OperationType;
use apistos::web::{get, scope};

use actix_web::test::{TestRequest, call_service, init_service, try_read_body_json};
use apistos::actix::CreatedJson;
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos::{ApiComponent, InstanceType, SingleOrVec};
use apistos_models::OpenApi;

use actix_web::http::StatusCode;

use actix_web::ResponseError;
use apistos::ApiErrorComponent;

use apistos::api_operation;

use core::fmt::Formatter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

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

  let schema_for_reponse_201 = body
    .paths
    .paths
    .get("/test/test2")
    .cloned()
    .unwrap_or_default()
    .operations
    .get(&OperationType::Get)
    .cloned()
    .unwrap_or_default()
    .responses
    .responses
    .get("201")
    .cloned()
    .unwrap()
    .get_object()
    .unwrap_or_default()
    .content
    .get("application/json")
    .cloned()
    .unwrap_or_default()
    .schema
    .unwrap();
  let schema = schema_for_reponse_201.get_object().unwrap().into_object();
  let instance_type = schema.instance_type.unwrap();
  let SingleOrVec::Single(instance_type) = instance_type else {
    panic!()
  };
  assert_eq!(*instance_type, InstanceType::Array);
  let SingleOrVec::Single(array) = schema.array.unwrap().items.unwrap() else {
    panic!()
  };
  assert_eq!(
    array.deref().clone().into_object().reference.unwrap(),
    "#/components/schemas/Test".to_owned()
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
use serde_json as _;
