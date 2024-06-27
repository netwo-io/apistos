use std::collections::BTreeMap;

use apistos_models::components::Components;
use apistos_models::paths::PathItem;
use apistos_models::reference_or::ReferenceOr;
use apistos_models::OpenApiVersion;

#[derive(Clone, Debug, Default)]
pub struct ApiWebhookDef {
  pub components: Vec<Components>,
  pub webhooks: BTreeMap<String, ReferenceOr<PathItem>>
}

pub trait ApiWebhook {
  fn webhooks(&self, _oas_version: OpenApiVersion) -> BTreeMap<String, ReferenceOr<PathItem>> {
    Default::default()
  }

  fn components(&self, _oas_version: OpenApiVersion) -> Vec<Components> {
    Default::default()
  }

  fn get_def(_oas_version: OpenApiVersion) -> ApiWebhookDef {
    Default::default()
  }
}
