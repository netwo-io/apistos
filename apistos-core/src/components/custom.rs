// @todo depends on ipnetwork's schemars feature instead once ipnetwork depends on schemars 1.x
#[cfg(feature = "ipnetwork")]
pub mod ipnetwork {
  use crate::simple::simple_modifier;
  use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
  use serde::{Deserialize, Serialize};
  use std::borrow::Cow;

  #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
  #[serde(transparent)]
  pub struct IpNetwork(ipnetwork::IpNetwork);

  #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
  #[serde(transparent)]
  pub struct Ipv4Network(ipnetwork::Ipv4Network);

  #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
  #[serde(transparent)]
  pub struct Ipv6Network(ipnetwork::Ipv6Network);

  simple_modifier!(IpNetwork);
  simple_modifier!(Ipv4Network);
  simple_modifier!(Ipv6Network);

  impl JsonSchema for IpNetwork {
    fn always_inline_schema() -> bool {
      true
    }

    fn schema_name() -> Cow<'static, str> {
      "Ip".into()
    }

    fn schema_id() -> Cow<'static, str> {
      "ipnetwork::Ip".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
      json_schema!({
        "oneOf": [
          {
            "type": "string",
            "format": "ipv4"
          }
          ,{
            "type": "string",
            "format": "ipv6"
          }
        ]
      })
    }
  }

  impl JsonSchema for Ipv4Network {
    fn always_inline_schema() -> bool {
      true
    }

    fn schema_name() -> Cow<'static, str> {
      "IpV4".into()
    }

    fn schema_id() -> Cow<'static, str> {
      "ipnetwork::Ipv4Network".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
      json_schema!({
        "type": "string",
        "format": "ipv4"
      })
    }
  }

  impl JsonSchema for Ipv6Network {
    fn always_inline_schema() -> bool {
      true
    }

    fn schema_name() -> Cow<'static, str> {
      "IpV6".into()
    }

    fn schema_id() -> Cow<'static, str> {
      "ipnetwork::Ipv6Network".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
      json_schema!({
        "type": "string",
        "format": "ipv6"
      })
    }
  }

  impl From<ipnetwork::IpNetwork> for IpNetwork {
    fn from(value: ipnetwork::IpNetwork) -> Self {
      Self(value)
    }
  }

  impl From<IpNetwork> for ipnetwork::IpNetwork {
    fn from(value: IpNetwork) -> Self {
      value.0
    }
  }

  impl From<ipnetwork::Ipv4Network> for Ipv4Network {
    fn from(value: ipnetwork::Ipv4Network) -> Self {
      Self(value)
    }
  }

  impl From<Ipv4Network> for ipnetwork::Ipv4Network {
    fn from(value: Ipv4Network) -> Self {
      value.0
    }
  }

  impl From<ipnetwork::Ipv6Network> for Ipv6Network {
    fn from(value: ipnetwork::Ipv6Network) -> Self {
      Self(value)
    }
  }

  impl From<Ipv6Network> for ipnetwork::Ipv6Network {
    fn from(value: Ipv6Network) -> Self {
      value.0
    }
  }

  #[cfg(test)]
  mod test {
    use crate::ApiComponent;
    use crate::custom::ipnetwork::IpNetwork;
    use apistos_models::reference_or::ReferenceOr;
    use apistos_models::{ApistosSchema, OpenApiVersion};
    use assert_json_diff::assert_json_eq;
    use schemars::{JsonSchema, SchemaGenerator};
    use serde_json::json;

    #[test]
    #[expect(dead_code)]
    fn ip_network_schema() {
      #[derive(JsonSchema)]
      struct Test {
        ip: IpNetwork,
        label: String,
      }

      impl ApiComponent for Test {
        fn child_schemas(oas_version: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
          <IpNetwork as ApiComponent>::schema(oas_version).into_iter().collect()
        }

        fn schema(oas_version: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
          let schema_settings = oas_version.get_schema_settings();
          let generator = SchemaGenerator::new(schema_settings);
          let schema = generator.into_root_schema_for::<Test>();
          Some(("Test".to_string(), ApistosSchema::new(schema, oas_version).into()))
        }
      }

      let schema = <Test as ApiComponent>::schema(OpenApiVersion::OAS3_0);
      let (schema_name, schema) = schema.expect("schema should be defined");
      assert_eq!(schema_name, "Test");
      let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
      assert_json_eq!(
        json,
        json!({
          "title": "Test",
          "type": "object",
          "properties": {
            "ip": {
              "oneOf": [
                {
                  "type": "string",
                  "format": "ipv4"
                },
                {
                  "type": "string",
                  "format": "ipv6"
                }
              ]
            },
            "label": {
              "type": "string"
            }
          },
          "required": [
            "ip",
            "label"
          ]
        })
      );

      let schema = <Test as ApiComponent>::schema(OpenApiVersion::OAS3_1);
      let (schema_name, schema) = schema.expect("schema should be defined");
      assert_eq!(schema_name, "Test");
      let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
      assert_json_eq!(
        json,
        json!({
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "title": "Test",
          "type": "object",
          "properties": {
            "ip": {
              "oneOf": [
                {
                  "type": "string",
                  "format": "ipv4"
                },
                {
                  "type": "string",
                  "format": "ipv6"
                }
              ]
            },
            "label": {
              "type": "string"
            }
          },
          "required": [
            "ip",
            "label"
          ]
        })
      );
    }
  }
}

#[cfg(feature = "ipnetwork_0_20")]
pub mod ipnetwork_20 {
  use crate::simple::simple_modifier;
  use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
  use serde::{Deserialize, Serialize};
  use std::borrow::Cow;

  #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
  #[serde(transparent)]
  pub struct IpNetwork(ipnetwork_0_20::IpNetwork);

  #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
  #[serde(transparent)]
  pub struct Ipv4Network(ipnetwork_0_20::Ipv4Network);

  #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
  #[serde(transparent)]
  pub struct Ipv6Network(ipnetwork_0_20::Ipv6Network);

  simple_modifier!(IpNetwork);
  simple_modifier!(Ipv4Network);
  simple_modifier!(Ipv6Network);

  impl JsonSchema for IpNetwork {
    fn always_inline_schema() -> bool {
      true
    }

    fn schema_name() -> Cow<'static, str> {
      "Ip".into()
    }

    fn schema_id() -> Cow<'static, str> {
      "ipnetwork::Ip".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
      json_schema!({
        "oneOf": [
          {
            "type": "string",
            "format": "ipv4"
          }
          ,{
            "type": "string",
            "format": "ipv6"
          }
        ]
      })
    }
  }

  impl JsonSchema for Ipv4Network {
    fn always_inline_schema() -> bool {
      true
    }

    fn schema_name() -> Cow<'static, str> {
      "IpV4".into()
    }

    fn schema_id() -> Cow<'static, str> {
      "ipnetwork::Ipv4Network".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
      json_schema!({
        "type": "string",
        "format": "ipv4"
      })
    }
  }

  impl JsonSchema for Ipv6Network {
    fn always_inline_schema() -> bool {
      true
    }

    fn schema_name() -> Cow<'static, str> {
      "IpV6".into()
    }

    fn schema_id() -> Cow<'static, str> {
      "ipnetwork::Ipv6Network".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
      json_schema!({
        "type": "string",
        "format": "ipv6"
      })
    }
  }

  impl From<ipnetwork_0_20::IpNetwork> for IpNetwork {
    fn from(value: ipnetwork_0_20::IpNetwork) -> Self {
      Self(value)
    }
  }

  impl From<IpNetwork> for ipnetwork_0_20::IpNetwork {
    fn from(value: IpNetwork) -> Self {
      value.0
    }
  }

  impl From<ipnetwork_0_20::Ipv4Network> for Ipv4Network {
    fn from(value: ipnetwork_0_20::Ipv4Network) -> Self {
      Self(value)
    }
  }

  impl From<Ipv4Network> for ipnetwork_0_20::Ipv4Network {
    fn from(value: Ipv4Network) -> Self {
      value.0
    }
  }

  impl From<ipnetwork_0_20::Ipv6Network> for Ipv6Network {
    fn from(value: ipnetwork_0_20::Ipv6Network) -> Self {
      Self(value)
    }
  }

  impl From<Ipv6Network> for ipnetwork_0_20::Ipv6Network {
    fn from(value: Ipv6Network) -> Self {
      value.0
    }
  }

  #[cfg(test)]
  mod test {
    use crate::ApiComponent;
    use crate::custom::ipnetwork_20::IpNetwork;
    use apistos_models::reference_or::ReferenceOr;
    use apistos_models::{ApistosSchema, OpenApiVersion};
    use assert_json_diff::assert_json_eq;
    use schemars::{JsonSchema, SchemaGenerator};
    use serde_json::json;

    #[test]
    #[expect(dead_code)]
    fn ip_network_schema() {
      #[derive(JsonSchema)]
      struct Test {
        ip: IpNetwork,
        label: String,
      }

      impl ApiComponent for Test {
        fn child_schemas(oas_version: OpenApiVersion) -> Vec<(String, ReferenceOr<ApistosSchema>)> {
          <IpNetwork as ApiComponent>::schema(oas_version).into_iter().collect()
        }

        fn schema(oas_version: OpenApiVersion) -> Option<(String, ReferenceOr<ApistosSchema>)> {
          let schema_settings = oas_version.get_schema_settings();
          let generator = SchemaGenerator::new(schema_settings);
          let schema = generator.into_root_schema_for::<Test>();
          Some(("Test".to_string(), ApistosSchema::new(schema, oas_version).into()))
        }
      }

      let schema = <Test as ApiComponent>::schema(OpenApiVersion::OAS3_0);
      let (schema_name, schema) = schema.expect("schema should be defined");
      assert_eq!(schema_name, "Test");
      let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
      assert_json_eq!(
        json,
        json!({
          "title": "Test",
          "type": "object",
          "properties": {
            "ip": {
              "oneOf": [
                {
                  "type": "string",
                  "format": "ipv4"
                },
                {
                  "type": "string",
                  "format": "ipv6"
                }
              ]
            },
            "label": {
              "type": "string"
            }
          },
          "required": [
            "ip",
            "label"
          ]
        })
      );

      let schema = <Test as ApiComponent>::schema(OpenApiVersion::OAS3_1);
      let (schema_name, schema) = schema.expect("schema should be defined");
      assert_eq!(schema_name, "Test");
      let json = serde_json::to_value(schema).expect("Unable to serialize as Json");
      assert_json_eq!(
        json,
        json!({
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "title": "Test",
          "type": "object",
          "properties": {
            "ip": {
              "oneOf": [
                {
                  "type": "string",
                  "format": "ipv4"
                },
                {
                  "type": "string",
                  "format": "ipv6"
                }
              ]
            },
            "label": {
              "type": "string"
            }
          },
          "required": [
            "ip",
            "label"
          ]
        })
      );
    }
  }
}
