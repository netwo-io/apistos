use std::collections::BTreeMap;
use utoipa::openapi::path::Operation;
use utoipa::openapi::{Components, PathItem};

pub trait PathItemDefinition {
  fn is_visible() -> bool {
    true
  }

  fn operation() -> Operation {
    Default::default()
  }

  fn components() -> Vec<Components> {
    Default::default()
  }
}
