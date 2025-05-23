use crate::internal::actix::METHODS;
use crate::internal::actix::utils::OperationUpdater;
use actix_service::boxed::BoxService;
use actix_service::{ServiceFactory, Transform};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::guard::Guard;
use actix_web::http::Method;
use actix_web::{Error, FromRequest, Handler, Responder};
use apistos_core::PathItemDefinition;
use apistos_models::components::Components;
use apistos_models::paths::{Operation, OperationType, PathItem};
use indexmap::IndexMap;
use log::warn;

/// Wrapper for [`actix_web::web::method`](https://docs.rs/actix-web/*/actix_web/web/fn.method.html).
pub fn method(method: Method) -> Route {
  Route::new().method(method)
}

/// Wrapper for [`actix_web::web::get`](https://docs.rs/actix-web/*/actix_web/web/fn.get.html).
pub fn get() -> Route {
  method(Method::GET)
}

/// Wrapper for [`actix_web::web::put`](https://docs.rs/actix-web/*/actix_web/web/fn.put.html).
pub fn put() -> Route {
  method(Method::PUT)
}

/// Wrapper for [`actix_web::web::post`](https://docs.rs/actix-web/*/actix_web/web/fn.post.html).
pub fn post() -> Route {
  method(Method::POST)
}

/// Wrapper for [`actix_web::web::patch`](https://docs.rs/actix-web/*/actix_web/web/fn.patch.html).
pub fn patch() -> Route {
  method(Method::PATCH)
}

/// Wrapper for [`actix_web::web::delete`](https://docs.rs/actix-web/*/actix_web/web/fn.delete.html).
pub fn delete() -> Route {
  method(Method::DELETE)
}

/// Wrapper for [`actix_web::web::options`](https://docs.rs/actix-web/*/actix_web/web/fn.options.html).
pub fn options() -> Route {
  method(Method::OPTIONS)
}

/// Wrapper for [`actix_web::web::head`](https://docs.rs/actix-web/*/actix_web/web/fn.head.html).
pub fn head() -> Route {
  method(Method::HEAD)
}

pub enum OperationTypeDoc {
  OperationType(OperationType),
  AllMethods,
  Undocumented,
}

pub struct Route {
  operation: Option<Operation>,
  path_item_type: OperationTypeDoc,
  components: Vec<Components>,
  inner: actix_web::Route,
}

impl ServiceFactory<ServiceRequest> for Route {
  type Response =
    <<actix_web::Route as ServiceFactory<ServiceRequest>>::Service as actix_service::Service<ServiceRequest>>::Response;
  type Error = Error;
  type Config = ();
  type Service = <actix_web::Route as ServiceFactory<ServiceRequest>>::Service;
  type InitError = ();
  type Future = <actix_web::Route as ServiceFactory<ServiceRequest>>::Future;

  #[allow(clippy::unit_arg)]
  fn new_service(&self, cfg: Self::Config) -> Self::Future {
    self.inner.new_service(cfg)
  }
}

impl Route {
  /// Wrapper for [`actix_web::Route::new`](https://docs.rs/actix-web/*/actix_web/struct.Route.html#method.new)
  #[allow(clippy::new_without_default)]
  pub fn new() -> Route {
    Route {
      operation: None,
      path_item_type: OperationTypeDoc::AllMethods,
      components: Default::default(),
      inner: actix_web::Route::new(),
    }
  }

  /// Drop in for [`actix_web::Route::wrap`](https://docs.rs/actix-web/*/actix_web/struct.Route.html#method.wrap)
  #[doc(alias = "middleware")]
  #[doc(alias = "use")] // nodejs terminology
  pub fn wrap<M, B>(self, mw: M) -> Self
  where
    M: Transform<
        BoxService<ServiceRequest, ServiceResponse, Error>,
        ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
        InitError = (),
      > + 'static,
    B: MessageBody + 'static,
  {
    Route {
      operation: self.operation,
      path_item_type: self.path_item_type,
      components: self.components,
      inner: self.inner.wrap(mw),
    }
  }

  /// Wrapper for [`actix_web::Route::method`](https://docs.rs/actix-web/*/actix_web/struct.Route.html#method.method)
  pub fn method(mut self, method: Method) -> Self {
    let path_item_type = match method.as_str() {
      "PUT" => OperationTypeDoc::OperationType(OperationType::Put),
      "POST" => OperationTypeDoc::OperationType(OperationType::Post),
      "DELETE" => OperationTypeDoc::OperationType(OperationType::Delete),
      "OPTIONS" => OperationTypeDoc::OperationType(OperationType::Options),
      "HEAD" => OperationTypeDoc::OperationType(OperationType::Head),
      "PATCH" => OperationTypeDoc::OperationType(OperationType::Patch),
      "TRACE" => OperationTypeDoc::OperationType(OperationType::Trace),
      "GET" => OperationTypeDoc::OperationType(OperationType::Get),
      m => {
        warn!("Unsupported method found: {m}, operation will not be documented");
        OperationTypeDoc::Undocumented
      }
    };
    self.path_item_type = path_item_type;
    self.inner = self.inner.method(method);
    self
  }

  /// Proxy for [`actix_web::Route::guard`](https://docs.rs/actix-web/*/actix_web/struct.Route.html#method.guard).
  ///
  /// **NOTE:** This doesn't affect spec generation.
  pub fn guard<G: Guard + 'static>(mut self, guard: G) -> Self {
    self.inner = self.inner.guard(guard);
    self
  }

  /// Wrapper for [`actix_web::Route::to`](https://docs.rs/actix-web/*/actix_web/struct.Route.html#method.to)
  pub fn to<F, Args>(mut self, handler: F) -> Self
  where
    F: Handler<Args>,
    Args: FromRequest + 'static,
    F::Output: Responder + 'static,
    F::Future: PathItemDefinition,
  {
    if F::Future::is_visible() {
      self.operation = Some(F::Future::operation());
      self.components = F::Future::components();
    }
    self.inner = self.inner.to(handler);
    self
  }
}

pub(crate) struct PathDefinition {
  pub(crate) path: String,
  pub(crate) item: PathItem,
}

pub(crate) struct RouteWrapper {
  pub(crate) def: PathDefinition,
  pub(crate) component: Vec<Components>,
  pub(crate) inner: actix_web::Route,
}

impl RouteWrapper {
  pub(crate) fn new<S: Into<String>>(path: S, route: Route) -> Self {
    let mut operations: IndexMap<OperationType, Operation> = Default::default();
    let mut path_item = PathItem::default();
    let path: String = path.into();
    if let Some(mut operation) = route.operation {
      operation.update_path_parameter_name_from_path(&path);

      match route.path_item_type {
        OperationTypeDoc::OperationType(path_item_type) => {
          operations.insert(path_item_type, operation);
        }
        OperationTypeDoc::AllMethods => {
          for path_item_type in METHODS {
            operations.insert(*path_item_type, operation.clone());
          }
        }
        OperationTypeDoc::Undocumented => {}
      }
    }
    path_item.operations = operations;

    Self {
      def: PathDefinition { path, item: path_item },
      component: route.components,
      inner: route.inner,
    }
  }
}
