#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use actix_web::web::Query;
#[cfg(feature = "lab_query")]
use actix_web_lab::extract::Query as LabQuery;
use apistos_core::ApiComponent;
use apistos_gen::ApiComponent;
#[cfg(all(feature = "lab_query", feature = "garde"))]
use garde_actix_web::web::LabQuery as GardeLabQuery;
#[cfg(feature = "garde")]
use garde_actix_web::web::Query as GardeQuery;
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

#[cfg(feature = "lab_query")]
#[actix_web::test]
async fn lab_query_parameters() {
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

  let parameters = <LabQuery<StructQuery> as ApiComponent>::parameters();
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

#[cfg(feature = "garde")]
#[actix_web::test]
async fn garde_query_parameters() {
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

  let parameters = <GardeQuery<StructQuery> as ApiComponent>::parameters();
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

#[cfg(all(feature = "lab_query", feature = "garde"))]
#[actix_web::test]
async fn garde_lab_query_parameters() {
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

  let parameters = <GardeLabQuery<StructQuery> as ApiComponent>::parameters();
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

// Imports bellow aim at making clippy happy. Those dependencies are necessary for integration-test.
use actix_service as _;
use apistos_models as _;
use apistos_plugins as _;
use apistos_rapidoc as _;
use apistos_redoc as _;
use apistos_scalar as _;
use apistos_swagger_ui as _;
use assert_json_diff as _;
use futures_util as _;
use indexmap as _;
use log as _;
use md5 as _;
use once_cell as _;
use regex as _;
use serde_json as _;

#[cfg(not(feature = "lab_query"))]
use actix_web_lab as _;
#[cfg(not(feature = "garde"))]
use garde_actix_web as _;
