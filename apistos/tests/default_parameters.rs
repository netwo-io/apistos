#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use actix_web::http::StatusCode;
use actix_web::test::{TestRequest, call_service, init_service, try_read_body_json};
use actix_web::web::{Header, Json, Path};
use actix_web::{App, ResponseError};
use apistos::app::OpenApiWrapper;
use apistos::spec::{DefaultParameterAccessor, DefaultParameters, Spec};
use apistos::web::{get, resource, scope};
use apistos_gen::{ApiComponent, ApiErrorComponent, ApiHeader, api_operation};
use apistos_models::OpenApi;
use apistos_models::info::Info;
use apistos_models::paths::{OperationType, Parameter, ParameterIn};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::tag::Tag;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[actix_web::test]
async fn default_parameters() {
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

  #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
  struct TestHeaderStruct {
    plop: u32,
    plap: String,
  }

  #[allow(dead_code)]
  #[derive(Clone, Debug, JsonSchema, ApiHeader)]
  #[openapi_header(
    name = "X-Env-Complex",
    description = "`X-Env-Complx` header should contain the current env",
    required = true
  )]
  struct SomeComplexHeader(TestHeaderStruct);

  #[allow(dead_code)]
  #[derive(Clone, Debug, JsonSchema, ApiHeader)]
  #[openapi_header(
    name = "X-Env",
    description = "`X-Env` header should contain the current env",
    required = true
  )]
  struct SomeHeader(String);

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

  let default_parameters_macro = <Header<SomeHeader> as DefaultParameterAccessor>::get_default_parameter();
  let default_complex_parameters_macro =
    <Header<SomeComplexHeader> as DefaultParameterAccessor>::get_default_parameter();
  let simple_default_parameters = DefaultParameters {
    parameters: vec![Parameter {
      name: "X-SomeParam".to_string(),
      _in: ParameterIn::Header,
      required: Some(true),
      ..Default::default()
    }],
    components: vec![],
  };
  let default_parameters = vec![
    default_parameters_macro,
    default_complex_parameters_macro,
    simple_default_parameters,
  ];
  let spec = Spec {
    info: info.clone(),
    tags: tags.clone(),
    default_parameters,
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
  let parameters: Vec<ReferenceOr<Parameter>> = body
    .paths
    .paths
    .get(&operation_path.to_string())
    .cloned()
    .unwrap_or_default()
    .operations
    .get(&OperationType::Get)
    .cloned()
    .unwrap_or_default()
    .parameters;

  assert_eq!(parameters.len(), 5);

  let parameters_name = parameters
    .iter()
    .map(|p| match p {
      ReferenceOr::Object(obj) => obj.name.clone(),
      ReferenceOr::Reference { _ref } => _ref.split('/').last().unwrap_or_default().to_string(),
    })
    .collect::<Vec<String>>();

  assert_eq!(
    parameters_name,
    vec![
      "plop_id".to_string(),
      "clap_name".to_string(),
      "X-Env".to_string(),
      "X-Env-Complex".to_string(),
      "X-SomeParam".to_string()
    ]
  );

  let parameter_components = body.components.clone().map(|c| c.parameters).unwrap_or_default();
  assert_eq!(parameter_components.len(), 3);
  assert_eq!(
    parameter_components.keys().cloned().collect::<Vec<String>>(),
    vec![
      "X-Env".to_string(),
      "X-Env-Complex".to_string(),
      "X-SomeParam".to_string()
    ]
  );

  let schema_components = body.components.map(|c| c.schemas).unwrap_or_default();
  assert_eq!(schema_components.len(), 2);
  assert_eq!(
    schema_components.keys().cloned().collect::<Vec<String>>(),
    vec!["Test".to_string(), "TestHeaderStruct".to_string(),]
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
use assert_json_diff as _;
use futures_util as _;
use garde_actix_web as _;
use indexmap as _;
use log as _;
use md5 as _;
use once_cell as _;
use regex as _;
use serde_json as _;
