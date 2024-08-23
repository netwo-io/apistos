use indexmap::IndexMap;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

/// # OAS 3.0
/// Lists the required security schemes to execute this operation. The name used for each property MUST correspond to a security scheme declared in the [Security Schemes](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#componentsSecuritySchemes) under the [Components Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#components-object).
///
/// Security Requirement Objects that contain multiple schemes require that all schemes MUST be satisfied for a request to be authorized. This enables support for scenarios where multiple query parameters or HTTP headers are required to convey security information.
///
/// When a list of Security Requirement Objects is defined on the [OpenAPI Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#openapi-object) or [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#operation-object), only one of the Security Requirement Objects in the list needs to be satisfied to authorize the request.
/// # OAS 3.1
/// Lists the required security schemes to execute this operation. The name used for each property MUST correspond to a security scheme declared in the [Security Schemes](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#componentsSecuritySchemes) under the [Components Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#components-object).
///
/// Security Requirement Objects that contain multiple schemes require that all schemes MUST be satisfied for a request to be authorized. This enables support for scenarios where multiple query parameters or HTTP headers are required to convey security information.
///
/// When a list of Security Requirement Objects is defined on the [OpenAPI Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#openapi-object) or [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#operation-object), only one of the Security Requirement Objects in the list needs to be satisfied to authorize the request.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct SecurityRequirement {
  /// # OAS 3.0
  /// Each name MUST correspond to a security scheme which is declared in the [Security Schemes](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#componentsSecuritySchemes) under the [Components Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#components-object). If the security scheme is of type `"oauth2"` or `"openIdConnect"`, then the value is a list of scope names required for the execution, and the list MAY be empty if authorization does not require a specified scope. For other security scheme types, the array MUST be empty.
  /// # OAS 3.1
  /// Each name MUST correspond to a security scheme which is declared in the [Security Schemes](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#componentsSecuritySchemes) under the [Components Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#components-object). If the security scheme is of type `"oauth2"` or `"openIdConnect"`, then the value is a list of scope names required for the execution, and the list MAY be empty if authorization does not require a specified scope. For other security scheme types, the array MAY contain a list of role names which are required for the execution, but are not otherwise defined or exchanged in-band.
  #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
  pub requirements: BTreeMap<String, Vec<String>>,
}

/// Defines a security scheme that can be used by the operations. Supported schemes are HTTP authentication, an API key (either as a header, a cookie parameter or as a query parameter), `OAuth2`'s common flows (implicit, password, client credentials and authorization code) as defined in [RFC6749](https://datatracker.ietf.org/doc/html/rfc6749), and [OpenID Connect Discovery](https://datatracker.ietf.org/doc/html/draft-ietf-oauth-discovery-06).
#[derive(Serialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct SecurityScheme {
  /// The type of the security scheme. Valid values are `"apiKey"`, `"http"`, `"oauth2"`, `"openIdConnect"`.
  #[serde(flatten)]
  pub _type: SecurityType,
  /// A short description for security scheme. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  /// # OAS 3.0
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  /// # OAS 3.1
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SecurityType {
  ApiKey(ApiKey),
  Http(Http),
  #[serde(rename = "oauth2")]
  OAuth2(OAuth2),
  OpenIdConnect(OpenIdConnect),
}

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
  /// The name of the header, query or cookie parameter to be used.
  pub name: String,
  /// The location of the API key. Valid values are `"query"`, `"header"` or `"cookie"`.
  #[serde(rename = "in")]
  pub _in: ApiKeyIn,
}

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub enum ApiKeyIn {
  Query,
  Header,
  Cookie,
}

#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Http {
  /// The name of the HTTP Authorization scheme to be used in the [Authorization header as defined in RFC7235](https://datatracker.ietf.org/doc/html/rfc7235#section-5.1). The values used SHOULD be registered in the [IANA Authentication Scheme registry](https://www.iana.org/assignments/http-authschemes/http-authschemes.xhtml).
  pub scheme: String,
  /// A hint to the client to identify how the bearer token is formatted. Bearer tokens are usually generated by an authorization server, so this information is primarily for documentation purposes.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bearer_format: Option<String>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct OAuth2 {
  /// An object containing configuration information for the flow types supported.
  pub flows: OauthFlows,
}

/// Allows configuration of the supported OAuth Flows.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct OauthFlows {
  /// Configuration for the OAuth Implicit flow
  #[serde(skip_serializing_if = "Option::is_none")]
  pub implicit: Option<OauthImplicit>,
  /// Configuration for the OAuth Resource Owner Password flow
  #[serde(skip_serializing_if = "Option::is_none")]
  pub password: Option<OauthToken>,
  /// Configuration for the OAuth Client Credentials flow. Previously called `application` in OpenAPI 2.0.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub client_credentials: Option<OauthToken>,
  /// Configuration for the OAuth Authorization Code flow. Previously called `accessCode` in OpenAPI 2.0.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub authorization_code: Option<OauthToken>,
  /// # OAS 3.0
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  /// # OAS 3.1
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct OauthImplicit {
  /// The authorization URL to be used for this flow. This MUST be in the form of a URL.
  pub authorization_url: String,
  /// The URL to be used for obtaining refresh tokens. This MUST be in the form of a URL.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub refresh_url: Option<String>,
  /// The available scopes for the OAuth2 security scheme. A map between the scope name and a short description for it. The map MAY be empty.
  pub scopes: BTreeMap<String, String>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct OauthToken {
  /// The token URL to be used for this flow. This MUST be in the form of a URL.
  pub token_url: String,
  /// The URL to be used for obtaining refresh tokens. This MUST be in the form of a URL.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub refresh_url: Option<String>,
  /// The available scopes for the OAuth2 security scheme. A map between the scope name and a short description for it. The map MAY be empty.
  pub scopes: BTreeMap<String, String>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct OpenIdConnect {
  /// OpenId Connect URL to discover OAuth2 configuration values. This MUST be in the form of a URL.
  pub open_id_connect_url: String,
}
