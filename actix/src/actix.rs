use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use actix_web::{HttpRequest, HttpResponse, Responder};
use pin_project::pin_project;
use utoipa::openapi::{Components, PathItem};
use crate::path_item_definition::PathItemDefinition;

#[pin_project]
pub struct ResponderWrapper<R, P> {
  #[pin]
  pub inner: R,
  pub path_item: P
}

impl<R: Responder, P> Responder for ResponderWrapper<R, P> {
  type Body = R::Body;

  fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
    self.inner.respond_to(req)
  }
}

impl<F, R, P> Future for ResponderWrapper<F, P>
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

impl<F, R, P> PathItemDefinition for ResponderWrapper<F, P>
  where
    F: Future<Output = R>,
    R: Responder,
    P: PathItemDefinition,
{
  fn is_visible() -> bool {
    P::is_visible()
  }

  fn path_item(default_tag: Option<&str>) -> PathItem {
    P::path_item(default_tag)
  }

  fn components() -> BTreeMap<String, Components> {
    P::components()
  }
}
