use indexmap::IndexMap;
use serde::Serialize;
use serde_json::Value;

/// The object provides metadata about the API. The metadata MAY be used by the clients if needed, and MAY be presented in editing or documentation generation tools for convenience.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Info {
  /// The title of the API
  pub title: String,
  /// # ⚠️ OAS 3.1 only
  /// A short summary of the API.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub summary: Option<String>,
  /// A short description of the API. [CommonMark](https://spec.commonmark.org/) syntax MAY be used for rich text representation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  /// A URL to the Terms of Service for the API. MUST be in the format of a URL.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub terms_of_service: Option<String>,
  /// The contact information for the exposed API.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub contact: Option<Contact>,
  /// The license information for the exposed API.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub license: Option<License>,
  /// # OAS 3.0
  /// The version of the OpenAPI document (which is distinct from the [OpenAPI Specification version](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#oasVersion) or the API implementation version).
  /// # OAS 3.1
  /// The version of the OpenAPI document (which is distinct from the [OpenAPI Specification version](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#oasVersion) or the API implementation version).
  pub version: String,
  /// # OAS 3.0
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  /// # OAS 3.1
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

/// Contact information for the exposed API.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Contact {
  /// The identifying name of the contact person/organization.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// The URL pointing to the contact information. MUST be in the format of a URL.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,
  /// The email address of the contact person/organization. MUST be in the format of an email address.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  /// # OAS 3.0
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  /// # OAS 3.1
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

/// License information for the exposed API.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct License {
  /// The license name used for the API.
  pub name: String,
  /// # ⚠️ OAS 3.1 only
  #[serde(flatten)]
  pub identifier: Option<LicenseIdentifier>,
  /// # ⚠️ OAS 3.0 only
  /// A URL to the license used for the API. MUST be in the format of a URL.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub url: Option<String>,
  /// # OAS 3.0
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  /// # OAS 3.1
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

impl License {
  pub fn new_3_0_license(name: String, url: Option<String>, extensions: IndexMap<String, Value>) -> Self {
    Self {
      name,
      url,
      extensions,
      ..Default::default()
    }
  }

  pub fn new_3_1_license(
    name: String,
    identifier: Option<LicenseIdentifier>,
    extensions: IndexMap<String, Value>,
  ) -> Self {
    Self {
      name,
      identifier,
      extensions,
      ..Default::default()
    }
  }
}

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub enum LicenseIdentifier {
  /// An [SPDX](https://spdx.org/licenses/) license expression for the API. The identifier field is mutually exclusive of the `url` field.
  Identifier(String),
  /// A URL to the license used for the API. This MUST be in the form of a URL. The `url` field is mutually exclusive of the `identifier` field.
  Url(String),
}
