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
use std::collections::BTreeMap;
use std::future::Future;
use std::sync::{Arc, RwLock};
use std::{fmt, mem};
use utoipa::openapi::OpenApi;
#[cfg(feature = "rapidoc")]
use utoipa_rapidoc::RapiDoc;
#[cfg(feature = "redoc")]
use utoipa_redoc::{Redoc, Servable};
#[cfg(feature = "swagger-ui")]
use utoipa_swagger_ui::SwaggerUi;

pub trait OpenApiWrapper<T> {
  type Wrapper;

  fn document(self, spec: Spec) -> Self::Wrapper;
}

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
      open_api_spec.tags = Some(spec.tags);
    }
    open_api_spec.external_docs = spec.external_docs;
    if !spec.servers.is_empty() {
      open_api_spec.servers = Some(spec.servers);
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
  pub fn app_data<U: 'static>(mut self, ext: U) -> Self {
    self.inner = self.inner.take().map(|app| app.app_data(ext));
    self
  }

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

  pub fn route(mut self, path: &str, route: Route) -> Self {
    let mut w = RouteWrapper::new(path, route);
    self.update_from_def_holder(&mut w);
    self.inner = self.inner.take().map(|app| app.route(path, w.inner));
    self
  }

  pub fn service<F>(mut self, mut factory: F) -> Self
  where
    F: DefinitionHolder + HttpServiceFactory + 'static,
  {
    self.update_from_def_holder(&mut factory);
    self.inner = self.inner.take().map(|app| app.service(factory));
    self
  }

  pub fn default_service<F, U>(mut self, svc: F) -> Self
  where
    F: IntoServiceFactory<U, ServiceRequest>,
    U: ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = Error> + 'static,
    U::InitError: fmt::Debug,
  {
    self.inner = self.inner.take().map(|app| app.default_service(svc));
    self
  }

  pub fn external_resource<N, U>(mut self, name: N, url: U) -> Self
  where
    N: AsRef<str>,
    U: AsRef<str>,
  {
    self.inner = self.inner.take().map(|app| app.external_resource(name, url));
    self
  }

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

  pub fn build(self, openapi_path: &str) -> actix_web::App<T> {
    let open_api_spec = self.open_api_spec.read().unwrap().clone();
    self
      .inner
      .expect("Missing app")
      .service(resource(openapi_path).route(get().to(OASHandler::new(open_api_spec))))
  }

  #[cfg(feature = "rapidoc")]
  pub fn build_with_rapidoc(self, ui_path: &'static str, openapi_path: &'static str) -> actix_web::App<T> {
    let open_api_spec = self.open_api_spec.read().unwrap().clone();
    self
      .inner
      .expect("Missing app")
      .service(RapiDoc::with_openapi(openapi_path, open_api_spec).path(ui_path))
  }

  #[cfg(feature = "redoc")]
  pub fn build_with_redoc(self, openapi_path: &'static str) -> actix_web::App<T> {
    let open_api_spec = self.open_api_spec.read().unwrap().clone();
    self
      .inner
      .expect("Missing app")
      .service(Redoc::with_url(openapi_path, open_api_spec))
  }

  #[cfg(feature = "swagger-ui")]
  pub fn build_with_swagger(self, ui_path: &'static str, openapi_path: &'static str) -> actix_web::App<T> {
    let open_api_spec = self.open_api_spec.read().unwrap().clone();
    self
      .inner
      .expect("Missing app")
      .service(SwaggerUi::new(format!("/{ui_path}/{{_:.*}}")).url(openapi_path, open_api_spec))
  }

  /// Updates the underlying spec with definitions and operations from the given factory.
  fn update_from_def_holder<D: DefinitionHolder>(&mut self, factory: &mut D) {
    let mut open_api_spec = self.open_api_spec.write().unwrap();
    let components = factory.components().into_iter().reduce(|mut acc, component| {
      acc.schemas.extend(component.schemas.into_iter());
      acc.responses.extend(component.responses.into_iter());
      acc.security_schemes.extend(component.security_schemes.into_iter());
      acc
    });
    factory.update_path_items(&mut open_api_spec.paths.paths);
    let mut paths = BTreeMap::new();
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
          match &mut op.tags {
            None => op.tags = Some(self.default_tags.clone()),
            Some(tags) => tags.append(&mut self.default_tags.clone()),
          }
        }
      }
    }
  }
}
