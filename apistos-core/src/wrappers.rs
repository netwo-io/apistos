use crate::{ApiComponent, PathItemDefinition};
use actix_web::{HttpRequest, HttpResponse, Responder};
use apistos_models::Schema;
use apistos_models::components::Components;
use apistos_models::paths::Operation;
use apistos_models::reference_or::ReferenceOr;
use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

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
  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    vec![]
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
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
