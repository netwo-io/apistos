use actix_web::{Error, HttpResponse};
use apistos_models::OpenApi;
use std::future::{Ready, ready};

#[derive(Clone)]
pub(crate) struct OASHandler(OpenApi);

impl OASHandler {
  pub(crate) fn new(open_api: OpenApi) -> Self {
    Self(open_api)
  }
}

impl actix_web::Handler<()> for OASHandler {
  type Output = Result<HttpResponse, Error>;
  type Future = Ready<Self::Output>;

  fn call(&self, _: ()) -> Self::Future {
    ready(Ok(HttpResponse::Ok().json(self.0.clone())))
  }
}
