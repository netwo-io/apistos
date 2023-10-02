use assert_json_diff::assert_json_eq;
use serde_json::json;

use netwopenapi_core::ApiComponent;
use netwopenapi_gen::ApiSecurity;

#[test]
#[allow(dead_code)]
fn api_security_derive_api_key() {
  #[derive(ApiSecurity)]
  #[openapi_security(scheme(security_type(api_key(name = "api_key", api_key_in = "header"))))]
  struct ApiKeyTest;

  let securities = ApiKeyTest::securities();
  assert_eq!(securities.len(), 1);
  let security_scheme = securities.get("api_key_test").expect("Unable to find security scheme");
  let json = serde_json::to_value(security_scheme).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "in": "header",
      "name": "api_key",
      "type": "apiKey"
    })
  );

  #[derive(ApiSecurity)]
  #[openapi_security(
    name = "my_api_key",
    scheme(security_type(api_key(name = "api_key", api_key_in = "header")))
  )]
  struct ApiKey;

  let securities = ApiKey::securities();
  assert_eq!(securities.len(), 1);
  let security_scheme = securities.get("my_api_key").expect("Unable to find security scheme");
  let json = serde_json::to_value(security_scheme).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "in": "header",
      "name": "api_key",
      "type": "apiKey"
    })
  )
}

#[test]
#[allow(dead_code)]
fn api_security_derive_oauth2() {
  #[derive(ApiSecurity)]
  #[openapi_security(scheme(security_type(oauth2(flows(implicit(
    authorization_url = "https://authorize.com",
    refresh_url = "https://refresh.com",
    scopes(scope = "all:read", description = "Read all the things"),
    scopes(scope = "all:write", description = "Write all the things")
  ))))))]
  pub struct ApiKey;

  let securities = ApiKey::securities();
  assert_eq!(securities.len(), 1);
  let security_scheme = securities.get("api_key").expect("Unable to find security scheme");
  let json = serde_json::to_value(security_scheme).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "flows": {
        "implicit": {
          "authorizationUrl": "https://authorize.com",
          "refreshUrl": "https://refresh.com",
          "scopes": {
            "all:read": "Read all the things",
            "all:write": "Write all the things"
          }
        },
      },
      "type": "oauth2"
    })
  );

  #[derive(ApiSecurity)]
  #[openapi_security(scheme(security_type(oauth2(flows(password(
    token_url = "https://token.com",
    refresh_url = "https://refresh.com",
    scopes(scope = "all:read", description = "Read all the things"),
    scopes(scope = "all:write", description = "Write all the things")
  ))))))]
  pub struct ApiKey2;

  let securities = ApiKey2::securities();
  assert_eq!(securities.len(), 1);
  let security_scheme = securities.get("api_key_2").expect("Unable to find security scheme");
  let json = serde_json::to_value(security_scheme).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "flows": {
        "password": {
          "tokenUrl": "https://token.com",
          "refreshUrl": "https://refresh.com",
          "scopes": {
            "all:read": "Read all the things",
            "all:write": "Write all the things"
          }
        },
      },
      "type": "oauth2"
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_security_derive_http() {
  #[derive(ApiSecurity)]
  #[openapi_security(scheme(security_type(http(scheme = "bearer", bearer_format = "JWT"))))]
  pub struct ApiKeyHttp;

  let securities = ApiKeyHttp::securities();
  assert_eq!(securities.len(), 1);
  let security_scheme = securities.get("api_key_http").expect("Unable to find security scheme");
  let json = serde_json::to_value(security_scheme).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "bearerFormat": "JWT",
      "scheme": "bearer",
      "type": "http"
    })
  );
}

#[test]
#[allow(dead_code)]
fn api_security_derive_openid_connect() {
  #[derive(ApiSecurity)]
  #[openapi_security(scheme(security_type(open_id_connect(open_id_connect_url = "https://connect.com"))))]
  pub struct ApiKeyOpenIdConnect;

  let securities = ApiKeyOpenIdConnect::securities();
  assert_eq!(securities.len(), 1);
  let security_scheme = securities
    .get("api_key_open_id_connect")
    .expect("Unable to find security scheme");
  let json = serde_json::to_value(security_scheme).expect("Unable to serialize as Json");
  assert_json_eq!(
    json,
    json!({
      "openIdConnectUrl": "https://connect.com",
      "type": "openIdConnect"
    })
  );
}
