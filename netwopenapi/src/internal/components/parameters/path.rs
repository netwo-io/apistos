use crate::ApiComponent;
use actix_web::web::Path;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::request_body::RequestBody;
use utoipa::openapi::{Object, RefOr, Required, Schema};

impl<T> ApiComponent for Path<T>
where
  T: ApiComponent,
{
  // always required in Path
  fn required() -> Required {
    Required::True
  }

  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    None
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
          parameters = gen_simple_path_parameter(_ref, Self::required());
        }
        RefOr::T(schema) => match &schema {
          Schema::Object(obj) => {
            parameters = gen_path_parameter_for_object(&schema, obj, Self::required());
          }
          Schema::OneOf(_) => {
            parameters = gen_simple_path_parameter(schema, Self::required());
          }
          Schema::Array(_) | Schema::AllOf(_) | Schema::AnyOf(_) | _ => {
            // these case should never exist right ?
          }
        },
      }
    }

    parameters
  }
}

fn gen_path_parameter_for_object(schema: &Schema, obj: &Object, required: Required) -> Vec<Parameter> {
  if obj.properties.is_empty() {
    gen_simple_path_parameter(schema.clone(), required)
  } else {
    obj
      .properties
      .clone()
      .into_iter()
      .map(|(name, schema)| {
        ParameterBuilder::new()
          .name(name)
          .parameter_in(ParameterIn::Path)
          .schema(Some(schema))
          .required(required.clone())
          .build()
      })
      .collect()
  }
}

fn gen_simple_path_parameter<I: Into<RefOr<Schema>>>(component: I, required: Required) -> Vec<Parameter> {
  vec![ParameterBuilder::new()
    .name("") // this name is overridden later because it is contained in the path
    .parameter_in(ParameterIn::Path)
    .schema(Some(component))
    .required(required)
    .build()]
}
