//! This crate allow to expose the generated openapi specification through [SwaggerUI](https://swagger.io/tools/swagger-ui/).

use actix_web::dev::{AppService, HttpServiceFactory};
use actix_web::guard::Get;
use actix_web::web::Data;
use actix_web::{HttpResponse, Resource, Responder};

const SWAGGER_UI_DEFAULT: &str = include_str!("../assets/index.html");

/// Config for exposing the openapi specification through swagger UI
pub struct SwaggerUIConfig {
  html: String,
  path: String,
}

impl SwaggerUIConfig {
  /// Create a new [`SwaggerUIConfig`] with the `path` on which to expose the swagger ui.
  pub fn new<T: ToString>(path: &T) -> Self {
    Self {
      html: SWAGGER_UI_DEFAULT.to_string(),
      path: path.to_string(),
    }
  }

  /// Allow to override the default swagger UI html file path. The new file should contain the
  /// `$specUrl` variable which refers the openapi specification url
  pub fn with_html(mut self, html_path: String) -> Self {
    self.html = html_path;
    self
  }
}

#[doc(hidden)]
pub struct SwaggerUi {
  html: String,
  path: String,
  openapi_path: String,
}

impl SwaggerUi {
  pub fn new(config: SwaggerUIConfig, openapi_path: &str) -> Self {
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

impl HttpServiceFactory for SwaggerUi {
  fn register(self, config: &mut AppService) {
    async fn swagger_handler(swagger_ui: Data<String>) -> impl Responder {
      HttpResponse::Ok()
        .content_type("text/html")
        .body(swagger_ui.to_string())
    }

    let html = self.to_html();
    Resource::new::<&str>(&self.path)
      .guard(Get())
      .app_data(Data::new(html))
      .to(swagger_handler)
      .register(config);
  }
}
