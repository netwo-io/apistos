use crate::ApiComponent;
#[cfg(feature = "query")]
use actix_web::web::Query;
#[cfg(feature = "qs_query")]
use serde_qs::actix::QsQuery;
use std::collections::HashMap;
#[cfg(feature = "qs_query")]
use utoipa::openapi::path::ParameterStyle;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::request_body::RequestBody;
use utoipa::openapi::{Object, ObjectBuilder, RefOr, Required, Schema};

#[cfg(feature = "query")]
impl<T> ApiComponent for Query<T>
where
  T: ApiComponent,
{
  fn required() -> Required {
    T::required()
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
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
        RefOr::Ref(_ref) => {
          // don't know what to do with it
        }
        RefOr::T(schema) => match &schema {
          Schema::Object(obj) => {
            parameters = obj
              .properties
              .clone()
              .into_iter()
              .map(|(name, schema)| {
                ParameterBuilder::new()
                  .name(name)
                  .parameter_in(ParameterIn::Query)
                  .schema(Some(schema))
                  .required(Self::required().clone())
                  .build()
              })
              .collect()
          }
          Schema::OneOf(_) | Schema::Array(_) | Schema::AllOf(_) | Schema::AnyOf(_) | _ => {
            // these case should never exist right ? (no key names)
          }
        },
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
  fn required() -> Required {
    Required::False
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    V::child_schemas()
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    V::raw_schema()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    None
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    let parameters;
    let schema = V::schema().map(|(_, sch)| sch).or_else(|| Self::raw_schema());
    if let Some(schema) = schema {
      parameters = vec![ParameterBuilder::new()
        .name("params")
        .parameter_in(ParameterIn::Query)
        .schema(Some(ObjectBuilder::new().additional_properties(Some(schema)).build()))
        .build()];
    } else {
      parameters = vec![ParameterBuilder::new()
        .name("params")
        .parameter_in(ParameterIn::Query)
        .schema(Some(Object::default()))
        .build()];
    }

    parameters
  }
}

#[cfg(feature = "qs_query")]
impl<T> ApiComponent for QsQuery<T>
where
  T: ApiComponent,
{
  fn required() -> Required {
    T::required()
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    T::child_schemas()
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    T::raw_schema()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
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
        RefOr::Ref(_ref) => {
          // don't know what to do with it
        }
        RefOr::T(schema) => match &schema {
          Schema::Object(obj) => {
            parameters = obj
              .properties
              .clone()
              .into_iter()
              .map(|(name, schema)| {
                ParameterBuilder::new()
                  .name(name)
                  .parameter_in(ParameterIn::Query)
                  .schema(Some(schema))
                  .required(Self::required().clone())
                  .style(Some(ParameterStyle::DeepObject))
                  .build()
              })
              .collect()
          }
          Schema::OneOf(_) | Schema::Array(_) | Schema::AllOf(_) | Schema::AnyOf(_) | _ => {
            // these case should never exist right ? (no key names)
          }
        },
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
  fn required() -> Required {
    Required::False
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    V::child_schemas()
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    V::raw_schema()
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    None
  }

  fn request_body() -> Option<RequestBody> {
    None
  }

  fn parameters() -> Vec<Parameter> {
    let mut parameters = vec![];
    let schema = V::schema().map(|(_, sch)| sch).or_else(|| Self::raw_schema());
    if let Some(schema) = schema {
      parameters = vec![ParameterBuilder::new()
        .name("params")
        .parameter_in(ParameterIn::Query)
        .style(Some(ParameterStyle::DeepObject))
        .schema(Some(ObjectBuilder::new().additional_properties(Some(schema)).build()))
        .build()];
    } else {
      parameters = vec![ParameterBuilder::new()
        .name("params")
        .parameter_in(ParameterIn::Query)
        .style(Some(ParameterStyle::DeepObject))
        .schema(Some(Object::default()))
        .build()];
    }

    parameters
  }
}
