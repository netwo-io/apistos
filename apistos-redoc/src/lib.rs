//! This crate allow to expose the generated openapi specification through [Redoc](https://redocly.com/redoc/).

use apistos_plugins::ui::{UIPlugin, UIPluginConfig};

const REDOC_DEFAULT: &str = include_str!("../assets/index.html");

/// Config for exposing the openapi specification through `Redoc`
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

impl UIPluginConfig for RedocConfig {
  fn build(self: Box<Self>, openapi_path: &str) -> Box<dyn UIPlugin> {
    Box::new(Redoc::new(*self, openapi_path))
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
}

impl UIPlugin for Redoc {
  fn path(&self) -> String {
    self.path.clone()
  }

  fn to_html(&self) -> String {
    self.html.replace("$specUrl", &self.openapi_path)
  }
}
