use apistos_models::components::Components;
use apistos_models::paths::Operation;
use apistos_models::OpenApiVersion;

pub trait PathItemDefinition {
  fn is_visible() -> bool {
    true
  }

  fn operation(_oas_version: OpenApiVersion) -> Operation {
    Default::default()
  }

  fn components(_oas_version: OpenApiVersion) -> Vec<Components> {
    Default::default()
  }
}
