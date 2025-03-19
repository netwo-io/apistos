use crate::ApiErrorComponent;
#[cfg(feature = "actix")]
use crate::{PathItemDefinition, ResponseWrapper};
#[cfg(feature = "actix")]
use actix_web::Either;
use apistos_models::Schema;
use apistos_models::paths::{MediaType, Parameter, RequestBody, Response, Responses};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::security::SecurityScheme;
#[cfg(feature = "actix")]
use schemars::schema::SubschemaValidation;
use schemars::schema::{ArrayValidation, InstanceType, SchemaObject, SingleOrVec};
use std::collections::BTreeMap;
#[cfg(feature = "actix")]
use std::future::Future;

pub trait ApiComponent {
  fn content_type() -> String {
    "application/json".to_string()
  }

  fn required() -> bool {
    true
  }

  /// Contains children schemas for this operation
  /// Each child can also contain child schemas
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

  fn responses(_content_type: Option<String>) -> Option<Responses> {
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
    let mut schemas = T::schema().into_iter().collect::<Vec<(String, ReferenceOr<Schema>)>>();
    schemas.append(&mut T::child_schemas());
    schemas
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    T::schema().map(|(name, schema)| {
      let _ref = match schema {
        ReferenceOr::Reference { _ref } => _ref,
        ReferenceOr::Object(_) => format!("#/components/schemas/{}", name),
      };

      (
        name,
        ReferenceOr::Object(Schema::Object(SchemaObject {
          instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Array))),
          array: Some(Box::new(ArrayValidation {
            items: Some(Schema::new_ref(_ref).into()),
            ..Default::default()
          })),
          ..Default::default()
        })),
      )
    })
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

  fn responses(content_type: Option<String>) -> Option<Responses> {
    T::responses(content_type)
  }
}

#[cfg(feature = "actix")]
impl<T, E> ApiComponent for Either<T, E>
where
  T: ApiComponent,
  E: ApiComponent,
{
  fn required() -> bool {
    T::required() && E::required()
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    let mut child_schemas = T::child_schemas();
    child_schemas.append(&mut E::child_schemas());
    child_schemas
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    match (T::raw_schema(), E::raw_schema()) {
      (Some(raw_schema1), Some(raw_schema2)) => {
        let raw_schema1 = match raw_schema1 {
          ReferenceOr::Object(schema_obj) => schema_obj,
          ReferenceOr::Reference { _ref } => Schema::Object(SchemaObject {
            reference: Some(_ref),
            ..Default::default()
          }),
        };
        let raw_schema2 = match raw_schema2 {
          ReferenceOr::Object(schema_obj) => schema_obj,
          ReferenceOr::Reference { _ref } => Schema::Object(SchemaObject {
            reference: Some(_ref),
            ..Default::default()
          }),
        };
        Some(ReferenceOr::Object(Schema::Object(SchemaObject {
          subschemas: Some(Box::new(SubschemaValidation {
            one_of: Some(vec![raw_schema1, raw_schema2]),
            ..Default::default()
          })),
          ..Default::default()
        })))
      }
      (Some(raw_schema1), None) => Some(raw_schema1),
      (None, Some(raw_schema2)) => Some(raw_schema2),
      (None, None) => None,
    }
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    match (T::schema(), E::schema()) {
      (Some(schema1), Some(schema2)) => {
        let (schema_name1, schema1) = schema1;
        let schema1 = match schema1 {
          ReferenceOr::Object(schema_obj) => schema_obj,
          ReferenceOr::Reference { _ref } => Schema::Object(SchemaObject {
            reference: Some(_ref),
            ..Default::default()
          }),
        };
        let (schema_name2, schema2) = schema2;
        let schema2 = match schema2 {
          ReferenceOr::Object(schema_obj) => schema_obj,
          ReferenceOr::Reference { _ref } => Schema::Object(SchemaObject {
            reference: Some(_ref),
            ..Default::default()
          }),
        };
        let schema = ReferenceOr::Object(Schema::Object(SchemaObject {
          subschemas: Some(Box::new(SubschemaValidation {
            one_of: Some(vec![schema1, schema2]),
            ..Default::default()
          })),
          ..Default::default()
        }));
        let schema_name = format!("Either{}Or{}", schema_name1, schema_name2);
        Some((schema_name, schema))
      }
      (Some(schema1), None) => Some(schema1),
      (None, Some(schema2)) => Some(schema2),
      (None, None) => None,
    }
  }

  fn error_responses() -> Vec<(String, Response)> {
    let mut error_responses = T::error_responses();
    error_responses.append(&mut E::error_responses());
    error_responses
  }

  fn error_schemas() -> BTreeMap<String, (String, ReferenceOr<Schema>)> {
    let mut error_schemas = E::error_schemas();
    error_schemas.append(&mut T::error_schemas());
    error_schemas
  }

  fn responses(content_type: Option<String>) -> Option<Responses> {
    let responses = T::responses(content_type.clone());
    match responses {
      None => E::responses(content_type),
      Some(mut responses) => {
        responses
          .responses
          .append(&mut E::responses(content_type).map(|r| r.responses).unwrap_or_default());
        Some(responses)
      }
    }
  }
}

#[cfg(feature = "actix")]
impl<T> ApiComponent for actix_web::web::Data<T> {
  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    vec![]
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }
}

#[cfg(feature = "actix")]
impl<T: Clone> ApiComponent for actix_web::web::ReqData<T> {
  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    vec![]
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }
}

#[cfg(feature = "actix")]
impl<F, R, P> ApiComponent for ResponseWrapper<F, P>
where
  F: Future<Output = R>,
  R: actix_web::Responder + ApiComponent,
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

  fn responses(content_type: Option<String>) -> Option<Responses> {
    let mut responses = vec![];
    if let Some(response) = R::responses(content_type.clone()) {
      responses.append(
        &mut response
          .responses
          .into_iter()
          .collect::<Vec<(String, ReferenceOr<Response>)>>(),
      );
    } else if let Some((name, schema)) = Self::schema() {
      let ref_or = match schema {
        r @ ReferenceOr::Reference { .. } => r,
        ReferenceOr::Object(schema_obj) => {
          let _ref = ReferenceOr::Reference {
            _ref: format!("#/components/schemas/{}", name),
          };
          match schema_obj {
            Schema::Object(obj) => {
              if obj.instance_type == Some(SingleOrVec::Single(Box::new(InstanceType::Array))) {
                ReferenceOr::Object(Schema::Object(obj))
              } else {
                _ref
              }
            }
            Schema::Bool(_) => _ref,
          }
        }
      };
      responses.push((
        "200".to_owned(),
        ReferenceOr::Object(Response {
          content: BTreeMap::from_iter(vec![(
            content_type.unwrap_or_else(Self::content_type),
            MediaType {
              schema: Some(ref_or),
              ..Default::default()
            },
          )]),
          ..Default::default()
        }),
      ));
    } else if let Some(schema) = Self::raw_schema() {
      responses.push((
        "200".to_owned(),
        ReferenceOr::Object(Response {
          content: BTreeMap::from_iter(vec![(
            content_type.unwrap_or_else(Self::content_type),
            MediaType {
              schema: Some(schema),
              ..Default::default()
            },
          )]),
          ..Default::default()
        }),
      ));
    } else if let Some(content_type) = content_type {
      responses.push((
        "200".to_owned(),
        ReferenceOr::Object(Response {
          content: BTreeMap::from_iter(vec![(content_type, MediaType::default())]),
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

#[cfg(test)]
mod test {
  use crate::ApiComponent;
  use apistos_models::reference_or::ReferenceOr;
  use assert_json_diff::assert_json_eq;
  use schemars::schema::{InstanceType, ObjectValidation, Schema, SchemaObject, SingleOrVec};
  use schemars::{Map, Set};
  use serde_json::json;

  #[test]
  #[allow(dead_code)]
  fn api_component_schema_vec() {
    struct TestChild {
      surname: String,
    }

    impl ApiComponent for TestChild {
      fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
        vec![]
      }

      fn schema() -> Option<(String, ReferenceOr<Schema>)> {
        Some((
          "TestChild".to_string(),
          ReferenceOr::Object(Schema::Object(SchemaObject {
            object: Some(Box::new(ObjectValidation {
              required: Set::from_iter(vec!["surname".to_string()]),
              properties: Map::from_iter(vec![(
                "surname".to_string(),
                Schema::Object(SchemaObject {
                  instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
                  ..Default::default()
                }),
              )]),
              ..Default::default()
            })),
            ..Default::default()
          })),
        ))
      }
    }

    struct Test {
      name: String,
      surname: TestChild,
    }

    impl ApiComponent for Test {
      fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
        <TestChild as ApiComponent>::schema().into_iter().collect()
      }

      fn schema() -> Option<(String, ReferenceOr<Schema>)> {
        Some((
          "Test".to_string(),
          ReferenceOr::Object(Schema::Object(SchemaObject {
            object: Some(Box::new(ObjectValidation {
              required: Set::from_iter(vec!["name".to_string(), "surname".to_string()]),
              properties: Map::from_iter(vec![
                (
                  "name".to_string(),
                  Schema::Object(SchemaObject {
                    instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
                    ..Default::default()
                  }),
                ),
                (
                  "surname".to_string(),
                  Schema::new_ref("#/components/schemas/TestChild".to_string()),
                ),
              ]),
              ..Default::default()
            })),
            ..Default::default()
          })),
        ))
      }
    }

    let schema = <Vec<Test> as ApiComponent>::schema();
    assert!(schema.is_some());

    let json = serde_json::to_value(schema.expect("Missing schema").1).expect("Unable to serialize as Json");
    assert_json_eq!(
      json,
      json!({
        "type": "array",
        "items": {
          "$ref": "#/components/schemas/Test"
        }
      })
    );

    let child_schema = <Vec<Test> as ApiComponent>::child_schemas();
    assert_eq!(child_schema.len(), 2);
    let first_child_schema = child_schema.first().cloned();
    assert!(first_child_schema.is_some());

    let json =
      serde_json::to_value(first_child_schema.expect("Missing child schema").1).expect("Unable to serialize as Json");
    assert_json_eq!(
      json,
      json!({
        "properties": {
          "name": {
            "type": "string"
          },
          "surname": {
            "$ref": "#/components/schemas/TestChild"
          }
        },
        "required": [
          "name",
          "surname"
        ]
      })
    );

    let last_child_schema = child_schema.last().cloned();
    assert!(last_child_schema.is_some());

    let json =
      serde_json::to_value(last_child_schema.expect("Missing child schema").1).expect("Unable to serialize as Json");
    assert_json_eq!(
      json,
      json!( {
        "properties": {
          "surname": {
            "type": "string"
          }
        },
        "required": [
          "surname"
        ]
      })
    );
  }
}
