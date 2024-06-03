use std::collections::BTreeMap;
#[cfg(feature = "actix")]
use std::future::Future;

use serde_json::{json, Value};

use apistos_models::paths::{MediaType, Parameter, RequestBody, Response, Responses};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::security::SecurityScheme;
use apistos_models::Schema;

use crate::ApiErrorComponent;
#[cfg(feature = "actix")]
use crate::{PathItemDefinition, ResponseWrapper};

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
        Schema::try_from(json!({
          "type": "array",
          "items": Schema::new_ref(_ref)
        }))
        .map_err(|err| {
          log::warn!("Error generating json schema: {err:?}");
          err
        })
        .unwrap_or_default()
        .into(),
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
          if let Some(obj) = schema_obj.as_object() {
            let value = obj.get("type");
            match value {
              Some(Value::String(string)) if string == "array" => ReferenceOr::Object(schema_obj),
              _ => _ref,
            }
          } else {
            _ref
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
  use schemars::gen::{SchemaGenerator, SchemaSettings};
  use schemars::{JsonSchema, Schema};
  use serde_json::json;

  use apistos_models::reference_or::ReferenceOr;
  use assert_json_diff::assert_json_eq;

  use crate::ApiComponent;

  #[test]
  #[allow(dead_code)]
  fn api_component_schema_vec() {
    #[derive(JsonSchema)]
    struct TestChild {
      surname: String,
    }

    impl ApiComponent for TestChild {
      fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
        vec![]
      }

      fn schema() -> Option<(String, ReferenceOr<Schema>)> {
        let gen = SchemaGenerator::new(SchemaSettings::openapi3());
        Some(("TestChild".to_string(), gen.into_root_schema_for::<TestChild>().into()))
      }
    }

    #[derive(JsonSchema)]
    struct Test {
      name: String,
      surname: TestChild,
    }

    impl ApiComponent for Test {
      fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
        <TestChild as ApiComponent>::schema().into_iter().collect()
      }

      fn schema() -> Option<(String, ReferenceOr<Schema>)> {
        let gen = SchemaSettings::openapi3().into_generator();
        let definition_path = gen.settings().definitions_path.clone();
        let definition_path = definition_path
          .trim_start_matches('/')
          .split('/')
          .next()
          .unwrap_or_default();
        let mut schema = gen.into_root_schema_for::<Test>();
        let obj = schema.ensure_object();
        obj.remove(definition_path);
        let schema = Schema::from(obj.clone());
        Some(("Test".to_string(), schema.into()))
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
        "$schema": "https://spec.openapis.org/oas/3.0/schema/2021-09-28#/definitions/Schema",
        "title": "Test",
        "type": "object",
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
      json!({
        "$schema": "https://spec.openapis.org/oas/3.0/schema/2021-09-28#/definitions/Schema",
        "title": "TestChild",
        "type": "object",
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
