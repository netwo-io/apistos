use crate::paths::{Callback, Example, Header, Link, Parameter, RequestBody, Response};
use crate::reference_or::ReferenceOr;
use crate::security::SecurityScheme;
use indexmap::IndexMap;
use schemars::schema::Schema;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

/// Holds a set of reusable objects for different aspects of the OAS. All objects defined within the components object will have no effect on the API unless they are explicitly referenced from properties outside the components object.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Components {
  /// An object to hold reusable [Schema Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#schema-object).
  pub schemas: BTreeMap<String, ReferenceOr<Schema>>,
  /// An object to hold reusable [Response Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#response-object).
  pub responses: BTreeMap<String, ReferenceOr<Response>>,
  /// An object to hold reusable [Parameter Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameter-object).
  pub parameters: BTreeMap<String, ReferenceOr<Parameter>>,
  /// 	An object to hold reusable [Example Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#example-object).
  pub examples: BTreeMap<String, ReferenceOr<Example>>,
  /// An object to hold reusable [Request Body Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#request-body-object).
  pub request_bodies: BTreeMap<String, ReferenceOr<RequestBody>>,
  /// An object to hold reusable [Header Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#header-object).
  pub headers: BTreeMap<String, ReferenceOr<Header>>,
  /// An object to hold reusable [Security Scheme Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#security-scheme-object).
  pub security_schemes: BTreeMap<String, ReferenceOr<SecurityScheme>>,
  /// An object to hold reusable [Link Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#link-object).
  pub links: BTreeMap<String, ReferenceOr<Link>>,
  /// An object to hold reusable [Callback Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#callback-object).
  pub callbacks: BTreeMap<String, ReferenceOr<Callback>>,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten)]
  pub extensions: IndexMap<String, Value>,
}
