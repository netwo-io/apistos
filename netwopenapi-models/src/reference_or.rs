use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ReferenceOr<T: Serialize + Clone> {
  Reference {
    #[serde(rename = "$ref")]
    _ref: String,
  },
  Object(T),
}
