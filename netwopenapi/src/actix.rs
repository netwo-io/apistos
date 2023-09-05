use crate::path_item_definition::PathItemDefinition;
use crate::ApiComponent;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use pin_project::pin_project;
use serde::Serialize;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use utoipa::openapi::path::Operation;
use utoipa::openapi::request_body::RequestBody;
use utoipa::openapi::{Components, ContentBuilder, Ref, RefOr, ResponseBuilder, Responses, ResponsesBuilder, Schema};

#[pin_project]
pub struct ResponseWrapper<R, P> {
  #[pin]
  pub inner: R,
  pub path_item: P,
}

impl<R: Responder, P> Responder for ResponseWrapper<R, P> {
  type Body = R::Body;

  fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
    self.inner.respond_to(req)
  }
}

impl<F, R, P> Future for ResponseWrapper<F, P>
where
  F: Future<Output = R>,
  R: Responder,
  P: PathItemDefinition,
{
  type Output = R;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    self.as_mut().project().inner.poll(cx)
  }
}

impl<F, R, P> PathItemDefinition for ResponseWrapper<F, P>
where
  F: Future<Output = R>,
  R: Responder,
  P: PathItemDefinition,
{
  fn is_visible() -> bool {
    P::is_visible()
  }

  fn operation() -> Operation {
    P::operation()
  }

  fn components() -> Vec<Components> {
    P::components()
  }
}

pub struct ResponderWrapper<T>(pub T);

impl<T: Responder> ApiComponent for ResponderWrapper<T> {
  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    vec![]
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    None
  }
}

impl<T: Responder> PathItemDefinition for ResponderWrapper<T> {}

impl<T: Responder> Responder for ResponderWrapper<T> {
  type Body = T::Body;

  fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
    self.0.respond_to(req)
  }
}

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
  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    vec![]
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    None
  }

  fn responses() -> Option<Responses> {
    let status = StatusCode::NO_CONTENT;
    Some(
      ResponsesBuilder::new()
        .response(
          status.as_str(),
          ResponseBuilder::new()
            .content(
              "application/json",
              ContentBuilder::new().schema(Schema::default()).build(),
            )
            .build(),
        )
        .build(),
    )
  }
}

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
  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    T::schema()
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn responses() -> Option<Responses> {
    let status = StatusCode::ACCEPTED;
    Self::schema().map(|(name, schema)| {
      let _ref = match schema {
        RefOr::Ref(r) => r,
        RefOr::T(_) => Ref::from_schema_name(name),
      };
      ResponsesBuilder::new()
        .response(
          status.as_str(),
          ResponseBuilder::new()
            .content("application/json", ContentBuilder::new().schema(_ref).build())
            .build(),
        )
        .build()
    })
  }
}

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
  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    T::schema()
  }

  fn responses() -> Option<Responses> {
    let status = StatusCode::CREATED;
    Self::schema().map(|(name, schema)| {
      let _ref = match schema {
        RefOr::Ref(r) => r,
        RefOr::T(_) => Ref::from_schema_name(name),
      };
      ResponsesBuilder::new()
        .response(
          status.as_str(),
          ResponseBuilder::new()
            .content("application/json", ContentBuilder::new().schema(_ref).build())
            .build(),
        )
        .build()
    })
  }
}
