use crate::OpenApiVersion;
use crate::schemars::{Schema, SchemaGenerator};
use log::warn;
use schemars::transform::Transform;
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Debug, Clone)]
pub(crate) struct EnumNewTypeInternallyTaggedEnumWrapRef;

impl Transform for EnumNewTypeInternallyTaggedEnumWrapRef {
  fn transform(&mut self, schema: &mut Schema) {
    let Some(root) = schema.as_object_mut() else { return };

    let mut one_of = match root.remove("oneOf") {
      Some(Value::Array(arr)) => arr,
      _ => vec![],
    };

    for v in &mut one_of {
      if let Some(v) = v.as_object_mut() {
        if let Some(component_ref) = v.remove("$ref") {
          let properties = v.entry("allOf").or_insert_with(|| Value::Array(vec![])).as_array_mut();
          if let Some(properties) = properties {
            properties.push(json!({
              "$ref": component_ref
            }));
          }
        }
      }
    }

    if !one_of.is_empty() {
      root.insert("oneOf".to_string(), Value::Array(one_of));
    }

    let components = root.get_mut("components").and_then(|c| {
      c.as_object_mut()
        .and_then(|c| c.get_mut("schemas").and_then(|s| s.as_object_mut()))
    });

    let components = match components {
      None => &mut serde_json::Map::new(),
      Some(obj) => obj,
    };

    for (_component_name, component) in components {
      let Some(component) = component.as_object_mut() else {
        return;
      };

      let mut one_of = match component.remove("oneOf") {
        Some(Value::Array(arr)) => arr,
        _ => vec![],
      };

      for v in &mut one_of {
        if let Some(v) = v.as_object_mut() {
          if let Some(component_ref) = v.remove("$ref") {
            let properties = v.entry("allOf").or_insert_with(|| Value::Array(vec![])).as_array_mut();
            if let Some(properties) = properties {
              properties.push(json!({
                "$ref": component_ref
              }));
            }
          }
        }
      }

      if !one_of.is_empty() {
        component.insert("oneOf".to_string(), Value::Array(one_of));
      }
    }
  }
}

#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(transparent)]
pub struct ApistosSchema(Schema);

impl ApistosSchema {
  pub fn new(mut schema: Schema, oas_version: OpenApiVersion) -> Self {
    let obj = schema.as_object_mut();

    match obj {
      None => Self(schema),
      Some(obj) => {
        // set title for each one_of if not set
        if let Some(one_of) = obj.get_mut("oneOf").and_then(|v| v.as_array_mut()) {
          Self::set_title_for_enum_variants(one_of);
        }

        // remove definitions from schema
        Self::remove_definition_from_schema(obj, &oas_version.get_schema_settings().into_generator());
        match oas_version {
          OpenApiVersion::OAS3_0 => {
            // remove $schema property
            obj.remove("$schema");
            Self(Schema::from(obj.clone()))
          }
          OpenApiVersion::OAS3_1 => Self(Schema::from(obj.clone())),
        }
      }
    }
  }

  pub fn from_value(value: &Value, oas_version: OpenApiVersion) -> Self {
    let schema = Schema::try_from(value.clone());
    match schema {
      Ok(sch) => Self::new(sch, oas_version),
      Err(e) => {
        warn!("Error converting value to schema: get {e:?} for value {value:?}");
        Self::new(Schema::default(), oas_version)
      }
    }
  }

  pub fn into_inner(self) -> Schema {
    self.0
  }

  pub fn inner(&self) -> &Schema {
    &self.0
  }

  pub fn inner_mut(&mut self) -> &mut Schema {
    &mut self.0
  }

  fn remove_definition_from_schema(obj: &mut serde_json::Map<String, Value>, generator: &SchemaGenerator) {
    let definition_path = generator.settings().definitions_path.clone();
    let definition_path = definition_path
      .trim_start_matches('/')
      .split('/')
      .next()
      .unwrap_or_default();
    obj.remove(definition_path);
  }

  fn set_title_for_enum_variants(one_of: &mut Vec<Value>) {
    for s in one_of {
      if let Some(sch_obj) = s.as_object_mut() {
        if let Some(props) = sch_obj.clone().get("properties").and_then(|v| v.as_object()) {
          if props.len() == 1 {
            if let Some((prop_name, prop_value)) = props.iter().next() {
              if let Some(prop_obj) = prop_value.as_object() {
                if let Some(Value::String(prop_name)) = prop_obj.get("const") {
                  // if const is set, use it as title if not already set
                  sch_obj.entry("title").or_insert_with(|| prop_name.clone().into());
                } else {
                  // else, use the property name as title if not already set
                  sch_obj.entry("title").or_insert_with(|| prop_name.clone().into());
                }
              } else {
                // use the property name as title if not already set
                sch_obj.entry("title").or_insert_with(|| prop_name.clone().into());
              }
            }
          } else if let Some(enum_values) = props.iter().find_map(|(_, p)| {
            p.as_object()
              .and_then(|sch_obj| sch_obj.get("enum").and_then(|v| v.as_array()))
          }) {
            if enum_values.len() == 1 {
              if let Some(Value::String(prop_name)) = enum_values.as_slice().first() {
                sch_obj.entry("title").or_insert_with(|| prop_name.clone().into());
              }
            }
          } else if let Some(_type) = props.get("type").and_then(|v| v.as_object()) {
            if let Some(Value::String(prop_name)) = _type.get("const") {
              sch_obj.entry("title").or_insert_with(|| prop_name.clone().into());
            }
          } else {
            let const_values: Vec<&Value> = props
              .iter()
              .filter_map(|(_, p)| p.as_object().and_then(|sch_obj| sch_obj.get("const")))
              .collect();
            if const_values.len() == 1 {
              if let Some(Value::String(prop_name)) = const_values.first() {
                sch_obj.entry("title").or_insert_with(|| prop_name.clone().into());
              }
            }
          }
        } else if let Some(enum_values) = sch_obj.clone().get_mut("enum").and_then(|v| v.as_array_mut()) {
          if enum_values.len() == 1 {
            if let Some(Value::String(prop_name)) = enum_values.as_slice().first() {
              sch_obj.entry("title").or_insert_with(|| prop_name.clone().into());
            }
          }
        }
      }
    }
  }
}

#[cfg(any(feature = "test", test))]
impl From<Schema> for ApistosSchema {
  fn from(value: Schema) -> Self {
    Self(value)
  }
}

#[cfg(any(feature = "test", test))]
impl From<Schema> for crate::reference_or::ReferenceOr<ApistosSchema> {
  fn from(value: Schema) -> Self {
    Self::Object(value.into())
  }
}

#[cfg(test)]
mod test {
  use assert_json_diff::assert_json_eq;

  use crate::{ApistosSchema, JsonSchema, OpenApiVersion};
  use schemars::generate::SchemaSettings;
  use serde::Serialize;
  use serde_json::json;

  #[test]
  #[expect(dead_code)]
  #[allow(unused_qualifications)]
  fn test_apistos_schema_transform_3_0() {
    #[derive(JsonSchema, Serialize)]
    struct TestStruct {
      name: String,
    }

    #[derive(JsonSchema, Serialize)]
    struct TestStruct2 {
      surname: String,
    }

    #[derive(JsonSchema)]
    enum TestEnum {
      Test,
      Test2,
    }

    #[derive(JsonSchema, Serialize)]
    #[serde(tag = "type")]
    enum TestAlgebraicEnum {
      Test { key: String, value: String },
      Test2 { key2: String, value2: String },
    }

    #[derive(JsonSchema, Serialize)]
    #[serde(tag = "type")]
    enum TestAlgebraicEnum2 {
      Test(TestStruct),
      Test2(TestStruct2),
    }

    let mut generator = SchemaSettings::openapi3().into_generator();
    let schema = TestEnum::json_schema(&mut generator);

    let apistos_schema = ApistosSchema::new(schema, OpenApiVersion::OAS3_0);

    assert_json_eq!(
      apistos_schema.into_inner(),
      json!({
        "type": "string",
        "enum": [
          "Test",
          "Test2"
        ]
      })
    );

    let mut generator = SchemaSettings::openapi3().into_generator();
    let schema = TestAlgebraicEnum::json_schema(&mut generator);

    let apistos_schema = ApistosSchema::new(schema, OpenApiVersion::OAS3_0);

    assert_json_eq!(
      apistos_schema.into_inner(),
      json!({
        "oneOf": [
          {
            "title": "Test",
            "type": "object",
            "properties": {
              "type": {
                "type": "string",
                "const": "Test"
              },
              "key": {
                "type": "string"
              },
              "value": {
                "type": "string"
              }
            },
            "required": [
              "type",
              "key",
              "value"
            ]
          },
          {
            "title": "Test2",
            "type": "object",
            "properties": {
              "type": {
                "type": "string",
                "const": "Test2"
              },
              "key2": {
                "type": "string"
              },
              "value2": {
                "type": "string"
              }
            },
            "required": [
              "type",
              "key2",
              "value2"
            ]
          }
        ]
      })
    );

    let mut generator = SchemaSettings::openapi3().into_generator();
    let schema = TestAlgebraicEnum2::json_schema(&mut generator);

    let apistos_schema = ApistosSchema::new(schema, OpenApiVersion::OAS3_0);

    assert_json_eq!(
      apistos_schema.into_inner(),
      json!({
        "oneOf": [
          {
            "title": "Test",
            "type": "object",
            "properties": {
              "type": {
                "type": "string",
                "const": "Test"
              }
            },
            "$ref": "#/components/schemas/TestStruct",
            "required": [
              "type"
            ]
          },
          {
            "title": "Test2",
            "type": "object",
            "properties": {
              "type": {
                "type": "string",
                "const": "Test2"
              }
            },
            "$ref": "#/components/schemas/TestStruct2",
            "required": [
              "type"
            ]
          }
        ]
      })
    );
  }

  #[test]
  #[expect(dead_code)]
  #[allow(unused_qualifications)]
  fn test_apistos_schema_transform_3_1() {
    #[derive(JsonSchema, Serialize)]
    struct TestStruct {
      name: String,
    }

    #[derive(JsonSchema, Serialize)]
    struct TestStruct2 {
      surname: String,
    }

    #[derive(JsonSchema)]
    enum TestEnum {
      Test,
      Test2,
    }

    #[derive(JsonSchema, Serialize)]
    #[serde(tag = "type")]
    enum TestAlgebraicEnum {
      Test { key: String, value: String },
      Test2 { key2: String, value2: String },
    }

    #[derive(JsonSchema, Serialize)]
    #[serde(tag = "type")]
    enum TestAlgebraicEnum2 {
      Test(TestStruct),
      Test2(TestStruct2),
    }

    let mut generator = SchemaSettings::draft2020_12().into_generator();
    let schema = TestEnum::json_schema(&mut generator);

    let apistos_schema = ApistosSchema::new(schema, OpenApiVersion::OAS3_1);

    assert_json_eq!(
      apistos_schema.into_inner(),
      json!({
        "type": "string",
        "enum": [
          "Test",
          "Test2"
        ]
      })
    );

    let mut generator = SchemaSettings::draft2020_12().into_generator();
    let schema = TestAlgebraicEnum::json_schema(&mut generator);

    let apistos_schema = ApistosSchema::new(schema, OpenApiVersion::OAS3_1);

    assert_json_eq!(
      apistos_schema.into_inner(),
      json!({
        "oneOf": [
          {
            "title": "Test",
            "type": "object",
            "properties": {
              "type": {
                "type": "string",
                "const": "Test"
              },
              "key": {
                "type": "string"
              },
              "value": {
                "type": "string"
              }
            },
            "required": [
              "type",
              "key",
              "value"
            ]
          },
          {
            "title": "Test2",
            "type": "object",
            "properties": {
              "type": {
                "type": "string",
                "const": "Test2"
              },
              "key2": {
                "type": "string"
              },
              "value2": {
                "type": "string"
              }
            },
            "required": [
              "type",
              "key2",
              "value2"
            ]
          }
        ]
      })
    );

    let mut generator = SchemaSettings::draft2020_12().into_generator();
    let schema = TestAlgebraicEnum2::json_schema(&mut generator);

    let apistos_schema = ApistosSchema::new(schema, OpenApiVersion::OAS3_1);

    assert_json_eq!(
      apistos_schema.into_inner(),
      json!({
        "oneOf": [
          {
            "title": "Test",
            "type": "object",
            "properties": {
              "type": {
                "type": "string",
                "const": "Test"
              }
            },
            "$ref": "#/$defs/TestStruct",
            "required": [
              "type"
            ]
          },
          {
            "title": "Test2",
            "type": "object",
            "properties": {
              "type": {
                "type": "string",
                "const": "Test2"
              }
            },
            "$ref": "#/$defs/TestStruct2",
            "required": [
              "type"
            ]
          }
        ]
      })
    )
  }
}
