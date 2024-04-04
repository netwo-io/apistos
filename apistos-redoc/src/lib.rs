//! This crate allow to expose the generated openapi specification through [Redoc](https://redocly.com/redoc/).

use actix_web::dev::{AppService, HttpServiceFactory};
use actix_web::guard::Get;
use actix_web::web::Data;
use actix_web::{HttpResponse, Resource, Responder};

const REDOC_DEFAULT: &str = include_str!("../assets/index.html");

/// Config for exposing the openapi specification through swagger UI
pub struct RedocConfig {
  html: String,
  path: String,
}

impl RedocConfig {
  /// Create a new [`RedocConfig`] with the `path` on which to expose the redoc ui.
  pub fn new<T: ToString>(path: &T) -> Self {
    Self {
      html: REDOC_DEFAULT.to_string(),
      path: path.to_string(),
    }
  }

  /// Allow to override the default redoc html file path. The new file should contain the
  /// `$specUrl` variable which refers the openapi specification url
  pub fn with_html(mut self, html_path: String) -> Self {
    self.html = html_path;
    self
  }
}

#[doc(hidden)]
pub struct Redoc {
  html: String,
  path: String,
  openapi_path: String,
}

impl Redoc {
  pub fn new(config: RedocConfig, openapi_path: &str) -> Self {
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

impl HttpServiceFactory for Redoc {
  fn register(self, config: &mut AppService) {
    async fn redoc_handler(redoc: Data<String>) -> impl Responder {
      HttpResponse::Ok().content_type("text/html").body(redoc.to_string())
    }

    let html = self.to_html();
    Resource::new::<&str>(&self.path)
      .guard(Get())
      .app_data(Data::new(html))
      .to(redoc_handler)
      .register(config);
  }
}
