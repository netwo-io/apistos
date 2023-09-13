use crate::actix::ResponseWrapper;
use crate::internal::components::error::ApiErrorComponent;
use crate::path_item_definition::PathItemDefinition;
use actix_web::web::{Data, ReqData};
use actix_web::Responder;
use netwopenapi_models::paths::{MediaType, Parameter, RequestBody, Response, Responses};
use netwopenapi_models::reference_or::ReferenceOr;
use netwopenapi_models::security::SecurityScheme;
use netwopenapi_models::InstanceType;
use netwopenapi_models::Schema;
use std::collections::BTreeMap;
use std::future::Future;

pub mod empty;
pub mod error;
pub mod form;
pub mod json;
#[cfg(any(feature = "multipart", feature = "extras"))]
pub mod multipart;
pub mod parameters;
pub mod simple;

pub trait TypedSchema {
  fn schema_type() -> InstanceType;
  fn format() -> Option<String>;
}

pub trait ApiComponent {
  fn content_type() -> String {
    "application/json".to_string()
  }

  fn required() -> bool {
    true
  }

  /// contains childs schemas recursively for this operation
  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)>;

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    None
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)>;

  fn securities() -> BTreeMap<String, SecurityScheme> {
    Default::default()
  }

  fn security_requirement_name() -> Option<String> {
    None
  }

  fn request_body() -> Option<RequestBody> {
    Self::schema().map(|(name, _)| RequestBody {
      content: BTreeMap::from_iter(vec![(
        Self::content_type(),
        MediaType {
          schema: Some(ReferenceOr::Reference {
            _ref: format!("#/components/schemas/{}", name),
          }),
          ..Default::default()
        },
      )]),
      required: Some(Self::required()),
      ..Default::default()
    })
  }

  fn error_responses() -> Vec<(String, Response)> {
    vec![]
  }

  fn error_schemas() -> BTreeMap<String, (String, ReferenceOr<Schema>)> {
    BTreeMap::default()
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
  fn required() -> bool {
    false
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    T::schema()
  }

  fn securities() -> BTreeMap<String, SecurityScheme> {
    T::securities()
  }

  fn security_requirement_name() -> Option<String> {
    T::security_requirement_name()
  }
}

impl<T> ApiComponent for Vec<T>
where
  T: ApiComponent,
{
  fn required() -> bool {
    true
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    T::schema()
  }
}

impl<T, E> ApiComponent for Result<T, E>
where
  T: ApiComponent,
  E: ApiErrorComponent,
{
  fn required() -> bool {
    T::required()
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    T::schema()
  }

  // We expect error to be present only for response part
  fn error_responses() -> Vec<(String, Response)> {
    E::error_responses()
  }

  // We expect error to be present only for response part
  fn error_schemas() -> BTreeMap<String, (String, ReferenceOr<Schema>)> {
    E::schemas_by_status_code()
  }

  fn responses() -> Option<Responses> {
    T::responses()
  }
}

impl<T> ApiComponent for Data<T> {
  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    vec![]
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }
}

impl<T: Clone> ApiComponent for ReqData<T> {
  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    vec![]
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }
}

impl<F, R, P> ApiComponent for ResponseWrapper<F, P>
where
  F: Future<Output = R>,
  R: Responder + ApiComponent,
  P: PathItemDefinition,
{
  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    R::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    R::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    R::schema()
  }

  fn error_responses() -> Vec<(String, Response)> {
    R::error_responses()
  }

  fn error_schemas() -> BTreeMap<String, (String, ReferenceOr<Schema>)> {
    R::error_schemas()
  }

  fn responses() -> Option<Responses> {
    let mut responses = vec![];
    if let Some(response) = R::responses() {
      responses.append(
        &mut response
          .responses
          .into_iter()
          .collect::<Vec<(String, ReferenceOr<Response>)>>(),
      );
    } else if let Some((name, schema)) = Self::schema() {
      let _ref = match schema {
        ReferenceOr::Reference { _ref } => _ref,
        ReferenceOr::Object(_) => format!("#/components/schemas/{}", name),
      };
      responses.push((
        "200".to_owned(),
        ReferenceOr::Object(Response {
          content: BTreeMap::from_iter(vec![(
            Self::content_type(),
            MediaType {
              schema: Some(ReferenceOr::Reference { _ref }),
              ..Default::default()
            },
          )]),
          ..Default::default()
        }),
      ));
    } else {
      responses.push(("200".to_owned(), ReferenceOr::Object(Response::default())));
    }

    responses.append(
      &mut Self::error_responses()
        .into_iter()
        .map(|(status, schema)| (status, ReferenceOr::Object(schema)))
        .collect(),
    );
    Some(Responses {
      responses: BTreeMap::from_iter(responses),
      ..Default::default()
    })
  }
}
