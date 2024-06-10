use std::collections::BTreeMap;
#[cfg(feature = "actix")]
use std::future::Future;

use serde_json::{json, Value};

use apistos_models::paths::{MediaType, Parameter, RequestBody, Response, Responses};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::security::SecurityScheme;
use apistos_models::{ApistosSchema, OpenApiVersion, Schema, VersionSpecificSchema};

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
  fn child_schemas(oas_version: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)>;

  fn raw_schema(_oas_version: OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
    None
  }

  fn schema(oas_version: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)>;

  fn securities() -> BTreeMap<String, SecurityScheme> {
    Default::default()
  }

  fn security_requirement_name() -> Option<String> {
    None
  }

  fn request_body(oas_version: OpenApiVersion) -> Option<RequestBody> {
    Self::schema(oas_version).map(|(name, sch)| {
      let schema = match oas_version {
        OpenApiVersion::OAS3_0 => VersionSpecificSchema::OAS3_0(ReferenceOr::Reference {
          _ref: format!("#/components/schemas/{}", name),
        }),
        OpenApiVersion::OAS3_1 => match sch {
          ReferenceOr::Object(schema) => VersionSpecificSchema::OAS3_1(schema),
          ReferenceOr::Reference { .. } => {
            log::warn!("Unexpected reference for request schema with oas 3.1");
            VersionSpecificSchema::OAS3_1(ApistosSchema::default())
          }
        },
      };

      RequestBody {
        content: BTreeMap::from_iter(vec![(
          Self::content_type(),
          MediaType {
            schema: Some(schema),
            ..Default::default()
          },
        )]),
        required: Some(Self::required()),
        ..Default::default()
      }
    })
  }

  fn error_responses(_oas_version: OpenApiVersion) -> Vec<(String, Response)> {
    vec![]
  }

  fn error_schemas(_oas_version: OpenApiVersion) -> BTreeMap<String, (String, ReferenceOr<ApistosSchema>)> {
    BTreeMap::default()
  }

  fn responses(_oas_version: OpenApiVersion, _content_type: Option<String>) -> Option<Responses> {
    None
  }

  fn parameters(_oas_version: OpenApiVersion) -> Vec<Parameter> {
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

  fn child_schemas(oas_version: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn raw_schema(oas_version: OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
    T::raw_schema(oas_version)
  }

  fn schema(oas_version: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version)
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

  fn child_schemas(oas_version: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    let mut schemas = T::schema(oas_version)
      .into_iter()
      .collect::<Vec<(String, ReferenceOr<ApistosSchema>)>>();
    schemas.append(&mut T::child_schemas(oas_version));
    schemas
  }

  fn raw_schema(oas_version: OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
    T::raw_schema(oas_version)
  }

  fn schema(oas_version: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version).map(|(name, schema)| {
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
        .map(|sch| ApistosSchema::new(sch, oas_version))
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

  fn child_schemas(oas_version: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    T::child_schemas(oas_version)
  }

  fn raw_schema(oas_version: OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
    T::raw_schema(oas_version)
  }

  fn schema(oas_version: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    T::schema(oas_version)
  }

  // We expect error to be present only for response part
  fn error_responses(oas_version: OpenApiVersion) -> Vec<(String, Response)> {
    E::error_responses(oas_version)
  }

  // We expect error to be present only for response part
  fn error_schemas(oas_version: OpenApiVersion) -> BTreeMap<String, (String, ReferenceOr<ApistosSchema>)> {
    E::schemas_by_status_code(oas_version)
  }

  fn responses(oas_version: OpenApiVersion, content_type: Option<String>) -> Option<Responses> {
    T::responses(oas_version, content_type)
  }
}

#[cfg(feature = "actix")]
impl<T> ApiComponent for actix_web::web::Data<T> {
  fn child_schemas(_: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    vec![]
  }

  fn schema(_: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    None
  }
}

#[cfg(feature = "actix")]
impl<T: Clone> ApiComponent for actix_web::web::ReqData<T> {
  fn child_schemas(_: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    vec![]
  }

  fn schema(_: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
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
  fn child_schemas(oas_version: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    R::child_schemas(oas_version)
  }

  fn raw_schema(oas_version: OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
    R::raw_schema(oas_version)
  }

  fn schema(oas_version: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
    R::schema(oas_version)
  }

  fn error_responses(oas_version: OpenApiVersion) -> Vec<(String, Response)> {
    R::error_responses(oas_version)
  }

  fn error_schemas(oas_version: OpenApiVersion) -> BTreeMap<String, (String, ReferenceOr<ApistosSchema>)> {
    R::error_schemas(oas_version)
  }

  fn responses(oas_version: OpenApiVersion, content_type: Option<String>) -> Option<Responses> {
    let mut responses = vec![];
    if let Some(response) = R::responses(oas_version, content_type.clone()) {
      responses.append(
        &mut response
          .responses
          .into_iter()
          .collect::<Vec<(String, ReferenceOr<Response>)>>(),
      );
    } else if let Some((name, schema)) = Self::schema(oas_version) {
      let response_schema = match oas_version {
        OpenApiVersion::OAS3_0 => {
          let schema_ref = match schema {
            r @ ReferenceOr::Reference { .. } => r,
            ReferenceOr::Object(schema_obj) => {
              let _ref = ReferenceOr::Reference {
                _ref: format!("#/components/schemas/{}", name),
              };
              if let Some(obj) = schema_obj.inner().as_object() {
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
          VersionSpecificSchema::OAS3_0(schema_ref)
        }
        OpenApiVersion::OAS3_1 => match schema {
          ReferenceOr::Object(schema) => VersionSpecificSchema::OAS3_1(schema),
          ReferenceOr::Reference { .. } => {
            log::warn!("Unexpected reference for response schema with oas 3.1");
            VersionSpecificSchema::OAS3_1(ApistosSchema::default())
          }
        },
      };
      responses.push((
        "200".to_owned(),
        ReferenceOr::Object(Response {
          content: BTreeMap::from_iter(vec![(
            content_type.unwrap_or_else(Self::content_type),
            MediaType {
              schema: Some(response_schema),
              ..Default::default()
            },
          )]),
          ..Default::default()
        }),
      ));
    } else if let Some(schema) = Self::raw_schema(oas_version) {
      let schema = match oas_version {
        OpenApiVersion::OAS3_0 => VersionSpecificSchema::OAS3_0(schema),
        OpenApiVersion::OAS3_1 => match schema {
          ReferenceOr::Object(sch) => VersionSpecificSchema::OAS3_1(sch),
          ReferenceOr::Reference { .. } => {
            log::warn!("Unexpected reference for response schema with oas 3.1");
            VersionSpecificSchema::OAS3_1(ApistosSchema::default())
          }
        },
      };
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
      &mut Self::error_responses(oas_version)
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
  use apistos_models::{ApistosSchema, OpenApiVersion};
  use schemars::gen::{SchemaGenerator, SchemaSettings};
  use schemars::JsonSchema;
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
      fn child_schemas(_: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
        vec![]
      }

      fn schema(oas_version: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
        let schema_settings = match oas_version {
          OpenApiVersion::OAS3_0 => SchemaSettings::openapi3(),
          OpenApiVersion::OAS3_1 => SchemaSettings::draft2020_12(),
        };
        let gen = SchemaGenerator::new(schema_settings);
        Some((
          "TestChild".to_string(),
          ApistosSchema::new(gen.into_root_schema_for::<TestChild>(), oas_version).into(),
        ))
      }
    }

    #[derive(JsonSchema)]
    struct Test {
      name: String,
      surname: TestChild,
    }

    impl ApiComponent for Test {
      fn child_schemas(oas_version: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
        <TestChild as ApiComponent>::schema(oas_version).into_iter().collect()
      }

      fn schema(oas_version: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
        let schema_settings = match oas_version {
          OpenApiVersion::OAS3_0 => SchemaSettings::openapi3(),
          OpenApiVersion::OAS3_1 => SchemaSettings::draft2020_12(),
        };
        let gen = SchemaGenerator::new(schema_settings);
        let schema = gen.into_root_schema_for::<Test>();
        Some(("Test".to_string(), ApistosSchema::new(schema, oas_version).into()))
      }
    }

    let schema = <Vec<Test> as ApiComponent>::schema(OpenApiVersion::OAS3_0);
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

    let child_schema = <Vec<Test> as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
    assert_eq!(child_schema.len(), 2);
    let first_child_schema = child_schema.first().cloned();
    assert!(first_child_schema.is_some());

    let json =
      serde_json::to_value(first_child_schema.expect("Missing child schema").1).expect("Unable to serialize as Json");
    assert_json_eq!(
      json,
      json!({
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
