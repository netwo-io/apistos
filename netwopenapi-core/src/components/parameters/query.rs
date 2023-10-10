use crate::ApiComponent;
#[cfg(feature = "query")]
use actix_web::web::Query;
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
    let schema = T::schema().map(|(_, sch)| sch).or_else(Self::raw_schema);
    parameters_from_schema(schema, None, &None)
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
    let schema = V::schema().map(|(_, sch)| sch).or_else(Self::raw_schema);
    parameters_from_hashmap(schema, None)
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
    let schema = T::schema().map(|(_, sch)| sch).or_else(Self::raw_schema);
    parameters_from_schema(schema, None, &None)
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
    let schema = V::schema().map(|(_, sch)| sch).or_else(Self::raw_schema);
    parameters_from_hashmap(schema, Some(ParameterStyle::DeepObject))
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
    let schema = T::schema().map(|(_, sch)| sch).or_else(Self::raw_schema);
    parameters_from_schema(schema, None, &None)
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
    let schema = V::schema().map(|(_, sch)| sch).or_else(Self::raw_schema);
    parameters_from_hashmap(schema, None)
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
    let schema = T::schema().map(|(_, sch)| sch).or_else(Self::raw_schema);
    parameters_from_schema(schema, None, &None)
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
    let schema = V::schema().map(|(_, sch)| sch).or_else(Self::raw_schema);
    parameters_from_hashmap(schema, Some(ParameterStyle::DeepObject))
  }
}

fn parameters_from_schema(
  schema: Option<ReferenceOr<Schema>>,
  required: Option<bool>,
  default_description: &Option<String>,
) -> Vec<Parameter> {
  let mut parameters = vec![];
  if let Some(schema) = schema {
    match schema {
      ReferenceOr::Reference { _ref } => {
        // don't know what to do with it
      }
      ReferenceOr::Object(schema) => {
        let sch = schema.into_object();
        if let Some(obj) = &sch.object {
          parameters.append(&mut parameter_for_obj(obj, &sch, required, default_description));
        }
        if let Some(subschema) = &sch.subschemas {
          if let Some(all_of) = &subschema.all_of {
            for sch in all_of {
              parameters.append(&mut parameters_from_schema(
                Some(ReferenceOr::Object(sch.clone())),
                required,
                default_description,
              ));
            }
          }
          if let Some(one_of) = &subschema.one_of {
            let mut properties = vec![];
            for one_of_sch in one_of {
              if let Some(obj) = one_of_sch.clone().into_object().object {
                obj
                  .properties
                  .iter()
                  .for_each(|(name, _)| properties.push(name.clone()))
              }
            }
            let description = format!("{} are mutually exclusive properties", properties.join(", "));
            for one_of_sch in one_of {
              parameters.append(&mut parameters_from_schema(
                Some(ReferenceOr::Object(one_of_sch.clone())),
                Some(false),
                &Some(description.clone()),
              ));
            }
          }
        }
      }
    }
  }
  parameters
}

fn parameters_from_hashmap(schema: Option<ReferenceOr<Schema>>, style: Option<ParameterStyle>) -> Vec<Parameter> {
  let parameters;
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
          style,
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
      style,
      definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
        SchemaObject::default(),
      )))),
      ..Default::default()
    }];
  }
  parameters
}

fn parameter_for_obj(
  obj: &ObjectValidation,
  sch: &SchemaObject,
  required: Option<bool>,
  default_description: &Option<String>,
) -> Vec<Parameter> {
  obj
    .properties
    .clone()
    .into_iter()
    .map(|(name, schema)| {
      let required = required.or_else(|| extract_required_from_schema(sch, &name));
      let description = schema
        .clone()
        .into_object()
        .metadata
        .and_then(|m| m.description)
        .or_else(|| default_description.clone());
      Parameter {
        name,
        _in: ParameterIn::Query,
        definition: Some(ParameterDefinition::Schema(schema.into())),
        required,
        description,
        ..Default::default()
      }
    })
    .collect()
}

fn extract_required_from_schema(sch_obj: &SchemaObject, property_name: &str) -> Option<bool> {
  if let Some(obj) = &sch_obj.object {
    for ri in &obj.required {
      if ri.clone() == *property_name {
        return Some(true);
      }
    }
  }
  if sch_obj.subschemas.is_some()
    || sch_obj.string.is_some()
    || sch_obj.number.is_some()
    || sch_obj.array.is_some()
    || sch_obj.reference.is_some()
  {
    return None;
  }
  Some(false)
}
