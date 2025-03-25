use crate::internal::actix::METHODS;
use crate::internal::actix::route::{Route, RouteWrapper};
use crate::internal::actix::utils::OperationUpdater;
use actix_service::{ServiceFactory, Transform};
use actix_web::body::MessageBody;
use actix_web::dev::{AppService, HttpServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::guard::Guard;
use actix_web::{Error, FromRequest, Handler, Responder};
use apistos_core::PathItemDefinition;
use apistos_models::components::Components;
use apistos_models::paths::PathItem;
use std::fmt::Debug;
use std::future::Future;

pub struct Resource<R = actix_web::Resource> {
  pub(crate) path: String,
  pub(crate) item_definition: Option<PathItem>,
  pub(crate) components: Vec<Components>,
  tags: Vec<String>,
  inner: R,
}

impl Resource {
  /// Wrapper for [`actix_web::Resource::new`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.new).
  pub fn new(path: &str) -> Resource {
    Resource {
      path: path.to_owned(),
      item_definition: None,
      components: Default::default(),
      tags: Default::default(),
      inner: actix_web::Resource::new(path),
    }
  }

  /// Wrapper for [`actix_web::Resource::new`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.new) with a list of tag names for the given scope.
  /// Tags should exist in `Spec` otherwise documentation might be considered invalid by consumers.
  pub fn new_tagged<T: Into<String>>(path: &str, tags: Vec<T>) -> Resource {
    Resource {
      path: path.to_owned(),
      item_definition: None,
      components: Default::default(),
      tags: tags.into_iter().map(Into::into).collect(),
      inner: actix_web::Resource::new(path),
    }
  }
}

impl<T, B> HttpServiceFactory for Resource<actix_web::Resource<T>>
where
  T:
    ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse<B>, Error = Error, InitError = ()> + 'static,
  B: MessageBody + 'static,
{
  fn register(self, config: &mut AppService) {
    self.inner.register(config)
  }
}

impl<T> Resource<actix_web::Resource<T>>
where
  T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
  /// Proxy for [`actix_web::Resource::name`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.name).
  ///
  /// **NOTE:** This doesn't affect spec generation.
  pub fn name(mut self, name: &str) -> Self {
    self.inner = self.inner.name(name);
    self
  }

  /// Proxy for [`actix_web::Resource::guard`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.guard).
  ///
  /// **NOTE:** This doesn't affect spec generation.
  pub fn guard<G: Guard + 'static>(mut self, guard: G) -> Self {
    self.inner = self.inner.guard(guard);
    self
  }

  /// Wrapper for [`actix_web::Resource::route`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.route).
  pub fn route(mut self, route: Route) -> Self {
    let w = RouteWrapper::new(&self.path, route);
    let mut item_definition = self.item_definition.unwrap_or_default();
    let mut operations = w.def.item.operations;
    for operation in operations.values_mut() {
      operation.tags.append(&mut self.tags.clone());
    }
    item_definition.operations.extend(operations);
    self.item_definition = Some(item_definition);
    self.components.extend(w.component);
    self.inner = self.inner.route(w.inner);
    self
  }

  /// Proxy for [`actix_web::Resource::app_data`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.app_data).
  ///
  /// **NOTE:** This doesn't affect spec generation.
  pub fn app_data<U: 'static>(mut self, data: U) -> Self {
    let w = self.inner.app_data(data);
    self.inner = w;
    self
  }

  /// Wrapper for [`actix_web::Resource::to`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.to).
  pub fn to<F, Args>(mut self, handler: F) -> Self
  where
    F: Handler<Args>,
    Args: FromRequest + 'static,
    F::Output: Responder + 'static,
    F::Future: PathItemDefinition,
  {
    if F::Future::is_visible() {
      let mut operation = F::Future::operation();
      operation.tags.append(&mut self.tags.clone());
      let mut item_definition = self.item_definition.unwrap_or_default();
      for method in METHODS {
        item_definition.operations.insert(*method, operation.clone());
      }
      operation.update_path_parameter_name_from_path(&self.path);
      self.item_definition = Some(item_definition);
      self.components.extend(F::Future::components());
    }
    self.inner = self.inner.to(handler);
    self
  }

  /// Proxy for [`actix_web::web::Resource::wrap`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.wrap).
  ///
  /// **NOTE:** This doesn't affect spec generation.
  pub fn wrap<M, B>(
    self,
    mw: M,
  ) -> Resource<
    actix_web::Resource<
      impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse<B>, Error = Error, InitError = ()>,
    >,
  >
  where
    B: MessageBody,
    M: Transform<T::Service, ServiceRequest, Response = ServiceResponse<B>, Error = Error, InitError = ()> + 'static,
  {
    Resource {
      path: self.path,
      item_definition: self.item_definition,
      components: self.components,
      tags: self.tags,
      inner: self.inner.wrap(mw),
    }
  }

  /// Proxy for [`actix_web::web::Resource::wrap_fn`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.wrap_fn).
  ///
  /// **NOTE:** This doesn't affect spec generation.
  pub fn wrap_fn<F, R, B>(
    self,
    mw: F,
  ) -> Resource<
    actix_web::Resource<
      impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse<B>, Error = Error, InitError = ()>,
    >,
  >
  where
    B: MessageBody,
    F: Fn(ServiceRequest, &T::Service) -> R + Clone + 'static,
    R: Future<Output = Result<ServiceResponse<B>, Error>>,
  {
    Resource {
      path: self.path,
      item_definition: self.item_definition,
      components: self.components,
      tags: self.tags,
      inner: self.inner.wrap_fn(mw),
    }
  }

  /// Proxy for [`actix_web::web::Resource::default_service`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.default_service).
  ///
  /// **NOTE:** This doesn't affect spec generation.
  pub fn default_service<F, U>(mut self, f: F) -> Self
  where
    F: actix_service::IntoServiceFactory<U, ServiceRequest>,
    U: ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = Error, InitError = ()> + 'static,
    U::InitError: Debug,
  {
    self.inner = self.inner.default_service(f);
    self
  }
}

/// Wrapper for [`actix_web::web::resource`](https://docs.rs/actix-web/*/actix_web/web/fn.resource.html).
pub fn resource(path: &str) -> Resource {
  Resource::new(path)
}

/// Wrapper for [`actix_web::web::resource`](https://docs.rs/actix-web/*/actix_web/web/fn.resource.html) with a list of tag names for the given scope.
/// Tags should exist in `Spec` otherwise documentation might be considered invalid by consumers.
pub fn tagged_resource<T: Into<String>>(path: &str, tags: Vec<T>) -> Resource {
  Resource::new_tagged(path, tags)
}
