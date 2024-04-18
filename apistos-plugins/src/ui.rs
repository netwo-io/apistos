use actix_web::dev::{AppService, HttpServiceFactory};
use actix_web::guard::Get;
use actix_web::web::Data;
use actix_web::{HttpResponse, Resource, Responder};

/// Trait to implement on a UI config which will serve a an argument of `BuildConfig::with` method
pub trait UIPluginConfig {
  /// Build a `UIPlugin` given a `UIPluginConfig` and an `openapi_path`
  fn build(self: Box<Self>, openapi_path: &str) -> Box<dyn UIPlugin>;
}

/// Trait to implement to expose a UI for the generated openapi specification
pub trait UIPlugin {
  /// Getter for the HTML file path
  fn path(&self) -> String;

  /// Transform the given file (defined at `path`) to a valid HTML String. Useful for variable replacement for example.
  fn to_html(&self) -> String;
}

impl UIPlugin for Box<dyn UIPlugin> {
  fn path(&self) -> String {
    self.as_ref().path()
  }

  fn to_html(&self) -> String {
    self.as_ref().to_html()
  }
}

pub struct UIPluginWrapper(Box<dyn UIPlugin>);

impl From<Box<dyn UIPlugin>> for UIPluginWrapper {
  fn from(value: Box<dyn UIPlugin>) -> Self {
    Self(value)
  }
}

impl HttpServiceFactory for UIPluginWrapper {
  fn register(self, config: &mut AppService) {
    async fn handler(html: Data<String>) -> impl Responder {
      HttpResponse::Ok().content_type("text/html").body(html.to_string())
    }

    let html = self.0.to_html();
    Resource::new::<&str>(&self.0.path())
      .guard(Get())
      .app_data(Data::new(html))
      .to(handler)
      .register(config);
  }
}
