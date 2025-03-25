use crate::internal::actix::route::{Route, RouteWrapper};
use crate::internal::definition_holder::DefinitionHolder;
use actix_web::dev::HttpServiceFactory;
use apistos_models::components::Components;
use apistos_models::paths::PathItem;
use indexmap::IndexMap;

pub struct ServiceConfig<'a> {
  pub(crate) item_map: IndexMap<String, PathItem>,
  pub(crate) components: Vec<Components>,
  inner: &'a mut actix_web::web::ServiceConfig,
}

impl<'a> From<&'a mut actix_web::web::ServiceConfig> for ServiceConfig<'a> {
  fn from(cfg: &'a mut actix_web::web::ServiceConfig) -> Self {
    ServiceConfig {
      item_map: Default::default(),
      components: Default::default(),
      inner: cfg,
    }
  }
}

impl ServiceConfig<'_> {
  /// Wrapper for [`actix_web::web::ServiceConfig::route`](https://docs.rs/actix-web/*/actix_web/web/struct.ServiceConfig.html#method.route).
  pub fn route(&mut self, path: &str, route: Route) -> &mut Self {
    let mut w = RouteWrapper::new(path, route);
    w.update_path_items(&mut self.item_map);
    self.components.extend(w.components());
    self.inner.route(path, w.inner);
    self
  }

  /// Wrapper for [`actix_web::web::ServiceConfig::service`](https://docs.rs/actix-web/*/actix_web/web/struct.ServiceConfig.html#method.configure).
  pub fn configure<F>(&mut self, f: F) -> &mut Self
  where
    F: FnOnce(&mut ServiceConfig),
  {
    f(self);
    self
  }

  /// Wrapper for [`actix_web::web::ServiceConfig::service`](https://docs.rs/actix-web/*/actix_web/web/struct.ServiceConfig.html#method.service).
  pub fn service<F>(&mut self, mut factory: F) -> &mut Self
  where
    F: DefinitionHolder + HttpServiceFactory + 'static,
  {
    factory.update_path_items(&mut self.item_map);
    self.components.extend(factory.components());
    self.inner.service(factory);
    self
  }

  /// Proxy for [`actix_web::web::ServiceConfig::external_resource`](https://docs.rs/actix-web/*/actix_web/web/struct.ServiceConfig.html#method.external_resource).
  ///
  /// **NOTE:** This doesn't affect spec generation.
  pub fn external_resource<N, U>(&mut self, name: N, url: U) -> &mut Self
  where
    N: AsRef<str>,
    U: AsRef<str>,
  {
    self.inner.external_resource(name, url);
    self
  }

  /// Proxy for [`actix_web::web::ServiceConfig::app_data`](https://docs.rs/actix-web/4.0.1/actix_web/web/struct.ServiceConfig.html#method.app_data).
  ///
  /// **NOTE:** This doesn't affect spec generation.
  pub fn app_data<U: 'static>(&mut self, data: U) -> &mut Self {
    self.inner.app_data(data);
    self
  }
}
