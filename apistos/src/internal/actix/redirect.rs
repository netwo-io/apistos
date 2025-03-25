use actix_web::dev::{AppService, HttpServiceFactory};

use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};
use apistos_models::paths::{Header, MediaType, ParameterDefinition, Response};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::{Schema, SchemaObject};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Redirect {
  pub(crate) path: String,
  pub(crate) redirect: String,
  pub(crate) code: StatusCode,
  inner: actix_web::web::Redirect,
}

impl From<Redirect> for actix_web::web::Redirect {
  fn from(value: Redirect) -> Self {
    value.inner
  }
}

impl Redirect {
  /// Wrapper for [`actix_web::web::Redirect`](https://docs.rs/actix-web/*/actix_web/web/struct.Redirect.html#method.new)
  pub fn new(from: impl Into<Cow<'static, str>>, to: impl Into<Cow<'static, str>>) -> Self {
    let from: Cow<'static, str> = from.into();
    let to: Cow<'static, str> = to.into();

    Redirect {
      path: from.clone().to_string(),
      redirect: to.clone().to_string(),
      code: StatusCode::TEMPORARY_REDIRECT,
      inner: actix_web::web::Redirect::new(from, to),
    }
  }

  /// Wrapper for [`actix_web::web::Redirect`](https://docs.rs/actix-web/*/actix_web/web/struct.Redirect.html#method.to).
  pub fn to(to: impl Into<Cow<'static, str>>) -> Self {
    let to: Cow<'static, str> = to.into();
    Redirect {
      path: "/".to_owned(),
      redirect: to.clone().to_string(),
      code: StatusCode::TEMPORARY_REDIRECT,
      inner: actix_web::web::Redirect::to(to),
    }
  }

  /// Wrapper for [`actix_web::web::Redirect`](https://docs.rs/actix-web/*/actix_web/web/struct.Redirect.html#method.permanent).
  pub fn permanent(self) -> Self {
    Redirect {
      path: self.path,
      redirect: self.redirect,
      code: StatusCode::PERMANENT_REDIRECT,
      inner: self.inner.permanent(),
    }
  }

  /// Wrapper for [`actix_web::web::Redirect`](https://docs.rs/actix-web/*/actix_web/web/struct.Redirect.html#method.temporary).
  pub fn temporary(self) -> Self {
    Redirect {
      path: self.path,
      redirect: self.redirect,
      code: StatusCode::TEMPORARY_REDIRECT,
      inner: self.inner.temporary(),
    }
  }

  /// Wrapper for [`actix_web::web::Redirect`](https://docs.rs/actix-web/*/actix_web/web/struct.Redirect.html#method.see_other).
  pub fn see_other(self) -> Self {
    Redirect {
      path: self.path,
      redirect: self.redirect,
      code: StatusCode::SEE_OTHER,
      inner: self.inner.see_other(),
    }
  }

  /// Wrapper for [`actix_web::web::Redirect`](https://docs.rs/actix-web/*/actix_web/web/struct.Redirect.html#method.using_status_code).
  pub fn using_status_code(mut self, status: StatusCode) -> Self {
    self = Redirect {
      path: self.path,
      redirect: self.redirect,
      code: status,
      inner: self.inner.using_status_code(status),
    };
    self
  }

  pub(crate) fn get_open_api_response(&self) -> Response {
    let location_header = Header {
      definition: Some(ParameterDefinition::Content(BTreeMap::from_iter(vec![(
        "text/plain".to_string(),
        MediaType {
          schema: Some(ReferenceOr::Object(Schema::Object(SchemaObject {
            enum_values: Some(vec![Value::String(self.redirect.clone())]),
            ..Default::default()
          }))),
          ..Default::default()
        },
      )]))),
      description: Some("Redirection URL".to_owned()),
      ..Default::default()
    };

    Response {
      headers: BTreeMap::from_iter(vec![("Location".to_string(), ReferenceOr::Object(location_header))]),
      ..Default::default()
    }
  }
}

impl HttpServiceFactory for Redirect {
  fn register(self, config: &mut AppService) {
    self.inner.register(config);
  }
}

impl Responder for Redirect {
  type Body = ();

  fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
    self.inner.respond_to(req)
  }
}

pub fn redirect(from: impl Into<Cow<'static, str>>, to: impl Into<Cow<'static, str>>) -> Redirect {
  Redirect::new(from, to)
}
