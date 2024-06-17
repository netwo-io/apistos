use std::collections::BTreeMap;
use std::fmt::Debug;

use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use serde::Serialize;

use apistos_core::__internal::{response_from_raw_schema, response_from_schema};
pub use apistos_core::{ResponderWrapper, ResponseWrapper};
use apistos_models::paths::{RequestBody, Response, Responses};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::{ApistosSchema, OpenApiVersion};

use crate::ApiComponent;

/// Empty struct to represent a 204 empty response
#[derive(Debug)]
pub struct NoContent;

impl Responder for NoContent {
  type Body = BoxBody;

  fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
    HttpResponse::build(StatusCode::NO_CONTENT)
      .content_type("application/json")
      .finish()
  }
}

impl ApiComponent for NoContent {
  fn child_schemas(_: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    vec![]
  }

  fn schema(_: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    None
  }

  fn responses(_: OpenApiVersion, _content_type: Option<String>) -> Option<Responses> {
    let status = StatusCode::NO_CONTENT;
    Some(Responses {
      responses: BTreeMap::from_iter(vec![(
        status.as_str().to_string(),
        ReferenceOr::Object(Response::default()),
      )]),
      ..Default::default()
    })
  }
}

/// Empty struct to represent a 202 with a body
pub struct AcceptedJson<T: Serialize + ApiComponent>(pub T);

impl<T> Responder for AcceptedJson<T>
where
  T: Serialize + ApiComponent,
{
  type Body = BoxBody;

  fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
    let status = StatusCode::ACCEPTED;
    let body = match serde_json::to_string(&self.0) {
      Ok(body) => body,
      Err(e) => return e.error_response(),
    };

    HttpResponse::build(status).content_type("application/json").body(body)
  }
}

impl<T> ApiComponent for AcceptedJson<T>
where
  T: Serialize + ApiComponent,
{
  fn child_schemas(oas_version: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn raw_schema(oas_version: OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
    T::raw_schema(oas_version)
  }

  fn schema(oas_version: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version)
  }

  fn request_body(_: OpenApiVersion) -> Option<RequestBody> {
    None
  }

  fn responses(oas_version: OpenApiVersion, _content_type: Option<String>) -> Option<Responses> {
    let status = StatusCode::ACCEPTED;
    response_from_schema(oas_version, status.as_str(), Self::schema(oas_version))
      .or_else(|| response_from_raw_schema(oas_version, status.as_str(), Self::raw_schema(oas_version)))
  }
}

/// Empty struct to represent a 201 with a body
pub struct CreatedJson<T: Serialize + ApiComponent>(pub T);

impl<T> Responder for CreatedJson<T>
where
  T: Serialize + ApiComponent,
{
  type Body = BoxBody;

  fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
    let status = StatusCode::CREATED;
    let body = match serde_json::to_string(&self.0) {
      Ok(body) => body,
      Err(e) => return e.error_response(),
    };

    HttpResponse::build(status).content_type("application/json").body(body)
  }
}

impl<T> ApiComponent for CreatedJson<T>
where
  T: Serialize + ApiComponent,
{
  fn child_schemas(oas_version: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn raw_schema(oas_version: OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
    T::raw_schema(oas_version)
  }

  fn schema(oas_version: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version)
  }

  fn responses(oas_version: OpenApiVersion, _content_type: Option<String>) -> Option<Responses> {
    let status = StatusCode::CREATED;
    response_from_schema(oas_version, status.as_str(), Self::schema(oas_version))
      .or_else(|| response_from_raw_schema(oas_version, status.as_str(), Self::raw_schema(oas_version)))
  }
}

#[cfg(test)]
mod test {
  #![allow(clippy::expect_used)]

  use serde::Serialize;

  use apistos_core::ApiComponent;
  use apistos_gen::ApiComponent;
  use apistos_models::paths::Response;
  use apistos_models::reference_or::ReferenceOr;
  use apistos_models::OpenApiVersion;
  use schemars::JsonSchema;

  use crate as apistos;
  use crate::actix::{AcceptedJson, CreatedJson, NoContent};

  #[test]
  fn no_content_generate_valid_response_oas_3_0() {
    let responses = <NoContent as ApiComponent>::responses(OpenApiVersion::OAS3_0, None);
    assert!(responses.is_some());

    let responses = responses.expect("missing responses");
    let no_content_response = responses.responses.get("204");
    assert!(no_content_response.is_some());

    let no_content_response = no_content_response.expect("missing responses").clone();
    assert!(matches!(no_content_response, ReferenceOr::Object(obj) if obj == Response::default()));
  }

  #[test]
  fn no_content_generate_valid_response_oas_3_1() {
    let responses = <NoContent as ApiComponent>::responses(OpenApiVersion::OAS3_1, None);
    assert!(responses.is_some());

    let responses = responses.expect("missing responses");
    let no_content_response = responses.responses.get("204");
    assert!(no_content_response.is_some());

    let no_content_response = no_content_response.expect("missing responses").clone();
    assert!(matches!(no_content_response, ReferenceOr::Object(obj) if obj == Response::default()));
  }

  #[test]
  fn accepted_json_generate_valid_response_oas_3_0() {
    #[derive(Serialize, ApiComponent, JsonSchema)]
    struct Test {
      test: String,
    }

    let responses = <AcceptedJson<Test> as ApiComponent>::responses(OpenApiVersion::OAS3_0, None);
    assert!(responses.is_some());

    let responses = responses.expect("missing responses");
    let accepted_json_response = responses.responses.get("202");
    assert!(accepted_json_response.is_some());
  }

  #[test]
  fn accepted_json_generate_valid_response_oas_3_1() {
    #[derive(Serialize, ApiComponent, JsonSchema)]
    struct Test {
      test: String,
    }

    let responses = <AcceptedJson<Test> as ApiComponent>::responses(OpenApiVersion::OAS3_1, None);
    assert!(responses.is_some());

    let responses = responses.expect("missing responses");
    let accepted_json_response = responses.responses.get("202");
    assert!(accepted_json_response.is_some());
  }

  #[test]
  fn created_json_generate_valid_response_oas_3_0() {
    #[derive(Serialize, ApiComponent, JsonSchema)]
    struct Test {
      test: String,
    }

    let responses = <CreatedJson<Test> as ApiComponent>::responses(OpenApiVersion::OAS3_0, None);
    assert!(responses.is_some());

    let responses = responses.expect("missing responses");
    let accepted_json_response = responses.responses.get("201");
    assert!(accepted_json_response.is_some());
  }

  #[test]
  fn created_json_generate_valid_response_oas_3_1() {
    #[derive(Serialize, ApiComponent, JsonSchema)]
    struct Test {
      test: String,
    }

    let responses = <CreatedJson<Test> as ApiComponent>::responses(OpenApiVersion::OAS3_1, None);
    assert!(responses.is_some());

    let responses = responses.expect("missing responses");
    let accepted_json_response = responses.responses.get("201");
    assert!(accepted_json_response.is_some());
  }
}
