use assert_json_diff::assert_json_eq;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::assert_schema;
use netwopenapi_core::ApiComponent;
use netwopenapi_gen::ApiComponent;

#[test]
#[allow(dead_code)]
fn api_component_derive() {
  #[derive(JsonSchema, ApiComponent)]
  struct Name {
    name: String,
  }

  let name_schema = <Name as ApiComponent>::schema();
  let name_child_schemas = <Name as ApiComponent>::child_schemas();
  assert!(name_schema.is_some());
  assert!(name_child_schemas.is_empty());
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Name");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "properties": {
        "name": {
          "type": "string"
        }
      },
      "required": [
        "name"
      ],
      "title": "Name",
      "type": "object"
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_component_derive_with_generic() {
  #[derive(JsonSchema, ApiComponent)]
  struct Name<T>
  where
    T: JsonSchema,
  {
    name: String,
    id: T,
  }

  #[derive(JsonSchema, ApiComponent)]
  struct Test {
    id_number: u32,
    id_string: String,
  }

  let name_schema = <Name<Test> as ApiComponent>::schema();
  let name_child_schemas = <Name<Test> as ApiComponent>::child_schemas();
  assert!(name_schema.is_some());
  assert_eq!(name_child_schemas.len(), 1);
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Name_for_Test");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "properties": {
        "id": {
          "$ref": "#/components/schemas/Test"
        },
        "name": {
          "type": "string"
        }
      },
      "required": [
        "id",
        "name"
      ],
      "title": "Name_for_Test",
      "type": "object"
    })
  );

  let (child_schema_name, child_schema) = name_child_schemas.first().expect("missing child schema");
  assert_eq!(child_schema_name, "Test");
  assert_schema(&child_schema.clone());
  let json = serde_json::to_value(child_schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "properties": {
        "id_number": {
          "format": "uint32",
          "minimum": 0.0,
          "type": "integer"
        },
        "id_string": {
          "type": "string"
        }
      },
      "required": [
        "id_number",
        "id_string"
      ],
      "type": "object"
    })
  );
}

#[test]
fn api_component_derive_with_flatten() {
  #[derive(JsonSchema, ApiComponent)]
  struct Name {
    name: String,
    #[schemars(flatten)] // also works with #[serde(flatten)]
    id: Test,
  }

  #[derive(JsonSchema, ApiComponent)]
  struct Test {
    id_number: u32,
    id_string: String,
  }

  let name_schema = <Name as ApiComponent>::schema();
  let name_child_schemas = <Name as ApiComponent>::child_schemas();
  assert!(name_schema.is_some());
  assert!(name_child_schemas.is_empty());
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Name");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "properties": {
        "id_number": {
          "format": "uint32",
          "minimum": 0.0,
          "type": "integer"
        },
        "id_string": {
          "type": "string"
        },
        "name": {
          "type": "string"
        }
      },
      "required": [
        "id_number",
        "id_string",
        "name"
      ],
      "title": "Name",
      "type": "object"
    })
  );
}

#[test]
fn api_component_derive_with_format() {
  #[derive(JsonSchema, ApiComponent)]
  struct Name {
    #[schemars(length(min = 1, max = 10))]
    usernames: Vec<String>,
  }

  let name_schema = <Name as ApiComponent>::schema();
  let name_child_schemas = <Name as ApiComponent>::child_schemas();
  assert!(name_schema.is_some());
  assert!(name_child_schemas.is_empty());
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Name");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "properties": {
        "usernames": {
          "items": {
            "type": "string"
          },
          "maxItems": 10,
          "minItems": 1,
          "type": "array"
        }
      },
      "required": [
        "usernames"
      ],
      "title": "Name",
      "type": "object"
    })
  );
}

#[test]
fn api_component_derive_recursive() {
  #[derive(JsonSchema, ApiComponent)]
  struct Name {
    old_name: Option<Box<Name>>,
  }

  let name_schema = <Name as ApiComponent>::schema();
  let name_child_schemas = <Name as ApiComponent>::child_schemas();
  assert!(name_schema.is_some());
  assert_eq!(name_child_schemas.len(), 1);
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Name");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "properties": {
        "old_name": {
          "allOf": [
            {
              "$ref": "#/components/schemas/Name"
            }
          ],
          "nullable": true
        }
      },
      "title": "Name",
      "type": "object"
    })
  );

  let (child_schema_name, child_schema) = name_child_schemas.first().expect("missing child schema");
  assert_eq!(child_schema_name, "Name");
  assert_schema(&child_schema.clone());
  let json = serde_json::to_value(child_schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "properties": {
        "old_name": {
          "allOf": [
            {
              "$ref": "#/components/schemas/Name"
            }
          ],
          "nullable": true
        }
      },
      "type": "object"
    })
  );
}

#[test]
fn api_component_derive_flatten_algebric_enums() {
  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) enum IdOrDateQuery {
    #[serde(rename = "after_id")]
    Id(u64),
    #[serde(rename = "after_date")]
    Date(DateTime<Utc>),
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) struct Query {
    #[serde(flatten)]
    pub(crate) after: IdOrDateQuery,
    pub(crate) limit: u32,
  }

  let name_schema = <Query as ApiComponent>::schema();
  let name_child_schemas = <Query as ApiComponent>::child_schemas();
  assert!(name_schema.is_some());
  assert!(name_child_schemas.is_empty());
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Query");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "oneOf": [
        {
          "additionalProperties": false,
          "properties": {
            "after_id": {
              "format": "uint64",
              "minimum": 0.0,
              "type": "integer"
            }
          },
          "required": [
            "after_id"
          ],
          "title": "after_id",
          "type": "object"
        },
        {
          "additionalProperties": false,
          "properties": {
            "after_date": {
              "format": "date-time",
              "type": "string"
            }
          },
          "required": [
            "after_date"
          ],
          "title": "after_date",
          "type": "object"
        }
      ],
      "properties": {
        "limit": {
          "format": "uint32",
          "minimum": 0.0,
          "type": "integer"
        }
      },
      "required": [
        "limit"
      ],
      "title": "Query",
      "type": "object"
    })
  );
}

#[test]
fn api_component_derive_optional_enums() {
  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) enum StatusQuery {
    Active,
    Inactive,
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) struct PaginationQuery {
    pub(crate) limit: u32,
    pub(crate) offset: Option<u32>,
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) struct Query {
    pub(crate) test: Option<String>,
    pub(crate) status: Option<StatusQuery>,
    #[serde(flatten)]
    pub(crate) pagination: PaginationQuery,
  }

  let name_schema = <Query as ApiComponent>::schema();
  let name_child_schemas = <Query as ApiComponent>::child_schemas();
  assert!(name_schema.is_some());
  assert_eq!(name_child_schemas.len(), 1);
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Query");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "properties": {
        "limit": {
          "format": "uint32",
          "minimum": 0.0,
          "type": "integer"
        },
        "offset": {
          "format": "uint32",
          "minimum": 0.0,
          "nullable": true,
          "type": "integer"
        },
        "status": {
          "allOf": [
            {
              "$ref": "#/components/schemas/StatusQuery"
            }
          ],
          "nullable": true
        },
        "test": {
          "nullable": true,
          "type": "string"
        }
      },
      "required": [
        "limit"
      ],
      "title": "Query",
      "type": "object"
    })
  );
}

#[test]
fn api_component_derive_named_enums() {
  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) struct ActiveOrInactiveQuery {
    pub(crate) id: u32,
    pub(crate) description: String,
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) enum KindQuery {
    Active(ActiveOrInactiveQuery),
    Inactive(ActiveOrInactiveQuery),
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) struct Query {
    pub(crate) test: String,
    #[serde(flatten)]
    pub(crate) kind: KindQuery,
    pub(crate) kinds: Vec<KindQuery>,
  }

  let name_schema = <Query as ApiComponent>::schema();
  let name_child_schemas = <Query as ApiComponent>::child_schemas();
  assert!(name_schema.is_some());
  assert_eq!(name_child_schemas.len(), 2);
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Query");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "oneOf": [
        {
          "additionalProperties": false,
          "properties": {
            "Active": {
              "$ref": "#/components/schemas/ActiveOrInactiveQuery"
            }
          },
          "required": [
            "Active"
          ],
          "title": "Active",
          "type": "object"
        },
        {
          "additionalProperties": false,
          "properties": {
            "Inactive": {
              "$ref": "#/components/schemas/ActiveOrInactiveQuery"
            }
          },
          "required": [
            "Inactive"
          ],
          "title": "Inactive",
          "type": "object"
        }
      ],
      "properties": {
        "kinds": {
          "items": {
            "$ref": "#/components/schemas/KindQuery"
          },
          "type": "array"
        },
        "test": {
          "type": "string"
        }
      },
      "required": [
        "kinds",
        "test"
      ],
      "title": "Query",
      "type": "object"
    })
  );

  let (child_schema_name, child_schema) = name_child_schemas.first().expect("missing child schema");
  assert_eq!(child_schema_name, "ActiveOrInactiveQuery");
  assert_schema(&child_schema.clone());
  let json = serde_json::to_value(child_schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "properties": {
        "description": {
          "type": "string"
        },
        "id": {
          "format": "uint32",
          "minimum": 0.0,
          "type": "integer"
        }
      },
      "required": [
        "description",
        "id"
      ],
      "type": "object"
    })
  );

  let (child_schema_name, child_schema) = name_child_schemas.last().expect("missing child schema");
  assert_eq!(child_schema_name, "KindQuery");
  assert_schema(&child_schema.clone());
  let json = serde_json::to_value(child_schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "oneOf": [
        {
          "additionalProperties": false,
          "properties": {
            "Active": {
              "$ref": "#/components/schemas/ActiveOrInactiveQuery"
            }
          },
          "required": [
            "Active"
          ],
          "title": "Active",
          "type": "object"
        },
        {
          "additionalProperties": false,
          "properties": {
            "Inactive": {
              "$ref": "#/components/schemas/ActiveOrInactiveQuery"
            }
          },
          "required": [
            "Inactive"
          ],
          "title": "Inactive",
          "type": "object"
        }
      ]
    })
  );
}
