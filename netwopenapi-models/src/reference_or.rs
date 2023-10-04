use schemars::schema::Schema;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(untagged)]
pub enum ReferenceOr<T: Clone> {
  Object(T),
  Reference {
    #[serde(rename = "$ref")]
    _ref: String,
  },
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

impl<T: Clone> ReferenceOr<T> {
  pub fn get_object(self) -> Option<T> {
    match self {
      ReferenceOr::Reference { .. } => None,
      ReferenceOr::Object(p) => Some(p),
    }
  }

  pub fn get_object_mut(&mut self) -> Option<&mut T> {
    match self {
      ReferenceOr::Reference { .. } => None,
      ReferenceOr::Object(p) => Some(p),
    }
  }
}
