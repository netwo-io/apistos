//! This crate allow to expose the generated openapi specification through [RapiDoc](https://rapidocweb.com/).

use actix_web::dev::{AppService, HttpServiceFactory};
use actix_web::guard::Get;
use actix_web::web::Data;
use actix_web::{HttpResponse, Resource, Responder};

const RAPIDOC_DEFAULT: &str = include_str!("../assets/index.html");

/// Config for exposing the openapi specification through swagger UI
pub struct RapidocConfig {
  html: String,
  path: String,
}

impl RapidocConfig {
  /// Create a new [`RapidocConfig`] with the `path` on which to expose the rapidoc ui.
  pub fn new(path: String) -> Self {
    Self {
      html: RAPIDOC_DEFAULT.to_string(),
      path,
    }
  }

  /// Allow to override the default rapidoc html file path. The new file should contain the
  /// `$specUrl` variable which refers the openapi specification url
  pub fn with_html(mut self, html_path: String) -> Self {
    self.html = html_path;
    self
  }
}

#[doc(hidden)]
pub struct Rapidoc {
  html: String,
  path: String,
  openapi_path: String,
}

impl Rapidoc {
  pub fn new(config: RapidocConfig, openapi_path: &str) -> Self {
    Self {
      html: config.html,
      path: config.path,
      openapi_path: openapi_path.to_string(),
    }
  }

  fn to_html(&self) -> String {
    self.html.replace("$specUrl", &self.openapi_path)
  }
}

impl HttpServiceFactory for Rapidoc {
  fn register(self, config: &mut AppService) {
    async fn rapidoc_handler(rapidoc: Data<String>) -> impl Responder {
      HttpResponse::Ok().content_type("text/html").body(rapidoc.to_string())
    }

    let html = self.to_html();
    Resource::new::<&str>(&self.path)
      .guard(Get())
      .app_data(Data::new(html))
      .to(rapidoc_handler)
      .register(config);
  }
}
