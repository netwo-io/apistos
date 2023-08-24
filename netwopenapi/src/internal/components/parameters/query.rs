use crate::ApiComponent;
use actix_web::web::Query;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{RefOr, Required, Schema};

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
    T::schema()
  }

  fn parameters() -> Vec<Parameter> {
    let mut parameters = vec![];
    let schema = Self::schema().map(|(_, sch)| sch).or_else(|| Self::raw_schema());

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
