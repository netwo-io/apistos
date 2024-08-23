#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use actix_web::http::StatusCode;
use actix_web::test::{call_service, init_service, try_read_body_json, TestRequest};
use actix_web::web::{Json, Path};
use actix_web::{App, ResponseError};
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos::web::{get, resource, scope};
use apistos_gen::{api_operation, ApiComponent, ApiErrorComponent};
use apistos_models::info::Info;
use apistos_models::paths::{OperationType, Parameter, ParameterDefinition};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::tag::Tag;
use apistos_models::OpenApi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[actix_web::test]
async fn path_parameter_replacement() {
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

  let openapi_path = "/test.json";
  let operation_path = "/test/{plop_id}/{clap_name}/";

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
    .service(scope(operation_path).service(resource("").route(get().to(test))))
    .build(openapi_path);
  let app = init_service(app).await;

  let req = TestRequest::get().uri(openapi_path).to_request();
  let resp = call_service(&app, req).await;
  assert!(resp.status().is_success());

  let body: OpenApi = try_read_body_json(resp).await.expect("Unable to read body");
  let parameters: Vec<Parameter> = body
    .paths
    .paths
    .get(&operation_path.to_string())
    .cloned()
    .unwrap_or_default()
    .operations
    .get(&OperationType::Get)
    .cloned()
    .unwrap_or_default()
    .parameters
    .iter()
    .filter_map(|p| match p {
      ReferenceOr::Reference { .. } => None,
      ReferenceOr::Object(obj) => Some(obj.clone()),
    })
    .collect();

  assert_eq!(parameters.len(), 2);

  let first_parameter = parameters.first().cloned().unwrap_or_default();
  assert_eq!(first_parameter.name, "plop_id");
  let first_parameter_schema = first_parameter
    .definition
    .and_then(|p| match p {
      ParameterDefinition::Schema(ReferenceOr::Object(sch)) => Some(sch),
      _ => None,
    })
    .unwrap_or_default();
  assert_eq!(
    first_parameter_schema
      .inner()
      .as_object()
      .and_then(|obj| obj.get("type"))
      .and_then(|_type| _type.as_str()),
    Some("integer")
  );

  let last_parameter = parameters.last().cloned().unwrap_or_default();
  assert_eq!(last_parameter.name, "clap_name");
  let last_parameter_schema = last_parameter
    .definition
    .and_then(|p| match p {
      ParameterDefinition::Schema(ReferenceOr::Object(sch)) => Some(sch),
      _ => None,
    })
    .unwrap_or_default();
  assert_eq!(
    last_parameter_schema
      .inner()
      .as_object()
      .and_then(|obj| obj.get("type"))
      .and_then(|_type| _type.as_str()),
    Some("string")
  );
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
use serde_json as _;
