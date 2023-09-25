use netwopenapi_models::paths::Response;
use netwopenapi_models::reference_or::ReferenceOr;
use netwopenapi_models::Schema;
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
