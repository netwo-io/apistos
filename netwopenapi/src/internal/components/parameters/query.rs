use crate::ApiComponent;
#[cfg(feature = "query")]
use actix_web::web::Query;
#[cfg(feature = "qs_query")]
use netwopenapi_models::paths::ParameterStyle;
use netwopenapi_models::paths::{Parameter, ParameterDefinition, ParameterIn, RequestBody};
use netwopenapi_models::reference_or::ReferenceOr;
use netwopenapi_models::Schema;
use netwopenapi_models::{ObjectValidation, SchemaObject};
#[cfg(feature = "qs_query")]
use serde_qs::actix::QsQuery;
use std::collections::HashMap;

#[cfg(feature = "query")]
impl<T> ApiComponent for Query<T>
where
  T: ApiComponent,
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
    None
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    let mut parameters = vec![];
    let schema = T::schema().map(|(_, sch)| sch).or_else(|| Self::raw_schema());
    if let Some(schema) = schema {
      match schema {
        ReferenceOr::Reference { _ref } => {
          // don't know what to do with it
        }
        ReferenceOr::Object(schema) => {
          let sch = schema.into_object();
          if let Some(obj) = sch.object {
            parameters = obj
              .properties
              .clone()
              .into_iter()
              .map(|(name, schema)| Parameter {
                name,
                _in: ParameterIn::Query,
                definition: Some(ParameterDefinition::Schema(schema.into())),
                required: Some(Self::required()),
                ..Default::default()
              })
              .collect()
          }
        }
      }
    }

    parameters
  }
}

#[cfg(feature = "query")]
impl<K, V> ApiComponent for Query<HashMap<K, V>>
where
  V: ApiComponent,
{
  fn required() -> bool {
    false
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    V::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    V::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    let parameters;
    let schema = V::schema().map(|(_, sch)| sch).or_else(|| Self::raw_schema());
    if let Some(schema) = schema {
      match schema {
        ReferenceOr::Reference { .. } => {
          parameters = vec![Parameter {
            name: "params".to_string(),
            _in: ParameterIn::Query,
            definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
              SchemaObject::default(),
            )))),
            ..Default::default()
          }];
        }
        ReferenceOr::Object(schema) => {
          parameters = vec![Parameter {
            name: "params".to_string(),
            _in: ParameterIn::Query,
            definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
              SchemaObject {
                object: Some(Box::new(ObjectValidation {
                  additional_properties: Some(Box::new(schema)),
                  ..Default::default()
                })),
                ..Default::default()
              },
            )))),
            ..Default::default()
          }];
        }
      }
    } else {
      parameters = vec![Parameter {
        name: "params".to_string(),
        _in: ParameterIn::Query,
        definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
          SchemaObject::default(),
        )))),
        ..Default::default()
      }];
    }

    parameters
  }
}

#[cfg(feature = "qs_query")]
impl<T> ApiComponent for QsQuery<T>
where
  T: ApiComponent,
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
    None
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    let mut parameters = vec![];
    let schema = T::schema().map(|(_, sch)| sch).or_else(|| Self::raw_schema());
    if let Some(schema) = schema {
      match schema {
        ReferenceOr::Reference { _ref } => {
          // don't know what to do with it
        }
        ReferenceOr::Object(schema) => {
          let sch = schema.into_object();
          if let Some(obj) = sch.object {
            parameters = obj
              .properties
              .clone()
              .into_iter()
              .map(|(name, schema)| Parameter {
                name,
                _in: ParameterIn::Query,
                definition: Some(ParameterDefinition::Schema(schema.into())),
                style: Some(ParameterStyle::DeepObject),
                required: Some(Self::required()),
                ..Default::default()
              })
              .collect()
          }
        }
      }
    }

    parameters
  }
}

#[cfg(feature = "qs_query")]
impl<K, V> ApiComponent for QsQuery<HashMap<K, V>>
where
  V: ApiComponent,
{
  fn required() -> bool {
    false
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    V::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    V::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    let parameters;
    let schema = V::schema().map(|(_, sch)| sch).or_else(|| Self::raw_schema());
    if let Some(schema) = schema {
      match schema {
        ReferenceOr::Reference { .. } => {
          parameters = vec![Parameter {
            name: "params".to_string(),
            _in: ParameterIn::Query,
            definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
              SchemaObject::default(),
            )))),
            ..Default::default()
          }];
        }
        ReferenceOr::Object(schema) => {
          parameters = vec![Parameter {
            name: "params".to_string(),
            _in: ParameterIn::Query,
            style: Some(ParameterStyle::DeepObject),
            definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
              SchemaObject {
                object: Some(Box::new(ObjectValidation {
                  additional_properties: Some(Box::new(schema)),
                  ..Default::default()
                })),
                ..Default::default()
              },
            )))),
            ..Default::default()
          }];
        }
      }
    } else {
      parameters = vec![Parameter {
        name: "params".to_string(),
        _in: ParameterIn::Query,
        style: Some(ParameterStyle::DeepObject),
        definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
          SchemaObject::default(),
        )))),
        ..Default::default()
      }];
    }

    parameters
  }
}

#[cfg(all(feature = "query", feature = "garde"))]
impl<T> ApiComponent for garde_actix_web::web::Query<T>
where
  T: ApiComponent,
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
    None
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    let mut parameters = vec![];
    let schema = T::schema().map(|(_, sch)| sch).or_else(|| Self::raw_schema());
    if let Some(schema) = schema {
      match schema {
        ReferenceOr::Reference { _ref } => {
          // don't know what to do with it
        }
        ReferenceOr::Object(schema) => {
          let sch = schema.into_object();
          if let Some(obj) = sch.object {
            parameters = obj
              .properties
              .clone()
              .into_iter()
              .map(|(name, schema)| Parameter {
                name,
                _in: ParameterIn::Query,
                definition: Some(ParameterDefinition::Schema(schema.into())),
                required: Some(Self::required()),
                ..Default::default()
              })
              .collect()
          }
        }
      }
    }

    parameters
  }
}

#[cfg(all(feature = "query", feature = "garde"))]
impl<K, V> ApiComponent for garde_actix_web::web::Query<HashMap<K, V>>
where
  V: ApiComponent,
{
  fn required() -> bool {
    false
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    V::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    V::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    let parameters;
    let schema = V::schema().map(|(_, sch)| sch).or_else(|| Self::raw_schema());
    if let Some(schema) = schema {
      match schema {
        ReferenceOr::Reference { .. } => {
          parameters = vec![Parameter {
            name: "params".to_string(),
            _in: ParameterIn::Query,
            definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
              SchemaObject::default(),
            )))),
            ..Default::default()
          }];
        }
        ReferenceOr::Object(schema) => {
          parameters = vec![Parameter {
            name: "params".to_string(),
            _in: ParameterIn::Query,
            definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
              SchemaObject {
                object: Some(Box::new(ObjectValidation {
                  additional_properties: Some(Box::new(schema)),
                  ..Default::default()
                })),
                ..Default::default()
              },
            )))),
            ..Default::default()
          }];
        }
      }
    } else {
      parameters = vec![Parameter {
        name: "params".to_string(),
        _in: ParameterIn::Query,
        definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
          SchemaObject::default(),
        )))),
        ..Default::default()
      }];
    }

    parameters
  }
}

#[cfg(all(feature = "qs_query", feature = "garde"))]
impl<T> ApiComponent for garde_actix_web::web::QsQuery<T>
where
  T: ApiComponent,
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
    None
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    let mut parameters = vec![];
    let schema = T::schema().map(|(_, sch)| sch).or_else(|| Self::raw_schema());
    if let Some(schema) = schema {
      match schema {
        ReferenceOr::Reference { _ref } => {
          // don't know what to do with it
        }
        ReferenceOr::Object(schema) => {
          let sch = schema.into_object();
          if let Some(obj) = sch.object {
            parameters = obj
              .properties
              .clone()
              .into_iter()
              .map(|(name, schema)| Parameter {
                name,
                _in: ParameterIn::Query,
                definition: Some(ParameterDefinition::Schema(schema.into())),
                required: Some(Self::required()),
                ..Default::default()
              })
              .collect()
          }
        }
      }
    }

    parameters
  }
}

#[cfg(all(feature = "qs_query", feature = "garde"))]
impl<K, V> ApiComponent for garde_actix_web::web::QsQuery<HashMap<K, V>>
where
  V: ApiComponent,
{
  fn required() -> bool {
    false
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    V::child_schemas()
  }

  fn raw_schema() -> Option<ReferenceOr<Schema>> {
    V::raw_schema()
  }

  fn schema() -> Option<(String, ReferenceOr<Schema>)> {
    None
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    let parameters;
    let schema = V::schema().map(|(_, sch)| sch).or_else(|| Self::raw_schema());
    if let Some(schema) = schema {
      match schema {
        ReferenceOr::Reference { .. } => {
          parameters = vec![Parameter {
            name: "params".to_string(),
            _in: ParameterIn::Query,
            definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
              SchemaObject::default(),
            )))),
            ..Default::default()
          }];
        }
        ReferenceOr::Object(schema) => {
          parameters = vec![Parameter {
            name: "params".to_string(),
            _in: ParameterIn::Query,
            style: Some(ParameterStyle::DeepObject),
            definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
              SchemaObject {
                object: Some(Box::new(ObjectValidation {
                  additional_properties: Some(Box::new(schema)),
                  ..Default::default()
                })),
                ..Default::default()
              },
            )))),
            ..Default::default()
          }];
        }
      }
    } else {
      parameters = vec![Parameter {
        name: "params".to_string(),
        _in: ParameterIn::Query,
        style: Some(ParameterStyle::DeepObject),
        definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
          SchemaObject::default(),
        )))),
        ..Default::default()
      }];
    }

    parameters
  }
}
