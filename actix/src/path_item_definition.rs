use std::collections::BTreeMap;
use utoipa::openapi::{Components, PathItem};

pub trait PathItemDefinition {
  fn is_visible() -> bool;
  fn path_item(_default_tag: Option<&str>) -> PathItem;
  fn components() -> BTreeMap<String, Components>;
}
