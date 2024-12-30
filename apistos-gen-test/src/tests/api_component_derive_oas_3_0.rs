use apistos::OpenApiVersion;
use assert_json_diff::assert_json_eq;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::assert_schema;
use apistos_core::ApiComponent;
use apistos_gen::ApiComponent;

#[test]
#[expect(dead_code)]
fn api_component_derive() {
  #[derive(JsonSchema, ApiComponent)]
  struct Name {
    name: String,
  }

  let name_schema = <Name as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
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
#[expect(dead_code)]
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

  let name_schema = <Name<Test> as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name<Test> as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
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
        "name",
        "id"
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
          "minimum": 0,
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
#[expect(dead_code)]
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

  let name_schema = <Name as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
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
          "minimum": 0,
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
        "name",
        "id_number",
        "id_string"
      ],
      "title": "Name",
      "type": "object"
    })
  );
}

#[test]
#[expect(dead_code)]
fn api_component_derive_with_deprecated_field() {
  #[derive(JsonSchema, ApiComponent)]
  struct Name {
    #[deprecated]
    name: Option<String>,
    new_name: String,
  }

  let name_schema = <Name as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
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
          "type": "string",
          "deprecated": true,
          "nullable": true
        },
        "new_name": {
          "type": "string"
        }
      },
      "required": [
        "new_name"
      ],
      "title": "Name",
      "type": "object"
    })
  );
}

#[test]
#[expect(dead_code)]
fn api_component_derive_with_format() {
  #[derive(JsonSchema, ApiComponent)]
  struct Name {
    #[schemars(length(min = 1, max = 10))]
    usernames: Vec<String>,
  }

  let name_schema = <Name as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
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
#[expect(dead_code)]
fn api_component_derive_recursive() {
  #[derive(JsonSchema, ApiComponent)]
  struct Name {
    old_name: Option<Box<Name>>,
  }

  let name_schema = <Name as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Name as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
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
      "type": "object",
      "properties": {
        "old_name": {
          "$ref": "#/components/schemas/Name",
          "nullable": true
        }
      }
    })
  );
}

#[test]
fn api_component_derive_flatten_algebraic_enums() {
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

  let name_schema = <Query as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Query as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
  assert!(name_schema.is_some());
  assert!(name_child_schemas.is_empty());
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Query");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "title": "Query",
      "type": "object",
      "properties": {
        "limit": {
          "type": "integer",
          "format": "uint32",
          "minimum": 0
        }
      },
      "oneOf": [
        {
          "title": "after_id",
          "type": "object",
          "properties": {
            "after_id": {
              "type": "integer",
              "format": "uint64",
              "minimum": 0
            }
          },
          "required": [
            "after_id"
          ]
        },
        {
          "title": "after_date",
          "type": "object",
          "properties": {
            "after_date": {
              "type": "string",
              "format": "date-time"
            }
          },
          "required": [
            "after_date"
          ]
        }
      ],
      "required": [
        "limit"
      ]
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

  let name_schema = <Query as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Query as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
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
          "minimum": 0,
          "type": "integer"
        },
        "offset": {
          "format": "uint32",
          "minimum": 0,
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

  let name_schema = <Query as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Query as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
  assert!(name_schema.is_some());
  assert_eq!(name_child_schemas.len(), 2);
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Query");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "title": "Query",
      "type": "object",
      "properties": {
        "test": {
          "type": "string"
        },
        "kinds": {
          "type": "array",
          "items": {
            "$ref": "#/components/schemas/KindQuery"
          }
        }
      },
      "required": [
        "test",
        "kinds"
      ],
      "oneOf": [
        {
          "title": "Active",
          "type": "object",
          "properties": {
            "Active": {
              "$ref": "#/components/schemas/ActiveOrInactiveQuery"
            }
          },
          "required": [
            "Active"
          ]
        },
        {
          "title": "Inactive",
          "type": "object",
          "properties": {
            "Inactive": {
              "$ref": "#/components/schemas/ActiveOrInactiveQuery"
            }
          },
          "required": [
            "Inactive"
          ]
        }
      ]
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
          "minimum": 0,
          "type": "integer"
        }
      },
      "required": [
        "id",
        "description"
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

#[test]
fn api_component_derive_named_enums_documented() {
  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) enum Kind {
    Complex,
    /// A simple stuff
    Simple,
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) struct Query {
    pub(crate) test: String,
    pub(crate) kind: Kind,
  }

  let name_schema = <Query as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Query as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
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
        "kind": {
          "$ref": "#/components/schemas/Kind"
        },
        "test": {
          "type": "string"
        }
      },
      "required": [
        "test",
        "kind"
      ],
      "title": "Query",
      "type": "object"
    })
  );

  let (_, child_schema) = name_child_schemas
    .iter()
    .find(|(name, _)| name == "Kind")
    .expect("missing child schema");
  assert_schema(&child_schema.clone());
  let json = serde_json::to_value(child_schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "oneOf": [
        {
          "title": "Complex",
          "type": "string",
          "enum": [
            "Complex"
          ]
        },
        {
          "description": "A simple stuff",
          "type": "string",
          "const": "Simple"
        }
      ]
    })
  );
}

#[test]
fn api_component_derive_named_tagged_enums() {
  #[derive(Serialize, Debug, ApiComponent, JsonSchema)]
  #[cfg_attr(test, derive(Deserialize))]
  #[serde(tag = "kind")]
  #[serde(rename_all = "snake_case")]
  pub(crate) enum TestStruct {
    Variant1 {
      expiration: DateTime<Utc>,
    },
    Variant2 {
      email: String,
      /// pseudonymised phone number
      phone_number: String,
    },
  }

  let name_schema = <TestStruct as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <TestStruct as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
  assert!(name_schema.is_some());
  assert!(name_child_schemas.is_empty());
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "TestStruct");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "title": "TestStruct",
      "oneOf": [
        {
          "title": "variant1",
          "type": "object",
          "properties": {
            "expiration": {
              "type": "string",
              "format": "date-time"
            },
            "kind": {
              "type": "string",
              "enum": [
                "variant1"
              ]
            }
          },
          "required": [
            "kind",
            "expiration"
          ]
        },
        {
          "title": "variant2",
          "type": "object",
          "properties": {
            "email": {
              "type": "string"
            },
            "phone_number": {
              "description": "pseudonymised phone number",
              "type": "string"
            },
            "kind": {
              "type": "string",
              "enum": [
                "variant2"
              ]
            }
          },
          "required": [
            "kind",
            "email",
            "phone_number"
          ]
        }
      ]
    })
  );
}

#[test]
fn api_component_derive_named_enums_deep() {
  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) struct TestStuff {
    pub(crate) name: String,
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  #[schemars(rename_all = "snake_case")]
  #[schemars(tag = "type")]
  pub(crate) enum Level4Query {
    Something(TestStuff),
    Other(TestStuff),
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) struct ActiveOrInactiveQuery {
    pub(crate) id: u32,
    pub(crate) description: String,
    pub(crate) level4: Level4Query,
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) enum KindQuery {
    Active(ActiveOrInactiveQuery),
    Inactive(ActiveOrInactiveQuery),
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) struct Level3Query {
    pub(crate) kinds: Vec<KindQuery>,
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) struct Level2Query {
    pub(crate) level3: Level3Query,
  }

  #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
  pub(crate) struct Query {
    pub(crate) test: String,
    pub(crate) level2: Level2Query,
  }

  let name_schema = <Query as ApiComponent>::schema(OpenApiVersion::OAS3_0);
  let name_child_schemas = <Query as ApiComponent>::child_schemas(OpenApiVersion::OAS3_0);
  assert!(name_schema.is_some());
  assert_eq!(name_child_schemas.len(), 5);
  let (schema_name, schema) = name_schema.expect("schema should be defined");
  assert_eq!(schema_name, "Query");
  assert_schema(&schema.clone());
  let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "properties": {
        "level2": {
          "$ref": "#/components/schemas/Level2Query"
        },
        "test": {
          "type": "string"
        }
      },
      "required": [
        "test",
        "level2"
      ],
      "title": "Query",
      "type": "object"
    })
  );

  let (_, child_schema) = name_child_schemas
    .iter()
    .find(|(name, _)| name == "Level2Query")
    .expect("missing child schema");
  assert_schema(&child_schema.clone());
  let json = serde_json::to_value(child_schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "properties": {
        "level3": {
          "$ref": "#/components/schemas/Level3Query"
        }
      },
      "required": [
        "level3"
      ],
      "type": "object"
    })
  );

  let (_, child_schema) = name_child_schemas
    .iter()
    .find(|(name, _)| name == "Level3Query")
    .expect("missing child schema");
  assert_schema(&child_schema.clone());
  let json = serde_json::to_value(child_schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "properties": {
        "kinds": {
          "items": {
            "$ref": "#/components/schemas/KindQuery"
          },
          "type": "array"
        }
      },
      "required": [
        "kinds"
      ],
      "type": "object"
    })
  );

  let (_, child_schema) = name_child_schemas
    .iter()
    .find(|(name, _)| name == "KindQuery")
    .expect("missing child schema");
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

  let (_, child_schema) = name_child_schemas
    .iter()
    .find(|(name, _)| name == "ActiveOrInactiveQuery")
    .expect("missing child schema");
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
          "minimum": 0,
          "type": "integer"
        },
        "level4": {
          "$ref": "#/components/schemas/Level4Query"
        }
      },
      "required": [
        "id",
        "description",
        "level4"
      ],
      "type": "object"
    })
  );

  let (_, child_schema) = name_child_schemas
    .iter()
    .find(|(name, _)| name == "Level4Query")
    .expect("missing child schema");
  assert_schema(&child_schema.clone());
  let json = serde_json::to_value(child_schema).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "oneOf": [
        {
          "title": "something",
          "type": "object",
          "properties": {
            "type": {
              "type": "string",
              "const": "something"
            },
            "name": {
              "type": "string"
            }
          },
          "required": [
            "type",
            "name"
          ]
        },
        {
          "title": "other",
          "type": "object",
          "properties": {
            "type": {
              "type": "string",
              "const": "other"
            },
            "name": {
              "type": "string"
            }
          },
          "required": [
            "type",
            "name"
          ]
        }
      ]
    })
  );
}
