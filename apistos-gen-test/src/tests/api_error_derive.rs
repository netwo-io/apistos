use apistos_core::ApiErrorComponent;
use apistos_gen::ApiErrorComponent;
use schemars::JsonSchema;
use std::collections::{BTreeMap, HashSet};

#[test]
#[allow(dead_code)]
fn api_component_derive() {
  #[allow(clippy::duplicated_attributes)]
  #[derive(ApiErrorComponent)]
  #[openapi_error(
    status(code = 403),
    status(code = 404),
    status(code = 405, description = "Invalid input"),
    status(code = 409)
  )]
  enum ErrorResponse {
    MethodNotAllowed(String),
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
  }

  let error_schemas = <ErrorResponse as ApiErrorComponent>::schemas_by_status_code();
  let error_responses = <ErrorResponse as ApiErrorComponent>::error_responses();
  assert!(error_schemas.is_empty());
  assert!(!error_responses.is_empty());
  assert_eq!(error_responses.len(), 4);

  let err_codes: HashSet<&str> = error_responses.iter().map(|(code, _)| code.as_str()).collect();
  assert_eq!(err_codes.len(), 4);
  let expected_err_codes = ["403", "404", "405", "409"];
  assert!(err_codes.iter().all(|code| expected_err_codes.contains(code)));

  let error_responses = BTreeMap::from_iter(error_responses);
  assert_eq!(
    error_responses.get("403").map(|r| r.description.clone()),
    Some("Forbidden".to_string())
  );
  assert_eq!(
    error_responses.get("404").map(|r| r.description.clone()),
    Some("Not Found".to_string())
  );
  assert_eq!(
    error_responses.get("405").map(|r| r.description.clone()),
    Some("Invalid input".to_string())
  );
  assert_eq!(
    error_responses.get("409").map(|r| r.description.clone()),
    Some("Conflict".to_string())
  );
}

#[test]
#[allow(dead_code)]
fn api_component_with_schema() {
  #[derive(JsonSchema)]
  struct AuthorizeError {
    reason: String,
    code: String,
  }

  #[allow(clippy::duplicated_attributes)]
  #[derive(ApiErrorComponent)]
  #[openapi_error(status(code = 403), status(code = 409, description = "Too many requests"))]
  enum ErrorResponse {
    Forbidden(AuthorizeError),
    Conflict(String),
  }

  let error_schemas = <ErrorResponse as ApiErrorComponent>::schemas_by_status_code();
  let error_responses = <ErrorResponse as ApiErrorComponent>::error_responses();
  assert!(error_schemas.is_empty());
  assert!(!error_responses.is_empty());
  assert_eq!(error_responses.len(), 2);

  let err_codes: HashSet<&str> = error_responses.iter().map(|(code, _)| code.as_str()).collect();
  assert_eq!(err_codes.len(), 2);
  let expected_err_codes = ["403", "409"];
  assert!(err_codes.iter().all(|code| expected_err_codes.contains(code)));

  let error_responses = BTreeMap::from_iter(error_responses);
  assert_eq!(
    error_responses.get("403").map(|r| r.description.clone()),
    Some("Forbidden".to_string())
  );
  assert!(
    error_responses
      .get("403")
      .map(|r| r.content.clone())
      .unwrap_or_default()
      .is_empty()
  );
  assert_eq!(
    error_responses.get("409").map(|r| r.description.clone()),
    Some("Too many requests".to_string())
  );
  assert!(
    error_responses
      .get("409")
      .map(|r| r.content.clone())
      .unwrap_or_default()
      .is_empty()
  );
}
