use crate::paths::ExternalDocumentation;
use indexmap::IndexMap;
use serde::Serialize;
use serde_json::Value;

/// Adds metadata to a single tag that is used by the [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#operation-object). It is not mandatory to have a Tag Object per tag defined in the Operation Object instances.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
  /// The name of the tag.
  pub name: String,
  /// A short description for the tag. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
  pub description: Option<String>,
  /// Additional external documentation for this tag.
  pub external_docs: Option<ExternalDocumentation>,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten)]
  pub extensions: IndexMap<String, Value>,
}
