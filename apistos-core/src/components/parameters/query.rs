use crate::ApiComponent;
#[cfg(feature = "query")]
use actix_web::web::Query;
#[cfg(feature = "lab_query")]
use actix_web_lab::extract::Query as LabQuery;
use apistos_models::Schema;
use apistos_models::paths::ParameterStyle;
use apistos_models::paths::{Parameter, ParameterDefinition, ParameterIn, RequestBody};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::{ObjectValidation, SchemaObject};
#[cfg(all(feature = "lab_query", feature = "garde"))]
use garde_actix_web::web::LabQuery as GardeLabQuery;
#[cfg(all(feature = "qs_query", feature = "garde"))]
use garde_actix_web::web::QsQuery as GardeQsQuery;
#[cfg(all(feature = "query", feature = "garde"))]
use garde_actix_web::web::Query as GardeQuery;
#[cfg(feature = "qs_query")]
use serde_qs::actix::QsQuery;
use std::collections::HashMap;

#[allow(unused_macro_rules)]
macro_rules! impl_query {
  ($ty:ident) => {
    impl_query!($ty, hashmap_style: None, style: None, explode: None);
  };
  ($ty:ident, hashmap_style: $hashmap_style:expr) => {
    impl_query!($ty, hashmap_style: $hashmap_style, style: None, explode: None);
  };
  ($ty:ident, style: $style:expr, explode: $explode:expr) => {
    impl_query!($ty, hashmap_style: None, style: $style, explode: $explode);
  };
  ($ty:ident, hashmap_style: $hashmap_style:expr, style: $style:expr, explode: $explode:expr) => {
    impl<T> ApiComponent for $ty<T>
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
        parameters_from_schema(schema, None, &None, &$style, $explode)
      }
    }

    impl<K, V> ApiComponent for $ty<HashMap<K, V>>
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
        parameters_from_hashmap(schema, $hashmap_style)
      }
    }
  };
}

#[cfg(feature = "query")]
impl_query!(Query);
#[cfg(feature = "lab_query")]
impl_query!(LabQuery, style: Some(ParameterStyle::Form), explode: Some(true));
#[cfg(feature = "qs_query")]
impl_query!(QsQuery, hashmap_style: Some(ParameterStyle::DeepObject));
#[cfg(all(feature = "query", feature = "garde"))]
impl_query!(GardeQuery);
#[cfg(all(feature = "qs_query", feature = "garde"))]
impl_query!(GardeQsQuery, hashmap_style: Some(ParameterStyle::DeepObject));
#[cfg(all(feature = "lab_query", feature = "garde"))]
impl_query!(GardeLabQuery, style: Some(ParameterStyle::Form), explode: Some(true));

fn parameters_from_schema(
  schema: Option<ReferenceOr<Schema>>,
  required: Option<bool>,
  default_description: &Option<String>,
  style: &Option<ParameterStyle>,
  explode: Option<bool>,
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
          parameters.append(&mut parameter_for_obj(
            obj,
            &sch,
            required,
            default_description,
            style,
            explode,
          ));
        }
        if let Some(subschema) = &sch.subschemas {
          if let Some(all_of) = &subschema.all_of {
            for sch in all_of {
              parameters.append(&mut parameters_from_schema(
                Some(ReferenceOr::Object(sch.clone())),
                required,
                default_description,
                style,
                explode,
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
                style,
                explode,
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
  style: &Option<ParameterStyle>,
  explode: Option<bool>,
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
        style: style.clone(),
        explode,
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

#[cfg(test)]
mod test {
  use crate::ApiComponent;
  use actix_web::web::Query;
  #[cfg(feature = "lab_query")]
  use actix_web_lab::extract::Query as LabQuery;
  #[cfg(feature = "lab_query")]
  use apistos_models::paths::ParameterStyle;
  use apistos_models::paths::{Parameter, ParameterDefinition, ParameterIn};
  use apistos_models::reference_or::ReferenceOr;
  use schemars::JsonSchema;
  use schemars::schema::{InstanceType, NumberValidation, RootSchema, Schema, SchemaObject, SingleOrVec};
  use serde::{Deserialize, Serialize};
  #[cfg(feature = "qs_query")]
  use serde_qs::actix::QsQuery;

  #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
  struct Test {
    id_number: u32,
    id_string: String,
  }

  impl ApiComponent for Test {
    fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
      vec![]
    }

    fn schema() -> Option<(String, ReferenceOr<Schema>)> {
      let (name, schema) = {
        let schema_name = <Self as JsonSchema>::schema_name();
        let settings = schemars::r#gen::SchemaSettings::openapi3();
        let generator = settings.into_generator();
        let schema: RootSchema = generator.into_root_schema_for::<Self>();
        (schema_name, ReferenceOr::Object(Schema::Object(schema.schema)))
      };
      Some((name, schema))
    }
  }

  #[test]
  fn test_query_parameter() {
    let parameters_schema = <Query<Test> as ApiComponent>::parameters();
    assert_eq!(parameters_schema.len(), 2);

    let id_number_parameter_schema = parameters_schema
      .iter()
      .find(|ps| ps.name == *"id_number")
      .unwrap()
      .clone();
    assert_eq!(
      id_number_parameter_schema,
      Parameter {
        name: "id_number".to_string(),
        _in: ParameterIn::Query,
        required: Some(true),
        definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
          SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Integer))),
            format: Some("uint32".to_string()),
            number: Some(Box::new(NumberValidation {
              minimum: Some(0.0),
              ..Default::default()
            })),
            ..Default::default()
          }
        )))),
        ..Default::default()
      }
    );

    let id_string_parameter_schema = parameters_schema
      .iter()
      .find(|ps| ps.name == *"id_string")
      .unwrap()
      .clone();
    assert_eq!(
      id_string_parameter_schema,
      Parameter {
        name: "id_string".to_string(),
        _in: ParameterIn::Query,
        required: Some(true),
        definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
          SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            ..Default::default()
          }
        )))),
        ..Default::default()
      }
    );
  }

  #[cfg(feature = "qs_query")]
  #[test]
  fn test_qs_query_parameter() {
    let parameters_schema = <QsQuery<Test> as ApiComponent>::parameters();
    assert_eq!(parameters_schema.len(), 2);

    let id_number_parameter_schema = parameters_schema
      .iter()
      .find(|ps| ps.name == *"id_number")
      .unwrap()
      .clone();
    assert_eq!(
      id_number_parameter_schema,
      Parameter {
        name: "id_number".to_string(),
        _in: ParameterIn::Query,
        required: Some(true),
        definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
          SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Integer))),
            format: Some("uint32".to_string()),
            number: Some(Box::new(NumberValidation {
              minimum: Some(0.0),
              ..Default::default()
            })),
            ..Default::default()
          }
        )))),
        ..Default::default()
      }
    );

    let id_string_parameter_schema = parameters_schema
      .iter()
      .find(|ps| ps.name == *"id_string")
      .unwrap()
      .clone();
    assert_eq!(
      id_string_parameter_schema,
      Parameter {
        name: "id_string".to_string(),
        _in: ParameterIn::Query,
        required: Some(true),
        definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
          SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            ..Default::default()
          }
        )))),
        ..Default::default()
      }
    );
  }

  #[cfg(feature = "lab_query")]
  #[test]
  fn test_lab_query_parameter() {
    let parameters_schema = <LabQuery<Test> as ApiComponent>::parameters();
    assert_eq!(parameters_schema.len(), 2);

    let id_number_parameter_schema = parameters_schema
      .iter()
      .find(|ps| ps.name == *"id_number")
      .unwrap()
      .clone();
    assert_eq!(
      id_number_parameter_schema,
      Parameter {
        name: "id_number".to_string(),
        _in: ParameterIn::Query,
        required: Some(true),
        style: Some(ParameterStyle::Form),
        explode: Some(true),
        definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
          SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Integer))),
            format: Some("uint32".to_string()),
            number: Some(Box::new(NumberValidation {
              minimum: Some(0.0),
              ..Default::default()
            })),
            ..Default::default()
          }
        )))),
        ..Default::default()
      }
    );

    let id_string_parameter_schema = parameters_schema
      .iter()
      .find(|ps| ps.name == *"id_string")
      .unwrap()
      .clone();
    assert_eq!(
      id_string_parameter_schema,
      Parameter {
        name: "id_string".to_string(),
        _in: ParameterIn::Query,
        required: Some(true),
        style: Some(ParameterStyle::Form),
        explode: Some(true),
        definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
          SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            ..Default::default()
          }
        )))),
        ..Default::default()
      }
    );
  }
}
