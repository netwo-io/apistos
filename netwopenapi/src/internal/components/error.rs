use actix_web::error::Error;
use netwopenapi_models::paths::Response;
use netwopenapi_models::reference_or::ReferenceOr;
use netwopenapi_models::Schema;
use std::collections::BTreeMap;

pub trait ApiErrorComponent {
  fn schemas_by_status_code() -> BTreeMap<String, (String, ReferenceOr<Schema>)>;
  fn error_responses() -> Vec<(String, Response)>;
}

impl ApiErrorComponent for Error {
  fn schemas_by_status_code() -> BTreeMap<String, (String, ReferenceOr<Schema>)> {
    BTreeMap::default()
  }

  fn error_responses() -> Vec<(String, Response)> {
    vec![]
  }
}
