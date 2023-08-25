use crate::actix::ResponseWrapper;
use crate::path_item_definition::PathItemDefinition;
use actix_web::web::{Data, ReqData};
use actix_web::Responder;
use std::collections::BTreeMap;
use std::future::Future;
use utoipa::openapi::path::Parameter;
use utoipa::openapi::request_body::{RequestBody, RequestBodyBuilder};
use utoipa::openapi::security::SecurityScheme;
use utoipa::openapi::{
  ContentBuilder, Ref, RefOr, Required, ResponseBuilder, Responses, ResponsesBuilder, Schema, SecurityRequirement,
};

pub mod empty;
pub mod json;
pub mod parameters;
pub mod simple;

pub trait ApiComponent {
  fn required() -> Required {
    Required::True
  }

  /// contains childs schemas recursively for this operation
  fn child_schemas() -> Vec<(String, RefOr<Schema>)>;

  fn raw_schema() -> Option<RefOr<Schema>> {
    None
  }

  fn schema() -> Option<(String, RefOr<Schema>)>;

  fn securities() -> BTreeMap<String, SecurityScheme> {
    Default::default()
  }

  fn security_requirement_name() -> Option<String> {
    None
  }

  fn request_body() -> Option<RequestBody> {
    Self::schema().map(|(name, _)| {
      RequestBodyBuilder::new()
        .content(
          "application/json", //@todo how to infer it
          ContentBuilder::new().schema(Ref::from_schema_name(name)).build(),
        )
        .required(Some(Self::required()))
        .build()
    })
  }

  fn responses() -> Option<Responses> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    vec![]
  }
}

impl<T> ApiComponent for Option<T>
where
  T: ApiComponent,
{
  fn required() -> Required {
    Required::False
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    T::schema()
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    T::raw_schema()
  }

  fn security_requirement_name() -> Option<String> {
    T::security_requirement_name()
  }

  fn securities() -> BTreeMap<String, SecurityScheme> {
    T::securities()
  }
}

impl<T> ApiComponent for Vec<T>
where
  T: ApiComponent,
{
  //@todo sure about this one ?
  fn required() -> Required {
    Required::True
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    T::raw_schema()
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    T::schema()
  }
}

impl<T, E> ApiComponent for Result<T, E>
where
  T: ApiComponent,
{
  fn required() -> Required {
    T::required()
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    T::raw_schema()
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    T::schema()
  }
}

impl<T> ApiComponent for Data<T> {
  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    vec![]
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    None
  }
}

impl<T: Clone> ApiComponent for ReqData<T> {
  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    vec![]
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    None
  }
}

impl<F, R, P> ApiComponent for ResponseWrapper<F, P>
where
  F: Future<Output = R>,
  R: Responder + ApiComponent,
  P: PathItemDefinition,
{
  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    R::child_schemas()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    R::schema()
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    R::raw_schema()
  }

  fn responses() -> Option<Responses> {
    //@todo handle error
    Self::schema().map(|(name, schema)| {
      let _ref = match schema {
        RefOr::Ref(r) => r,
        RefOr::T(_) => Ref::from_schema_name(name),
      };
      let mut responses = ResponsesBuilder::new();
      responses = responses.response(
        "200",
        ResponseBuilder::new()
          .content(
            "application/json", //@todo how to infer it
            ContentBuilder::new().schema(_ref).build(),
          )
          .build(),
      );
      responses.build()
    })
  }
}
