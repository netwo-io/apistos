use std::collections::BTreeMap;

use apistos_models::paths::Response;
use apistos_models::reference_or::ReferenceOr;
use apistos_models::Schema;

pub trait ApiErrorComponent {
  fn schemas_by_status_code(
    oas_version: apistos_models::OpenApiVersion,
  ) -> BTreeMap<String, (String, ReferenceOr<Schema>)>;
  fn error_responses(oas_version: apistos_models::OpenApiVersion) -> Vec<(String, Response)>;
}

#[cfg(feature = "actix")]
impl ApiErrorComponent for actix_web::error::Error {
  fn schemas_by_status_code(_: apistos_models::OpenApiVersion) -> BTreeMap<String, (String, ReferenceOr<Schema>)> {
    BTreeMap::default()
  }

  fn error_responses(_: apistos_models::OpenApiVersion) -> Vec<(String, Response)> {
    vec![]
  }
}
