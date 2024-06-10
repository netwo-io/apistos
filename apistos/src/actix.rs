use crate::ApiComponent;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use apistos_models::paths::{MediaType, RequestBody, Response, Responses};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::{ApistosSchema, OpenApiVersion, VersionSpecificSchema};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Debug;

pub use apistos_core::{ResponderWrapper, ResponseWrapper};

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
    response_from_schema(oas_version, status, Self::schema(oas_version))
      .or_else(|| response_from_raw_schema(oas_version, status, Self::raw_schema(oas_version)))
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
    response_from_schema(oas_version, status, Self::schema(oas_version))
      .or_else(|| response_from_raw_schema(oas_version, status, Self::raw_schema(oas_version)))
  }
}

fn response_from_schema(
  oas_version: OpenApiVersion,
  status: StatusCode,
  schema: Option<(String, ReferenceOr<ApistosSchema>)>,
) -> Option<Responses> {
  schema.map(|(name, schema)| match schema {
    ReferenceOr::Reference { _ref } => Responses {
      responses: BTreeMap::from_iter(vec![(status.as_str().to_string(), ReferenceOr::Reference { _ref })]),
      ..Default::default()
    },
    ReferenceOr::Object(sch) => {
      let schema = match oas_version {
        OpenApiVersion::OAS3_0 => VersionSpecificSchema::OAS3_0(ReferenceOr::Reference {
          _ref: format!("#/components/schemas/{}", name),
        }),
        OpenApiVersion::OAS3_1 => VersionSpecificSchema::OAS3_1(sch),
      };
      let response = Response {
        content: BTreeMap::from_iter(vec![(
          "application/json".to_string(),
          MediaType {
            schema: Some(schema),
            ..Default::default()
          },
        )]),
        ..Default::default()
      };
      Responses {
        responses: BTreeMap::from_iter(vec![(status.as_str().to_string(), ReferenceOr::Object(response))]),
        ..Default::default()
      }
    }
  })
}

fn response_from_raw_schema(
  oas_version: OpenApiVersion,
  status: StatusCode,
  raw_schema: Option<ReferenceOr<ApistosSchema>>,
) -> Option<Responses> {
  raw_schema.map(|schema| match schema {
    ReferenceOr::Reference { _ref } => Responses {
      responses: BTreeMap::from_iter(vec![(status.as_str().to_string(), ReferenceOr::Reference { _ref })]),
      ..Default::default()
    },
    ReferenceOr::Object(sch) => {
      let schema = match oas_version {
        OpenApiVersion::OAS3_0 => VersionSpecificSchema::OAS3_0(sch.into()),
        OpenApiVersion::OAS3_1 => VersionSpecificSchema::OAS3_1(sch),
      };
      let response = Response {
        content: BTreeMap::from_iter(vec![(
          "application/json".to_string(),
          MediaType {
            schema: Some(schema),
            ..Default::default()
          },
        )]),
        ..Default::default()
      };
      Responses {
        responses: BTreeMap::from_iter(vec![(status.as_str().to_string(), ReferenceOr::Object(response))]),
        ..Default::default()
      }
    }
  })
}

#[cfg(test)]
mod test {
  #![allow(clippy::expect_used)]

  use crate as apistos;
  use crate::actix::{AcceptedJson, CreatedJson, NoContent};
  use apistos_core::ApiComponent;
  use apistos_gen::ApiComponent;
  use apistos_models::paths::Response;
  use apistos_models::reference_or::ReferenceOr;
  use apistos_models::OpenApiVersion;
  use schemars::JsonSchema;
  use serde::Serialize;

  #[test]
  fn no_content_generate_valid_response() {
    let responses = <NoContent as ApiComponent>::responses(OpenApiVersion::OAS3_0, None);
    assert!(responses.is_some());

    let responses = responses.expect("missing responses");
    let no_content_response = responses.responses.get("204");
    assert!(no_content_response.is_some());

    let no_content_response = no_content_response.expect("missing responses").clone();
    assert!(matches!(no_content_response, ReferenceOr::Object(obj) if obj == Response::default()));
  }

  #[test]
  fn accepted_json_generate_valid_response() {
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
  fn created_json_generate_valid_response() {
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
}
