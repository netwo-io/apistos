use crate::internal::actix::route::{Route, RouteWrapper};
use crate::internal::actix::service_config::ServiceConfig;
use crate::internal::actix::utils::OperationUpdater;
use crate::internal::definition_holder::DefinitionHolder;
use actix_service::{ServiceFactory, Transform};
use actix_web::body::MessageBody;
use actix_web::dev::{AppService, HttpServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::guard::Guard;
use actix_web::Error;
use indexmap::IndexMap;
use netwopenapi_models::components::Components;
use netwopenapi_models::paths::PathItem;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::future::Future;

pub struct Scope<S = actix_web::Scope> {
  pub(crate) item_map: BTreeMap<String, PathItem>,
  pub(crate) components: Vec<Components>,
  path: String,
  inner: Option<S>,
}

impl Scope {
  /// Wrapper for [`actix_web::Scope::new`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.new)
  pub fn new(path: &str) -> Self {
    Scope {
      item_map: Default::default(),
      components: Default::default(),
      path: path.into(),
      inner: Some(actix_web::Scope::new(path)),
    }
  }
}

impl<T, B> HttpServiceFactory for Scope<actix_web::Scope<T>>
where
  T:
    ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse<B>, Error = Error, InitError = ()> + 'static,
  B: MessageBody + 'static,
{
  fn register(self, config: &mut AppService) {
    if let Some(s) = self.inner {
      s.register(config);
    }
  }
}

impl<T> Scope<actix_web::Scope<T>>
where
  T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
  /// Proxy for [`actix_web::Scope::guard`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.guard).
  ///
  /// **NOTE:** This doesn't affect spec generation.
  pub fn guard<G: Guard + 'static>(mut self, guard: G) -> Self {
    self.inner = self.inner.take().map(|s| s.guard(guard));
    self
  }

  /// Proxy for [`actix_web::Scope::app_data`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.data).
  ///
  /// **NOTE:** This doesn't affect spec generation.
  pub fn app_data<U: 'static>(mut self, data: U) -> Self {
    self.inner = self.inner.take().map(|s| s.app_data(data));
    self
  }

  /// Wrapper for [`actix_web::Scope::configure`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.configure).
  pub fn configure<F>(mut self, f: F) -> Self
  where
    F: FnOnce(&mut ServiceConfig),
  {
    self.inner = self.inner.take().map(|s| {
      s.configure(|c| {
        let mut cfg: ServiceConfig = ServiceConfig::from(c);
        f(&mut cfg);
        self.update_from_def_holder(&mut cfg);
      })
    });
    self
  }

  /// Wrapper for [`actix_web::Scope::service`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.service).
  pub fn service<F>(mut self, mut factory: F) -> Self
  where
    F: DefinitionHolder + HttpServiceFactory + 'static,
  {
    self.update_from_def_holder(&mut factory);
    self.inner = self.inner.take().map(|s| s.service(factory));
    self
  }

  /// Wrapper for [`actix_web::Scope::route`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.route).
  pub fn route(mut self, path: &str, route: Route) -> Self {
    let mut w = RouteWrapper::new(&self.path, route);
    self.update_from_def_holder(&mut w);
    self.inner = self.inner.take().map(|s| s.route(path, w.inner));
    self
  }

  /// Proxy for [`actix_web::web::Scope::default_service`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.default_service).
  ///
  /// **NOTE:** This doesn't affect spec generation.
  pub fn default_service<F, U>(mut self, f: F) -> Self
  where
    F: actix_service::IntoServiceFactory<U, ServiceRequest>,
    U: ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = Error, InitError = ()> + 'static,
    U::InitError: Debug,
  {
    self.inner = self.inner.map(|s| s.default_service(f));
    self
  }

  /// Proxy for [`actix_web::web::Scope::wrap`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.wrap).
  ///
  /// **NOTE:** This doesn't affect spec generation.
  pub fn wrap<M, B>(
    mut self,
    mw: M,
  ) -> Scope<
    actix_web::Scope<
      impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse<B>, Error = Error, InitError = ()>,
    >,
  >
  where
    M: Transform<T::Service, ServiceRequest, Response = ServiceResponse<B>, Error = Error, InitError = ()> + 'static,
    B: MessageBody,
  {
    Scope {
      item_map: self.item_map,
      components: self.components,
      path: self.path,
      inner: self.inner.take().map(|s| s.wrap(mw)),
    }
  }

  /// Proxy for [`actix_web::web::Scope::wrap_fn`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.wrap_fn).
  ///
  /// **NOTE:** This doesn't affect spec generation.
  pub fn wrap_fn<F, R>(
    mut self,
    mw: F,
  ) -> Scope<
    actix_web::Scope<
      impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = Error, InitError = ()>,
    >,
  >
  where
    F: Fn(ServiceRequest, &T::Service) -> R + Clone + 'static,
    R: Future<Output = Result<ServiceResponse, Error>>,
  {
    Scope {
      item_map: self.item_map,
      components: self.components,
      path: self.path,
      inner: self.inner.take().map(|s| s.wrap_fn(mw)),
    }
  }

  fn update_from_def_holder<D: DefinitionHolder>(&mut self, dh: &mut D) {
    self.components.extend(dh.components());
    let mut item_map = IndexMap::new();
    dh.update_path_items(&mut item_map);
    for (path, mut path_item) in item_map {
      let p = [self.path.clone(), path]
        .iter()
        .map(|p| p.trim_matches('/'))
        .collect::<Vec<&str>>()
        .join("/");

      for operation in path_item.operations.values_mut() {
        operation.update_path_parameter_name_from_path(&p);
      }

      self.item_map.insert(p, path_item);
    }
  }
}

/// Wrapper for [`actix_web::web::scope`](https://docs.rs/actix-web/*/actix_web/web/fn.scope.html).
pub fn scope(path: &str) -> Scope {
  Scope::new(path)
}
