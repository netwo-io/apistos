use crate::paths::{Callback, Example, Header, Link, Parameter, PathItem, RequestBody, Response};
use crate::reference_or::ReferenceOr;
use crate::schema::ApistosSchema;
use crate::security::SecurityScheme;
use indexmap::IndexMap;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

/// Holds a set of reusable objects for different aspects of the OAS. All objects defined within the components object will have no effect on the API unless they are explicitly referenced from properties outside the components object.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Components {
  /// # OAS 3.0
  /// An object to hold reusable [Schema Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#schema-object).
  /// # OAS 3.1
  /// An object to hold reusable [Schema Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#schema-object).
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub schemas: BTreeMap<String, ReferenceOr<ApistosSchema>>,
  /// # OAS 3.0
  /// An object to hold reusable [Response Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#response-object).
  /// # OAS 3.1
  /// An object to hold reusable [Response Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#response-object).
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub responses: BTreeMap<String, ReferenceOr<Response>>,
  /// # OAS 3.0
  /// An object to hold reusable [Parameter Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameter-object).
  /// # OAS 3.1
  /// An object to hold reusable [Parameter Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#parameter-object).
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub parameters: BTreeMap<String, ReferenceOr<Parameter>>,
  /// # OAS 3.0
  /// An object to hold reusable [Example Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#example-object).
  /// # OAS 3.1
  /// An object to hold reusable [Example Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#example-object).
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub examples: BTreeMap<String, ReferenceOr<Example>>,
  /// # OAS 3.0
  /// An object to hold reusable [Request Body Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#request-body-object).
  /// # OAS 3.1
  /// An object to hold reusable [Request Body Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#request-body-object).
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub request_bodies: BTreeMap<String, ReferenceOr<RequestBody>>,
  /// # OAS 3.0
  /// An object to hold reusable [Header Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#header-object).
  /// # OAS 3.1
  /// An object to hold reusable [Header Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#header-object).
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub headers: BTreeMap<String, ReferenceOr<Header>>,
  /// # OAS 3.0
  /// An object to hold reusable [Security Scheme Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#security-scheme-object).
  /// # OAS 3.1
  /// An object to hold reusable [Security Scheme Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#security-scheme-object).
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub security_schemes: BTreeMap<String, ReferenceOr<SecurityScheme>>,
  /// # OAS 3.0
  /// An object to hold reusable [Link Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#link-object).
  /// # OAS 3.1
  /// An object to hold reusable [Link Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#link-object).
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub links: BTreeMap<String, ReferenceOr<Link>>,
  /// # OAS 3.0
  /// An object to hold reusable [Callback Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#callback-object).
  /// # OAS 3.1
  /// An object to hold reusable [Callback Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#callback-object).
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub callbacks: BTreeMap<String, ReferenceOr<Callback>>,
  /// # ⚠️ OAS 3.1 only
  /// An object to hold reusable [Path Item Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#path-item-object).
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub path_items: BTreeMap<String, ReferenceOr<PathItem>>,
  /// # OAS 3.0
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  /// # OAS 3.1
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}
