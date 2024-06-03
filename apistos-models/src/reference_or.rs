use log::warn;
use schemars::Schema;
use serde::Serialize;
use serde_json::Value;

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

impl From<Value> for ReferenceOr<Schema> {
  fn from(value: Value) -> Self {
    let schema = Schema::try_from(value.clone());
    match schema {
      Ok(sch) => Self::Object(sch),
      Err(e) => {
        warn!("Error converting value to schema: get {e:?} for value {value:?}");
        Self::Object(Schema::default())
      }
    }
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
