#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use actix_web::http::StatusCode;
use actix_web::test::{TestRequest, call_service, init_service, try_read_body_json};
use actix_web::web::{Json, Path};
use actix_web::{App, ResponseError};
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos::web::{get, resource, scope};
use apistos_gen::{ApiComponent, ApiErrorComponent, api_operation};
use apistos_models::OpenApi;
use apistos_models::info::Info;
use apistos_models::paths::{OperationType, Parameter, ParameterDefinition};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::tag::Tag;
use schemars::JsonSchema;
use schemars::schema::{InstanceType, SchemaObject, SingleOrVec};
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
    .and_then(|p| {
      if let ParameterDefinition::Schema(schema) = p {
        if let ReferenceOr::Object(sch) = *schema {
          return Some(sch.into_object().clone());
        }
      }
      None
    })
    .unwrap_or_default();
  assert_eq!(
    first_parameter_schema.instance_type,
    Some(SingleOrVec::Single(Box::new(InstanceType::Integer)))
  );

  let last_parameter = parameters.last().cloned().unwrap_or_default();
  assert_eq!(last_parameter.name, "clap_name");
  let last_parameter_schema = last_parameter
    .definition
    .and_then(|p| {
      if let ParameterDefinition::Schema(schema) = p {
        if let ReferenceOr::Object(sch) = *schema {
          return Some(sch.into_object().clone());
        }
      }
      None
    })
    .unwrap_or_default();
  assert_eq!(
    last_parameter_schema.instance_type,
    Some(SingleOrVec::Single(Box::new(InstanceType::String)))
  );
}

#[actix_web::test]
async fn struct_path_parameter_schema_matches_parameter_name() {
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
    id: u32,
  }

  #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
  struct ExamplePath {
    string_param: String,
    float_param: f64,
    int_param: u32,
    bool_param: bool,
  }

  #[api_operation(tag = "pet")]
  pub(crate) async fn test(_params: Path<ExamplePath>) -> Result<Json<Test>, ErrorResponse> {
    panic!()
  }

  let openapi_path = "/test.json";
  let operation_path = "/api/{int_param}/{bool_param}/{string_param}/{float_param}";

  let spec = Spec {
    info: Info {
      title: "A well documented API".to_string(),
      ..Default::default()
    },
    tags: vec![Tag {
      name: "A super tag".to_owned(),
      ..Default::default()
    }],
    ..Default::default()
  };
  let app = App::new()
    .document(spec)
    .service(
      scope("/api/{int_param}/{bool_param}").service(resource("/{string_param}/{float_param}").route(get().to(test))),
    )
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

  assert_eq!(parameters.len(), 4);

  let schema_for = |name: &str| -> SchemaObject {
    parameters
      .iter()
      .find(|p| p.name == name)
      .and_then(|p| p.definition.clone())
      .and_then(|p| {
        if let ParameterDefinition::Schema(schema) = p {
          if let ReferenceOr::Object(sch) = *schema {
            return Some(sch.into_object().clone());
          }
        }
        None
      })
      .unwrap_or_default()
  };

  assert_eq!(
    schema_for("int_param").instance_type,
    Some(SingleOrVec::Single(Box::new(InstanceType::Integer)))
  );
  assert_eq!(
    schema_for("bool_param").instance_type,
    Some(SingleOrVec::Single(Box::new(InstanceType::Boolean)))
  );
  assert_eq!(
    schema_for("string_param").instance_type,
    Some(SingleOrVec::Single(Box::new(InstanceType::String)))
  );
  assert_eq!(
    schema_for("float_param").instance_type,
    Some(SingleOrVec::Single(Box::new(InstanceType::Number)))
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
