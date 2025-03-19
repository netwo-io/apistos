use apistos_models::Schema;
use apistos_models::paths::Response;
use apistos_models::reference_or::ReferenceOr;
use std::collections::BTreeMap;

pub trait ApiErrorComponent {
  fn schemas_by_status_code() -> BTreeMap<String, (String, ReferenceOr<Schema>)>;
  fn error_responses() -> Vec<(String, Response)>;
}

#[cfg(feature = "actix")]
impl ApiErrorComponent for actix_web::error::Error {
  fn schemas_by_status_code() -> BTreeMap<String, (String, ReferenceOr<Schema>)> {
    BTreeMap::default()
  }

  fn error_responses() -> Vec<(String, Response)> {
    vec![]
  }
}
