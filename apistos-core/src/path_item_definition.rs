use apistos_models::components::Components;
use apistos_models::paths::Operation;

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
