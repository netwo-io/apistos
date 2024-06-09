use crate::paths::ExternalDocumentation;
use indexmap::IndexMap;
use serde::Serialize;
use serde_json::Value;

/// Adds metadata to a single tag that is used by the [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#operation-object). It is not mandatory to have a Tag Object per tag defined in the Operation Object instances.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Tag {
  /// The name of the tag.
  pub name: String,
  /// A short description for the tag. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  /// Additional external documentation for this tag.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub external_docs: Option<ExternalDocumentation>,
  /// # OAS 3.0
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  /// # OAS 3.1
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}
