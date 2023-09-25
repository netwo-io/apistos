use schemars::schema::Schema;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ReferenceOr<T: Clone> {
  Reference {
    #[serde(rename = "$ref")]
    _ref: String,
  },
  Object(T),
}

impl From<Schema> for ReferenceOr<Schema> {
  fn from(value: Schema) -> Self {
    Self::Object(value)
  }
}

impl From<String> for ReferenceOr<String> {
  fn from(value: String) -> Self {
    Self::Reference { _ref: value }
  }
}
