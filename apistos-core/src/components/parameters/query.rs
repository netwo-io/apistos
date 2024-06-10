use std::collections::HashMap;

#[cfg(feature = "query")]
use actix_web::web::Query;
#[cfg(all(feature = "lab_query", feature = "garde"))]
use garde_actix_web::web::LabQuery as GardeLabQuery;
#[cfg(all(feature = "qs_query", feature = "garde"))]
use garde_actix_web::web::QsQuery as GardeQsQuery;
#[cfg(all(feature = "query", feature = "garde"))]
use garde_actix_web::web::Query as GardeQuery;
use schemars::Schema;
use serde_json::{json, Map, Value};
#[cfg(feature = "qs_query")]
use serde_qs::actix::QsQuery;

#[cfg(feature = "lab_query")]
use actix_web_lab::extract::Query as LabQuery;
use apistos_models::paths::ParameterStyle;
use apistos_models::paths::{Parameter, ParameterDefinition, ParameterIn, RequestBody};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::{ApistosSchema, OpenApiVersion};

use crate::ApiComponent;

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

      fn child_schemas(oas_version: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
        T::child_schemas(oas_version)
      }

      fn raw_schema(oas_version: OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
        T::raw_schema(oas_version)
      }

      fn schema(_: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
        None
      }

      fn request_body(_: OpenApiVersion) -> Option<RequestBody> {
        None
      }

      fn parameters(oas_version: OpenApiVersion) -> Vec<Parameter> {
        let schema = T::schema(oas_version).map(|(_, sch)| sch).or_else(|| Self::raw_schema(oas_version));
        parameters_from_schema(oas_version, schema, None, &None, &$style, $explode)
      }
    }

    impl<K, V> ApiComponent for $ty<HashMap<K, V>>
    where
      V: ApiComponent,
    {
      fn required() -> bool {
        false
      }

      fn child_schemas(oas_version: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
        V::child_schemas(oas_version)
      }

      fn raw_schema(oas_version: OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
        V::raw_schema(oas_version)
      }

      fn schema(_: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
        None
      }

      fn request_body(_: OpenApiVersion) -> Option<RequestBody> {
        None
      }

      fn parameters(oas_version: OpenApiVersion) -> Vec<Parameter> {
        let schema = V::schema(oas_version).map(|(_, sch)| sch).or_else(|| Self::raw_schema(oas_version));
        parameters_from_hashmap(oas_version, schema, $hashmap_style)
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
  oas_version: OpenApiVersion,
  schema: Option<ReferenceOr<ApistosSchema>>,
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
      ReferenceOr::Object(mut schema) => {
        let sch = schema.inner_mut().as_object_mut();
        if let Some(obj) = sch {
          parameters.append(&mut parameter_for_obj(
            oas_version,
            obj,
            required,
            default_description,
            style,
            explode,
          ));
          if let Some(all_of) = obj.get("allOf").and_then(|v| v.as_array()) {
            for sch in all_of {
              parameters.append(&mut parameters_from_schema(
                oas_version,
                Some(
                  Schema::try_from(sch.clone())
                    .map_err(|err| {
                      log::warn!("Error generating json schema: {err:?}");
                      err
                    })
                    .map(|sch| ApistosSchema::new(sch, oas_version))
                    .unwrap_or_default()
                    .into(),
                ),
                required,
                default_description,
                style,
                explode,
              ));
            }
          }
          if let Some(one_of) = obj.get("oneOf").and_then(|v| v.as_array()) {
            let mut properties = vec![];
            for one_of_sch in one_of {
              if let Some(obj) = one_of_sch.clone().as_object() {
                if let Some(props) = obj.get("properties").and_then(|v| v.as_object()) {
                  props.iter().for_each(|(name, _)| properties.push(name.clone()))
                }
              }
            }
            let description = format!("{} are mutually exclusive properties", properties.join(", "));
            for one_of_sch in one_of {
              parameters.append(&mut parameters_from_schema(
                oas_version,
                Some(
                  Schema::try_from(one_of_sch.clone())
                    .map_err(|err| {
                      log::warn!("Error generating json schema: {err:?}");
                      err
                    })
                    .map(|sch| ApistosSchema::new(sch, oas_version))
                    .unwrap_or_default()
                    .into(),
                ),
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

fn parameters_from_hashmap(
  oas_version: OpenApiVersion,
  schema: Option<ReferenceOr<ApistosSchema>>,
  style: Option<ParameterStyle>,
) -> Vec<Parameter> {
  let parameters;
  if let Some(schema) = schema {
    match schema {
      ReferenceOr::Reference { .. } => {
        parameters = vec![Parameter {
          name: "params".to_string(),
          _in: ParameterIn::Query,
          definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(
            ApistosSchema::default(),
          ))),
          ..Default::default()
        }];
      }
      ReferenceOr::Object(schema) => {
        parameters = vec![Parameter {
          name: "params".to_string(),
          _in: ParameterIn::Query,
          style,
          definition: Some(ParameterDefinition::Schema(
            Schema::try_from(json!({
              "type": "object",
              "additionalProperties": schema
            }))
            .map_err(|err| {
              log::warn!("Error generating json schema: {err:?}");
              err
            })
            .map(|sch| ApistosSchema::new(sch, oas_version))
            .unwrap_or_default()
            .into(),
          )),
          ..Default::default()
        }];
      }
    }
  } else {
    parameters = vec![Parameter {
      name: "params".to_string(),
      _in: ParameterIn::Query,
      style,
      definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(
        ApistosSchema::default(),
      ))),
      ..Default::default()
    }];
  }
  parameters
}

fn parameter_for_obj(
  oas_version: OpenApiVersion,
  obj: &mut Map<String, Value>,
  required: Option<bool>,
  default_description: &Option<String>,
  style: &Option<ParameterStyle>,
  explode: Option<bool>,
) -> Vec<Parameter> {
  if let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) {
    properties
      .clone()
      .into_iter()
      .map(|(name, schema)| {
        let required = required.or_else(|| extract_required_from_schema(obj, &name));
        let description = schema
          .clone()
          .as_object()
          .and_then(|v| v.get("description").and_then(|v| v.as_str().map(|v| v.to_string())))
          .or_else(|| default_description.clone());
        Parameter {
          name,
          _in: ParameterIn::Query,
          definition: Some(ParameterDefinition::Schema(
            ApistosSchema::from_value(schema, oas_version).into(),
          )),
          required,
          description,
          style: style.clone(),
          explode,
          ..Default::default()
        }
      })
      .collect()
  } else {
    vec![]
  }
}

fn extract_required_from_schema(sch_props: &Map<String, Value>, property_name: &str) -> Option<bool> {
  let obj = sch_props;
  if let Some(required) = obj.get("required").and_then(|v| v.as_array()) {
    for ri in required {
      if let Some(required_property_name) = ri.as_str() {
        if required_property_name == property_name {
          return Some(true);
        }
      }
    }
  }
  if obj.get("allOf").is_some() || obj.get("oneOf").is_some() || obj.get("anyOf").is_some() {
    return None;
  }
  if let Some(_type) = obj.get("type") {
    match _type {
      Value::String(string) if string == "array" || string == "string" || string == "number" || string == "integer" => {
        None
      }
      _ => Some(false),
    }
  } else if let Some(_ref) = obj.get("$ref") {
    None
  } else {
    Some(false)
  }
}

#[cfg(test)]
mod test {
  use actix_web::web::Query;
  use schemars::{json_schema, JsonSchema};
  use serde::{Deserialize, Serialize};
  #[cfg(feature = "qs_query")]
  use serde_qs::actix::QsQuery;

  #[cfg(feature = "lab_query")]
  use actix_web_lab::extract::Query as LabQuery;
  #[cfg(feature = "lab_query")]
  use apistos_models::paths::ParameterStyle;
  use apistos_models::paths::{Parameter, ParameterDefinition, ParameterIn};
  use apistos_models::reference_or::ReferenceOr;
  use apistos_models::{ApistosSchema, OpenApiVersion};

  use crate::ApiComponent;

  #[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
  struct Test {
    id_number: u32,
    id_string: String,
  }

  impl ApiComponent for Test {
    fn child_schemas(_: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
      vec![]
    }

    fn schema(oas_version: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
      let (name, schema) = {
        let schema_name = <Self as JsonSchema>::schema_name().to_string();
        let gen = match oas_version {
          OpenApiVersion::OAS3_0 => schemars::gen::SchemaSettings::openapi3().into_generator(),
          OpenApiVersion::OAS3_1 => schemars::gen::SchemaSettings::draft2020_12().into_generator(),
        };
        let schema = gen.into_root_schema_for::<Self>();
        (schema_name, ApistosSchema::new(schema, oas_version).into())
      };
      Some((name, schema))
    }
  }

  #[test]
  fn test_query_parameter() {
    let parameters_schema = <Query<Test> as ApiComponent>::parameters(OpenApiVersion::OAS3_0);
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
        definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(ApistosSchema::new(
          json_schema!({
            "type": "integer",
            "format": "uint32",
            "minimum": 0
          }),
          OpenApiVersion::OAS3_0
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
        definition: Some(ParameterDefinition::Schema(ReferenceOr::Object(
          json_schema!({
            "type": "string",
          })
          .into()
        ))),
        ..Default::default()
      }
    );
  }

  #[cfg(feature = "qs_query")]
  #[test]
  fn test_qs_query_parameter() {
    let parameters_schema = <QsQuery<Test> as ApiComponent>::parameters(OpenApiVersion::OAS3_0);
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
        definition: Some(ParameterDefinition::Schema(
          json_schema!({
            "type": "integer",
            "format": "uint32",
            "minimum": 0
          })
          .into()
        )),
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
        definition: Some(ParameterDefinition::Schema(
          json_schema!({
            "type": "string",
          })
          .into()
        )),
        ..Default::default()
      }
    );
  }

  #[cfg(feature = "lab_query")]
  #[test]
  fn test_lab_query_parameter() {
    let parameters_schema = <LabQuery<Test> as ApiComponent>::parameters(OpenApiVersion::OAS3_0);
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
        definition: Some(ParameterDefinition::Schema(
          json_schema!({
            "type": "integer",
            "format": "uint32",
            "minimum": 0
          })
          .into()
        )),
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
        definition: Some(ParameterDefinition::Schema(
          json_schema!({
            "type": "string",
          })
          .into()
        )),
        ..Default::default()
      }
    );
  }
}
