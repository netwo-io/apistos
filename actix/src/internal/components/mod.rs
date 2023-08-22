use actix_web::web::{Data, ReqData};
use utoipa::openapi::request_body::{RequestBody, RequestBodyBuilder};
use utoipa::openapi::{ContentBuilder, Ref, RefOr, Required, Schema};
use utoipa::ToSchema;

pub mod empty;
pub mod json;
pub mod parameters;
pub mod simple;

pub trait ApiComponent {
  fn required() -> bool {
    true
  }

  /// contains childs schemas recursively for this operation
  fn child_schemas() -> Vec<(String, RefOr<Schema>)>;

  fn schema() -> Option<(String, RefOr<Schema>)>;

  fn request_body() -> Option<RequestBody> {
    let required = match Self::required() {
      true => Required::True,
      false => Required::False,
    };
    Self::schema().map(|(name, _)| {
      RequestBodyBuilder::new()
        .content(
          "application/json",
          ContentBuilder::new().schema(Ref::from_schema_name(name)).build(),
        )
        .required(Some(required))
        .build()
    })
  }
}

impl<T> ApiComponent for Option<T>
where
  T: ApiComponent,
{
  fn required() -> bool {
    false
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    T::schema()
  }
}

impl<T> ApiComponent for Vec<T>
where
  T: ApiComponent,
{
  //@todo sure about this one ?
  fn required() -> bool {
    true
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
  fn required() -> bool {
    T::required()
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
