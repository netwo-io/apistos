use indexmap::IndexMap;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

/// An object representing a Server.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Server {
  /// A URL to the target host. This URL supports Server Variables and MAY be relative, to indicate that the host location is relative to the location where the OpenAPI document is being served. Variable substitutions will be made when a variable is named in `{`brackets`}`.
  pub url: String,
  /// An optional string describing the host designated by the URL. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  /// A map between a variable name and its value. The value is used for substitution in the server's URL template.
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub variables: BTreeMap<String, ServerVariable>,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

/// An object representing a Server Variable for server URL template substitution.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct ServerVariable {
  /// An enumeration of string values to be used if the substitution options are from a limited set. The array SHOULD NOT be empty.
  #[serde(rename = "enum")]
  pub _enum: Vec<String>,
  /// The default value to use for substitution, which SHALL be sent if an alternate value is not supplied. Note this behavior is different than the [Schema Object's](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#schema-object) treatment of default values, because in those cases parameter values are optional. If the [`enum`](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#serverVariableEnum) is defined, the value SHOULD exist in the enum's values.
  pub default: String,
  /// An optional description for the server variable. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
}
