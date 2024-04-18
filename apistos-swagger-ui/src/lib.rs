//! This crate allow to expose the generated openapi specification through [SwaggerUI](https://swagger.io/tools/swagger-ui/).

use apistos_plugins::ui::{UIPlugin, UIPluginConfig};

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

impl UIPluginConfig for SwaggerUIConfig {
  fn build(self: Box<Self>, openapi_path: &str) -> Box<dyn UIPlugin> {
    Box::new(SwaggerUi::new(*self, openapi_path))
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
}

impl UIPlugin for SwaggerUi {
  fn path(&self) -> String {
    self.path.clone()
  }

  fn to_html(&self) -> String {
    self.html.replace("$specUrl", &self.openapi_path)
  }
}
