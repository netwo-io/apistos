//! This crate allow to expose the generated openapi specification through [RapiDoc](https://rapidocweb.com/).

use apistos_plugins::ui::{UIPlugin, UIPluginConfig};

const RAPIDOC_DEFAULT: &str = include_str!("../assets/index.html");

/// Config for exposing the openapi specification through `RapiDoc`
pub struct RapidocConfig {
  html: String,
  path: String,
}

impl RapidocConfig {
  /// Create a new [`RapidocConfig`] with the `path` on which to expose the rapidoc ui.
  pub fn new<T: ToString>(path: &T) -> Self {
    Self {
      html: RAPIDOC_DEFAULT.to_string(),
      path: path.to_string(),
    }
  }

  /// Allow to override the default rapidoc html file path. The new file should contain the
  /// `$specUrl` variable which refers the openapi specification url
  pub fn with_html(mut self, html_path: String) -> Self {
    self.html = html_path;
    self
  }
}

impl UIPluginConfig for RapidocConfig {
  fn build(self: Box<Self>, openapi_path: &str) -> Box<dyn UIPlugin> {
    Box::new(Rapidoc::new(*self, openapi_path))
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
}

impl UIPlugin for Rapidoc {
  fn path(&self) -> String {
    self.path.clone()
  }

  fn to_html(&self) -> String {
    self.html.replace("$specUrl", &self.openapi_path)
  }
}
