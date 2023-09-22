use crate::internal::actix::handler::OASHandler;
use crate::internal::actix::route::{Route, RouteWrapper};
use crate::internal::definition_holder::DefinitionHolder;
use crate::spec::Spec;
use crate::web::ServiceConfig;
use actix_service::{IntoServiceFactory, ServiceFactory, Transform};
use actix_web::body::MessageBody;
use actix_web::dev::{HttpServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::{get, resource};
use actix_web::Error;
use indexmap::IndexMap;
use netwopenapi_models::OpenApi;
use std::future::Future;
use std::sync::{Arc, RwLock};
use std::{fmt, mem};

pub trait OpenApiWrapper<T> {
  type Wrapper;

  fn document(self, spec: Spec) -> Self::Wrapper;
}

/// Wrapper for [actix_web::App] with openapi specification
pub struct App<T> {
  open_api_spec: Arc<RwLock<OpenApi>>,
  inner: Option<actix_web::App<T>>, //an option juste to be able to replace it with a default in memory
  default_tags: Vec<String>,
}

impl<T> OpenApiWrapper<T> for actix_web::App<T> {
  type Wrapper = App<T>;

  fn document(self, spec: Spec) -> Self::Wrapper {
    let mut open_api_spec = OpenApi::default();
    open_api_spec.info = spec.info;
    if !spec.tags.is_empty() {
      open_api_spec.tags = spec.tags;
    }
    open_api_spec.external_docs = spec.external_docs;
    if !spec.servers.is_empty() {
      open_api_spec.servers = spec.servers;
    }
    App {
      open_api_spec: Arc::new(RwLock::new(open_api_spec)),
      inner: Some(self),
      default_tags: spec.default_tags,
    }
  }
}

impl<T> App<T>
where
  T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
  /// Drop in for [`actix_web::App::app_data`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.app_data)
  pub fn app_data<U: 'static>(mut self, ext: U) -> Self {
    self.inner = self.inner.take().map(|app| app.app_data(ext));
    self
  }

  /// Drop in for [`actix_web::App::data_factory`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.data_factory)
  pub fn data_factory<F, Out, D, E>(mut self, data: F) -> Self
  where
    F: Fn() -> Out + 'static,
    Out: Future<Output = Result<D, E>> + 'static,
    D: 'static,
    E: fmt::Debug,
  {
    self.inner = self.inner.take().map(|app| app.data_factory(data));
    self
  }

  /// Drop in for [`actix_web::App::configure`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.configure)
  pub fn configure<F>(mut self, f: F) -> Self
  where
    F: FnOnce(&mut ServiceConfig),
  {
    self.inner = self.inner.take().map(|app| {
      app.configure(|c| {
        let mut cfg = ServiceConfig::from(c);
        f(&mut cfg);
        self.update_from_def_holder(&mut cfg);
      })
    });
    self
  }

  /// Drop in for [`actix_web::App::route`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.route)
  pub fn route(mut self, path: &str, route: Route) -> Self {
    let mut w = RouteWrapper::new(path, route);
    self.update_from_def_holder(&mut w);
    self.inner = self.inner.take().map(|app| app.route(path, w.inner));
    self
  }

  /// Drop in for [`actix_web::App::service`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.service)
  pub fn service<F>(mut self, mut factory: F) -> Self
  where
    F: DefinitionHolder + HttpServiceFactory + 'static,
  {
    self.update_from_def_holder(&mut factory);
    self.inner = self.inner.take().map(|app| app.service(factory));
    self
  }

  /// Drop in for [`actix_web::App::default_service`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.default_service)
  pub fn default_service<F, U>(mut self, svc: F) -> Self
  where
    F: IntoServiceFactory<U, ServiceRequest>,
    U: ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = Error> + 'static,
    U::InitError: fmt::Debug,
  {
    self.inner = self.inner.take().map(|app| app.default_service(svc));
    self
  }

  /// Drop in for [`actix_web::App::external_resource`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.external_resource)
  pub fn external_resource<N, U>(mut self, name: N, url: U) -> Self
  where
    N: AsRef<str>,
    U: AsRef<str>,
  {
    self.inner = self.inner.take().map(|app| app.external_resource(name, url));
    self
  }

  /// Drop in for [`actix_web::App::wrap`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.wrap)
  pub fn wrap<M, B>(
    mut self,
    mw: M,
  ) -> App<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse<B>, Error = Error, InitError = ()>>
  where
    M: Transform<T::Service, ServiceRequest, Response = ServiceResponse<B>, Error = Error, InitError = ()> + 'static,
    B: MessageBody,
  {
    App {
      open_api_spec: self.open_api_spec,
      inner: self.inner.take().map(|app| app.wrap(mw)),
      default_tags: self.default_tags,
    }
  }

  /// Drop in for [`actix_web::App::wrap_fn`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.wrap_fn)
  pub fn wrap_fn<F, R, B>(
    mut self,
    mw: F,
  ) -> App<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse<B>, Error = Error, InitError = ()>>
  where
    F: Fn(ServiceRequest, &T::Service) -> R + Clone + 'static,
    R: Future<Output = Result<ServiceResponse<B>, Error>>,
    B: MessageBody,
  {
    App {
      open_api_spec: self.open_api_spec,
      inner: self.inner.take().map(|app| app.wrap_fn(mw)),
      default_tags: self.default_tags,
    }
  }

  /// Add a new resource at **openapi_path** to expose the generated openapi schema and return an [actix_web::App]
  pub fn build(self, openapi_path: &str) -> actix_web::App<T> {
    let open_api_spec = self.open_api_spec.read().unwrap().clone();
    self
      .inner
      .expect("Missing app")
      .service(resource(openapi_path).route(get().to(OASHandler::new(open_api_spec))))
  }

  /// Updates the underlying spec with definitions and operations from the given definition holder.
  fn update_from_def_holder<D: DefinitionHolder>(&mut self, definition_holder: &mut D) {
    let mut open_api_spec = self.open_api_spec.write().unwrap();
    let components = definition_holder.components().into_iter().reduce(|mut acc, component| {
      acc.schemas.extend(component.schemas.into_iter());
      acc.responses.extend(component.responses.into_iter());
      acc.security_schemes.extend(component.security_schemes.into_iter());
      acc
    });
    definition_holder.update_path_items(&mut open_api_spec.paths.paths);
    let mut paths = IndexMap::new();
    for (path, item) in mem::take(&mut open_api_spec.paths.paths) {
      let path = if path.starts_with('/') {
        path
      } else {
        "/".to_owned() + &path
      };
      paths.insert(path, item);
    }
    open_api_spec.paths.paths = paths;
    open_api_spec.components = components;

    if !self.default_tags.is_empty() {
      for pi in open_api_spec.paths.paths.values_mut() {
        for op in pi.operations.values_mut() {
          op.tags.append(&mut self.default_tags.clone());
        }
      }
    }
  }
}
