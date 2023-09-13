use netwopenapi_models::Schema::{AnyOf, AnyOfBuilder};
use netwopenapi_models::{ArrayValidation, InstanceType, ObjectValidation, SchemaObject, SingleOrVec};
use serde_json::Value;
use utoipa::openapi::{
  AllOf, AllOfBuilder, Array, ArrayBuilder, Deprecated, Object, ObjectBuilder, OneOf, OneOfBuilder, Ref, ReferenceOr,
  Schema, SchemaFormat, SchemaType,
};

pub fn json_schema_to_schemas(mut sch: SchemaObject) -> ReferenceOr<Schema> {
  if sch.is_ref() {
    return ReferenceOr::Ref(Ref::new(&sch.reference.unwrap_or_default()));
  }
  let mut schemas: Vec<Schema> = vec![];

  if let Some(subschemas) = sch.subschemas.clone() {
    if let Some(all_of) = subschemas.all_of {
      schemas.push(Schema::AllOf(schema_object_to_all_of(&mut sch, all_of)))
    }
    if let Some(any_of) = subschemas.any_of {
      schemas.push(Schema::AnyOf(schema_object_to_any_of(&mut sch, any_of)))
    }
    if let Some(one_of) = subschemas.one_of {
      schemas.push(Schema::OneOf(schema_object_to_one_of(&mut sch, one_of)))
    }
  }
  if let Some(obj) = sch.object.clone() {
    schemas.push(Schema::Object(schema_object_to_object(&mut sch, obj)))
  }
  if let Some(arr) = sch.array.clone() {
    schemas.push(Schema::Array(schema_object_to_array(&mut sch, arr)))
  }
  if let Some(enum_vals) = sch.enum_values.clone() {
    schemas.push(Schema::Object(enum_value_to_object(&mut sch, enum_vals)))
  }
  if let Some(instance_type) = sch.instance_type.clone() {
    if schemas.is_empty() {
      schemas.push(Schema::Object(type_to_object(&mut sch, instance_type)))
    }
  }

  if schemas.len() > 1 {
    Schema::AllOf(merge_schemas_to_all_of(&mut sch, schemas)).into()
  } else if let Some(schema) = schemas.first() {
    schema.clone().into()
  } else {
    panic!("Unprocessable schema")
  }
}

fn type_to_object(schema: &mut SchemaObject, instance_type: SingleOrVec<InstanceType>) -> Object {
  let schema_type = match instance_type {
    SingleOrVec::Single(instance_type) => instance_type_to_schema_type(*instance_type),
    SingleOrVec::Vec(v) => instance_type_to_schema_type(v.first().cloned().unwrap_or(InstanceType::Null)),
  };

  let mut obj_builder = ObjectBuilder::new().schema_type(schema_type);

  if let Some(format) = schema.format.clone() {
    obj_builder = obj_builder.format(Some(SchemaFormat::Custom(format)));
  }
  obj_builder.build()
}

fn enum_value_to_object(schema: &mut SchemaObject, enum_vals: Vec<Value>) -> Object {
  let schema_type = match schema
    .instance_type
    .clone()
    .unwrap_or(SingleOrVec::Single(Box::new(InstanceType::Object)))
  {
    SingleOrVec::Single(instance_type) => instance_type_to_schema_type(*instance_type),
    SingleOrVec::Vec(v) => instance_type_to_schema_type(v.first().cloned().unwrap_or(InstanceType::Null)),
  };

  let mut obj_builder = ObjectBuilder::new()
    .enum_values(Some(enum_vals))
    .schema_type(schema_type);

  if let Some(format) = schema.format.clone() {
    obj_builder = obj_builder.format(Some(SchemaFormat::Custom(format)));
  }
  obj_builder.build()
}

fn schema_object_to_one_of(schema: &mut SchemaObject, one_of: Vec<netwopenapi_models::Schema>) -> OneOf {
  let schema_metadata = schema.metadata();
  let mut one_of_builder = OneOfBuilder::new()
    .title(schema_metadata.title.clone())
    .description(schema_metadata.description.clone())
    .default(schema_metadata.default.clone())
    .example(schema_metadata.examples.first().cloned());

  for s in one_of {
    one_of_builder = one_of_builder.item(json_schema_to_schemas(s.into_object()));
  }

  one_of_builder.build()
}

fn schema_object_to_any_of(schema: &mut SchemaObject, any_of: Vec<netwopenapi_models::Schema>) -> AnyOf {
  let schema_metadata = schema.metadata();
  let mut any_of_builder = AnyOfBuilder::new()
    .description(schema_metadata.description.clone())
    .default(schema_metadata.default.clone())
    .example(schema_metadata.examples.first().cloned());

  for s in any_of {
    any_of_builder = any_of_builder.item(json_schema_to_schemas(s.into_object()));
  }

  any_of_builder.build()
}

fn schema_object_to_all_of(schema: &mut SchemaObject, all_of: Vec<netwopenapi_models::Schema>) -> AllOf {
  let schema_metadata = schema.metadata();
  let mut all_of_builder = AllOfBuilder::new()
    .title(schema_metadata.title.clone())
    .description(schema_metadata.description.clone())
    .default(schema_metadata.default.clone())
    .example(schema_metadata.examples.first().cloned());

  for s in all_of {
    all_of_builder = all_of_builder.item(json_schema_to_schemas(s.into_object()));
  }

  all_of_builder.build()
}

fn schema_object_to_array(schema: &mut SchemaObject, arr: Box<ArrayValidation>) -> Array {
  let schema_metadata = schema.metadata();
  let mut arr_builder = ArrayBuilder::new()
    .title(schema_metadata.title.clone())
    .description(schema_metadata.description.clone())
    .default(schema_metadata.default.clone())
    .deprecated(if schema_metadata.deprecated {
      Some(Deprecated::True)
    } else {
      None
    })
    .example(schema_metadata.examples.first().cloned());

  if let Some(items) = arr.items {
    match items {
      SingleOrVec::Single(s) => arr_builder = arr_builder.items(json_schema_to_schemas(s.into_object())),
      SingleOrVec::Vec(v) => {
        if let Some(s) = v.first().cloned() {
          arr_builder = arr_builder.items(json_schema_to_schemas(s.into_object()))
        }
      }
    }
  }

  arr_builder.build()
}

fn schema_object_to_object(schema: &mut SchemaObject, obj: Box<ObjectValidation>) -> Object {
  let schema_metadata = schema.metadata();
  let mut obj_builder = ObjectBuilder::new()
    .title(schema_metadata.title.clone())
    .description(schema_metadata.description.clone())
    .default(schema_metadata.default.clone())
    .deprecated(if schema_metadata.deprecated {
      Some(Deprecated::True)
    } else {
      None
    })
    .example(schema_metadata.examples.first().cloned());

  for r in obj.required {
    obj_builder = obj_builder.required(r);
  }

  obj_builder = obj_builder.format(schema.format.clone().map(schema_format_from_string));
  obj_builder = obj_builder.schema_type(SchemaType::Object);

  for (p_name, p_schema) in obj.properties {
    obj_builder = obj_builder.property(p_name, json_schema_to_schemas(p_schema.into_object()));
  }

  obj_builder.build()
}

fn schema_format_from_string(schema_format_string: String) -> SchemaFormat {
  SchemaFormat::Custom(schema_format_string)
}

fn instance_type_to_schema_type(instance_type: InstanceType) -> SchemaType {
  match instance_type {
    InstanceType::Null => SchemaType::Value,
    InstanceType::Boolean => SchemaType::Boolean,
    InstanceType::Object => SchemaType::Object,
    InstanceType::Array => SchemaType::Array,
    InstanceType::Number => SchemaType::Number,
    InstanceType::String => SchemaType::String,
    InstanceType::Integer => SchemaType::Integer,
  }
}

fn merge_schemas_to_all_of(schema: &mut SchemaObject, schemas: Vec<Schema>) -> AllOf {
  let schema_metadata = schema.metadata();
  let mut all_of_builder = AllOfBuilder::new()
    .title(schema_metadata.title.clone())
    .description(schema_metadata.description.clone())
    .default(schema_metadata.default.clone())
    .example(schema_metadata.examples.first().cloned());

  for s in schemas {
    all_of_builder = all_of_builder.item(s);
  }

  all_of_builder.build()
}
