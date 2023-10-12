#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use actix_web::http::StatusCode;
use actix_web::test::{call_service, init_service, try_read_body_json, TestRequest};
use actix_web::web::{Header, Json, Path};
use actix_web::{App, ResponseError};
use netwopenapi::app::OpenApiWrapper;
use netwopenapi::spec::Spec;
use netwopenapi::web::{get, resource, scope};
use netwopenapi_core::ApiComponent;
use netwopenapi_gen::{api_operation, ApiComponent, ApiErrorComponent, ApiHeader};
use netwopenapi_models::info::Info;
use netwopenapi_models::paths::{OperationType, Parameter, ParameterIn};
use netwopenapi_models::reference_or::ReferenceOr;
use netwopenapi_models::tag::Tag;
use netwopenapi_models::OpenApi;
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

  #[allow(unused_tuple_struct_fields)]
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

  let mut default_parameters_macro = <Header<SomeHeader> as ApiComponent>::parameters();
  let simple_default_parameters = Parameter {
    name: "X-SomeParam".to_string(),
    _in: ParameterIn::Header,
    required: Some(true),
    ..Default::default()
  };
  let mut default_parameters = vec![];
  default_parameters.append(&mut default_parameters_macro);
  default_parameters.push(simple_default_parameters);
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

  assert_eq!(parameters.len(), 4);

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
      "X-SomeParam".to_string()
    ]
  );

  let parameter_components = body.components.map(|c| c.parameters).unwrap_or_default();
  assert_eq!(parameter_components.len(), 2);
  assert_eq!(
    parameter_components.keys().cloned().collect::<Vec<String>>(),
    vec!["X-Env".to_string(), "X-SomeParam".to_string()]
  );
}

// Imports bellow aim at making cargo-cranky happy. Those dependencies are necessary for integration-test.
use actix_service as _;
use indexmap as _;
use log as _;
use md5 as _;
use netwopenapi_core as _;
use once_cell as _;
use regex as _;
use serde_json as _;
