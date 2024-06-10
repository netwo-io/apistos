use crate::ApiComponent;
use actix_web::web::Path;
use apistos_models::paths::{Parameter, ParameterDefinition, ParameterIn, RequestBody};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::{ApistosSchema, OpenApiVersion};
use schemars::Schema;
use serde_json::{Map, Value};

impl<T> ApiComponent for Path<T>
where
  T: ApiComponent,
{
  // always required in Path
  fn required() -> bool {
    true
  }

  fn child_schemas(_: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    vec![]
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
    let schema = T::schema(oas_version)
      .map(|(_, sch)| sch)
      .or_else(|| Self::raw_schema(oas_version));

    if let Some(schema) = schema {
      parameters_for_schema(oas_version, schema, Self::required())
    } else {
      vec![]
    }
  }
}

#[cfg(feature = "garde")]
impl<T> ApiComponent for garde_actix_web::web::Path<T>
where
  T: ApiComponent,
{
  // always required in Path
  fn required() -> bool {
    true
  }

  fn child_schemas(_: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
    vec![]
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
    let schema = T::schema(oas_version)
      .map(|(_, sch)| sch)
      .or_else(|| Self::raw_schema(oas_version));
    if let Some(schema) = schema {
      parameters_for_schema(oas_version, schema, Self::required())
    } else {
      vec![]
    }
  }
}

macro_rules! impl_path_tuple ({ $($ty:ident),+ } => {
  impl<$($ty,)+> ApiComponent for Path<($($ty,)+)>
  where
    $($ty: ApiComponent,)+
  {
    // always required in Path
    fn required() -> bool {
      true
    }

    fn child_schemas(_: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
      vec![]
    }

    fn raw_schema(_: OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
      None
    }

    fn schema(_: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
      None
    }

    fn request_body(_: OpenApiVersion) -> Option<RequestBody> {
      None
    }

    fn parameters(oas_version: OpenApiVersion) -> Vec<Parameter> {
      let mut parameters = vec![];
      $(
        let schema = $ty::schema(oas_version).map(|(_, sch)| sch).or_else(|| $ty::raw_schema(oas_version));

        if let Some(schema) = schema {
          parameters.append(&mut parameters_for_schema(oas_version, schema, Self::required()));
        }
      )+
      parameters
    }
  }

  #[cfg(feature = "garde")]
  impl<$($ty,)+> ApiComponent for garde_actix_web::web::Path<($($ty,)+)>
  where
    $($ty: ApiComponent,)+
  {

    fn required() -> bool {
      true
    }

    fn child_schemas(_: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
      vec![]
    }

    fn raw_schema(_: OpenApiVersion) -> Option<ReferenceOr<ApistosSchema>> {
      None
    }

    fn schema(_: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
      None
    }

    fn request_body(_: OpenApiVersion) -> Option<RequestBody> {
      None
    }

    fn parameters(oas_version: OpenApiVersion) -> Vec<Parameter> {
      let mut parameters = vec![];
      $(
        let schema = $ty::schema(oas_version).map(|(_, sch)| sch).or_else(|| $ty::raw_schema(oas_version));

        if let Some(schema) = schema {
          parameters.append(&mut parameters_for_schema(oas_version, schema, Self::required()));
        }
      )+
      parameters
    }
  }
});

impl_path_tuple!(A);
impl_path_tuple!(A, B);
impl_path_tuple!(A, B, C);
impl_path_tuple!(A, B, C, D);
impl_path_tuple!(A, B, C, D, E);
impl_path_tuple!(A, B, C, D, E, F);
impl_path_tuple!(A, B, C, D, E, F, G);
impl_path_tuple!(A, B, C, D, E, F, G, H);
impl_path_tuple!(A, B, C, D, E, F, G, H, I);
impl_path_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_path_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_path_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_path_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);

fn parameters_for_schema(
  oas_version: OpenApiVersion,
  schema: ReferenceOr<ApistosSchema>,
  required: bool,
) -> Vec<Parameter> {
  let mut parameters = vec![];

  match schema {
    r @ ReferenceOr::Reference { .. } => {
      parameters.push(gen_simple_path_parameter(r, required));
    }
    ReferenceOr::Object(schema) => {
      let sch = schema.inner().as_object();
      if let Some(obj) = sch {
        // any_of and one_of should not exist for path ?
        if let Some(all_of) = obj.get("allOf").and_then(|v| v.as_array()) {
          for schema_value in all_of {
            parameters.append(&mut parameters_for_schema(
              oas_version,
              Schema::try_from(schema_value.clone())
                .map_err(|err| {
                  log::warn!("Error generating json schema from #ident : {err:?}");
                  err
                })
                .map(|sch| ApistosSchema::new(sch, oas_version))
                .unwrap_or_default()
                .into(),
              required,
            ));
          }
        }

        let _type = obj.get("type");
        if let Some(Value::String(string)) = _type {
          if string == "object" {
            parameters.append(&mut gen_path_parameter_for_object(oas_version, &schema, obj, required))
          } else if processable_instance_type(string) {
            parameters.push(gen_simple_path_parameter(schema.into(), required));
          }
        }
      }
    }
  }

  parameters
}

fn gen_path_parameter_for_object(
  oas_version: OpenApiVersion,
  schema: &ApistosSchema,
  obj: &Map<String, Value>,
  required: bool,
) -> Vec<Parameter> {
  let properties = obj
    .get("properties")
    .and_then(|v| v.as_object())
    .cloned()
    .unwrap_or_default();
  if properties.is_empty() {
    vec![gen_simple_path_parameter(schema.clone().into(), required)]
  } else {
    properties
      .clone()
      .into_iter()
      .map(|(name, schema)| Parameter {
        name,
        _in: ParameterIn::Path,
        definition: Some(ParameterDefinition::Schema(
          ApistosSchema::from_value(schema, oas_version).into(),
        )),
        required: Some(required),
        ..Default::default()
      })
      .collect()
  }
}

fn gen_simple_path_parameter(component: ReferenceOr<ApistosSchema>, required: bool) -> Parameter {
  Parameter {
    name: "".to_string(), // this name is overridden later because it is contained in the path
    _in: ParameterIn::Path,
    definition: Some(ParameterDefinition::Schema(component)),
    required: Some(required),
    ..Default::default()
  }
}

fn processable_instance_type(instance_type: &str) -> bool {
  !(instance_type == "null" || instance_type == "object")
}
