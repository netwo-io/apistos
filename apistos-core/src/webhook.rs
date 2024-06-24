use std::collections::BTreeMap;

use apistos_models::components::Components;
use apistos_models::paths::PathItem;
use apistos_models::reference_or::ReferenceOr;
use apistos_models::OpenApiVersion;

pub trait ApiWebhook {
  fn webhooks(_oas_version: OpenApiVersion) -> BTreeMap<String, ReferenceOr<PathItem>> {
    Default::default()
  }

  fn components(_oas_version: OpenApiVersion) -> Vec<Components> {
    Default::default()
  }
}
