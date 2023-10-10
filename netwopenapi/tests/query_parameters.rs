#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use actix_web::web::Query;
use netwopenapi_core::ApiComponent;
use netwopenapi_gen::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[actix_web::test]
async fn query_parameters() {
  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) enum StatusQuery {
    Active,
    Inactive,
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) struct PaginationQuery {
    pub(crate) limit: u32,
    pub(crate) offset: Option<u32>,
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) struct StructQuery {
    pub(crate) test: Option<String>,
    pub(crate) status: Option<StatusQuery>,
    #[serde(flatten)]
    pub(crate) pagination: PaginationQuery,
  }

  let parameters = <Query<StructQuery> as ApiComponent>::parameters();
  assert_eq!(parameters.len(), 4);

  let test_parameter = parameters
    .iter()
    .find(|p| p.name == *"test")
    .expect("Unable to retrieve test parameter");
  assert_eq!(test_parameter.required, Some(false));

  let status_parameter = parameters
    .iter()
    .find(|p| p.name == *"status")
    .expect("Unable to retrieve status parameter");
  assert_eq!(status_parameter.required, Some(false));

  let limit_parameter = parameters
    .iter()
    .find(|p| p.name == *"limit")
    .expect("Unable to retrieve limit parameter");
  assert_eq!(limit_parameter.required, Some(true));

  let offset_parameter = parameters
    .iter()
    .find(|p| p.name == *"offset")
    .expect("Unable to retrieve offset parameter");
  assert_eq!(offset_parameter.required, Some(false));
}

#[actix_web::test]
async fn query_parameters_with_flatten_enums() {
  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) enum StatusQuery {
    Active(u32),
    Inactive(u32),
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) enum KindQuery {
    Big(String),
    Small(String),
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) struct StructQuery {
    pub(crate) test: Option<String>,
    #[serde(flatten)]
    pub(crate) status: Option<StatusQuery>,
    #[serde(flatten)]
    pub(crate) kind: KindQuery,
  }

  let parameters = <Query<StructQuery> as ApiComponent>::parameters();
  assert_eq!(parameters.len(), 5);

  let test_parameter = parameters
    .iter()
    .find(|p| p.name == *"test")
    .expect("Unable to retrieve test parameter");
  assert_eq!(test_parameter.required, None);

  let active_parameter = parameters
    .iter()
    .find(|p| p.name == *"Active")
    .expect("Unable to retrieve Active parameter");
  assert_eq!(active_parameter.required, Some(false));

  let inactive_parameter = parameters
    .iter()
    .find(|p| p.name == *"Inactive")
    .expect("Unable to retrieve Inactive parameter");
  assert_eq!(inactive_parameter.required, Some(false));

  let big_parameter = parameters
    .iter()
    .find(|p| p.name == *"Big")
    .expect("Unable to retrieve Big parameter");
  assert_eq!(big_parameter.required, Some(false));

  let small_parameter = parameters
    .iter()
    .find(|p| p.name == *"Small")
    .expect("Unable to retrieve Small parameter");
  assert_eq!(small_parameter.required, Some(false));
}

// Imports bellow aim at making cargo-cranky happy. Those dependencies are necessary for integration-test.
use actix_service as _;
use indexmap as _;
use log as _;
use md5 as _;
use netwopenapi_models as _;
use once_cell as _;
use regex as _;
use serde_json as _;
