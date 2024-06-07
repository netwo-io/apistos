use std::sync::RwLock;

use once_cell::sync::Lazy;

use apistos_models::OpenApiVersion;

pub(crate) mod actix;
pub(crate) mod definition_holder;

static GLOBAL_OAS_VERSION: Lazy<RwLock<OpenApiVersion>> = Lazy::new(|| RwLock::new(OpenApiVersion::default()));

pub(crate) fn set_oas_version(oas_version: OpenApiVersion) {
  let _lock = GLOBAL_OAS_VERSION
    .write()
    .map(|mut global_oas_version| *global_oas_version = oas_version);
}

pub(crate) fn get_oas_version() -> OpenApiVersion {
  GLOBAL_OAS_VERSION.read().map(|v| *v).unwrap_or_default()
}
