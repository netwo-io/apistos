use crate::ApiComponent;
use actix_web::web::Path;
use apistos_models::ObjectValidation;
use apistos_models::Schema;
use apistos_models::paths::{Parameter, ParameterDefinition, ParameterIn, RequestBody};
use apistos_models::reference_or::ReferenceOr;
use schemars::schema::{InstanceType, SingleOrVec};

impl<T> ApiComponent for Path<T>
where
  T: ApiComponent,
{
  // always required in Path
  fn required() -> bool {
    true
  }

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    vec![]
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

    if let Some(schema) = schema {
      parameters_for_schema(schema, Self::required())
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

  fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
    vec![]
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
    if let Some(schema) = schema {
      parameters_for_schema(schema, Self::required())
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

    fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
      vec![]
    }

    fn raw_schema() -> Option<ReferenceOr<Schema>> {
      None
    }

    fn schema() -> Option<(String, ReferenceOr<Schema>)> {
      None
    }

    fn request_body() -> Option<RequestBody> {
      None
    }

    fn parameters() -> Vec<Parameter> {
      let mut parameters = vec![];
      $(
        let schema = $ty::schema().map(|(_, sch)| sch).or_else($ty::raw_schema);

        if let Some(schema) = schema {
          parameters.append(&mut parameters_for_schema(schema, Self::required()));
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

    fn child_schemas() -> Vec<(String, ReferenceOr<Schema>)> {
      vec![]
    }

    fn raw_schema() -> Option<ReferenceOr<Schema>> {
      None
    }

    fn schema() -> Option<(String, ReferenceOr<Schema>)> {
      None
    }

    fn request_body() -> Option<RequestBody> {
      None
    }

    fn parameters() -> Vec<Parameter> {
      let mut parameters = vec![];
      $(
        let schema = $ty::schema().map(|(_, sch)| sch).or_else($ty::raw_schema);

        if let Some(schema) = schema {
          parameters.append(&mut parameters_for_schema(schema, Self::required()));
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

fn parameters_for_schema(schema: ReferenceOr<Schema>, required: bool) -> Vec<Parameter> {
  let mut parameters = vec![];

  match schema {
    r @ ReferenceOr::Reference { .. } => {
      parameters.push(gen_simple_path_parameter(r, required));
    }
    ReferenceOr::Object(schema) => {
      let sch = schema.clone().into_object();
      if let Some(subschemas) = sch.subschemas {
        // any_of and one_of should not exists for path ?
        if let Some(all_of) = subschemas.all_of {
          for schema in all_of {
            parameters.append(&mut parameters_for_schema(schema.into(), required));
          }
        }
      }
      if let Some(obj) = sch.object.clone() {
        parameters.append(&mut gen_path_parameter_for_object(&schema, &obj, required));
      }
      if let Some(instance_type) = sch.instance_type.clone() {
        let processable_instance_type = match instance_type {
          SingleOrVec::Single(it) => processable_instance_type(*it),
          SingleOrVec::Vec(its) => its.first().map(|it| processable_instance_type(*it)).unwrap_or_default(),
        };
        if processable_instance_type {
          parameters.push(gen_simple_path_parameter(schema.into(), required));
        }
      }
    }
  }

  parameters
}

fn gen_path_parameter_for_object(schema: &Schema, obj: &ObjectValidation, required: bool) -> Vec<Parameter> {
  if obj.properties.is_empty() {
    vec![gen_simple_path_parameter(schema.clone().into(), required)]
  } else {
    obj
      .properties
      .clone()
      .into_iter()
      .map(|(name, schema)| Parameter {
        name,
        _in: ParameterIn::Path,
        definition: Some(ParameterDefinition::Schema(schema.into())),
        required: Some(required),
        ..Default::default()
      })
      .collect()
  }
}

fn gen_simple_path_parameter(component: ReferenceOr<Schema>, required: bool) -> Parameter {
  Parameter {
    name: "".to_string(), // this name is overridden later because it is contained in the path
    _in: ParameterIn::Path,
    definition: Some(ParameterDefinition::Schema(component)),
    required: Some(required),
    ..Default::default()
  }
}

fn processable_instance_type(instance_type: InstanceType) -> bool {
  match instance_type {
    InstanceType::Null | InstanceType::Object => false,
    InstanceType::Boolean
    | InstanceType::Array
    | InstanceType::Number
    | InstanceType::String
    | InstanceType::Integer => true,
  }
}
