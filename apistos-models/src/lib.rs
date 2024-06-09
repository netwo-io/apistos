//! [OAS 3.0](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md) models over [schemars](https://github.com/GREsau/schemars)'s [`Schema`](https://docs.rs/schemars/latest/schemars/schema/enum.Schema.html).
//!
//! These models are not linked to any web framework.

// schemars::Schema does not implement Eq
#![allow(clippy::derive_partial_eq_without_eq)]

use std::collections::BTreeMap;
use std::fmt::Debug;

use indexmap::IndexMap;
pub use schemars::*;
use serde::Serialize;
use serde_json::Value;

use crate::components::Components;
use crate::info::Info;
use crate::paths::{ExternalDocumentation, PathItem, Paths};
use crate::reference_or::ReferenceOr;
use crate::security::SecurityRequirement;
use crate::server::Server;
use crate::tag::Tag;

pub mod components;
pub mod info;
pub mod paths;
pub mod reference_or;
pub mod security;
pub mod server;
pub mod tag;

#[derive(Serialize, Clone, Copy, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
pub enum OpenApiVersion {
  #[serde(rename = "3.0.3")]
  OAS3_0,
  #[serde(rename = "3.1")]
  OAS3_1,
}

impl Default for OpenApiVersion {
  fn default() -> Self {
    Self::OAS3_0
  }
}

/// This is the root document object of the [OpenAPI document](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#openapi-document).
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct OpenApi {
  /// # OAS 3.0
  /// This string MUST be the [semantic version number](https://semver.org/spec/v2.0.0.html) of the [OpenAPI Specification version](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#versions) that the OpenAPI document uses. The `openapi` field SHOULD be used by tooling specifications and clients to interpret the OpenAPI document. This is not related to the API [**`info.version`**](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#infoVersion) string.
  /// # OAS 3.1
  /// This string MUST be the [version number](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#versions) of the OpenAPI Specification that the OpenAPI document uses. The `openapi` field SHOULD be used by tooling to interpret the OpenAPI document. This is not related to the API [**`info.version`**](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#infoVersion) string.
  pub openapi: OpenApiVersion,
  /// Provides metadata about the API. The metadata MAY be used by tooling as required.
  pub info: Info,
  /// # ⚠️ OAS 3.1 only
  /// The default value for the $schema keyword within [Schema Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#schema-object). contained within this OAS document. This MUST be in the form of a URI.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub json_schema_dialect: Option<String>,
  /// # OAS 3.0
  /// An array of Server Objects, which provide connectivity information to a target server. If the `servers` property is not provided, or is an empty array, the default value would be a [Server Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#server-object) with a [url](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#serverUrl) value of `/`.
  /// # OAS 3.1
  /// An array of Server Objects, which provide connectivity information to a target server. If the `servers` property is not provided, or is an empty array, the default value would be a [Server Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#server-object) with a [url](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#serverUrl) value of `/`.
  pub servers: Vec<Server>,
  /// The available paths and operations for the API.
  pub paths: Paths,
  /// # ⚠️ OAS 3.1 only
  /// The incoming webhooks that MAY be received as part of this API and that the API consumer MAY choose to implement. Closely related to the `callbacks` feature, this section describes requests initiated other than by an API call, for example by an out of band registration. The key name is a unique string to refer to each webhook, while the (optionally referenced) Path Item Object describes a request that may be initiated by the API provider and the expected responses. An [example](https://github.com/OAI/OpenAPI-Specification/blob/main/examples/v3.1/webhook-example.yaml) is available.
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub webhooks: BTreeMap<String, ReferenceOr<PathItem>>,
  /// An element to hold various schemas for the specification.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub components: Option<Components>,
  /// A declaration of which security mechanisms can be used across the API. The list of values includes alternative security requirement objects that can be used. Only one of the security requirement objects need to be satisfied to authorize a request. Individual operations can override this definition. To make security optional, an empty security requirement (`{}`) can be included in the array.
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub security: Vec<SecurityRequirement>,
  /// # OAS 3.0
  /// A list of tags used by the specification with additional metadata. The order of the tags can be used to reflect on their order by the parsing tools. Not all tags that are used by the [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#operation-object) must be declared. The tags that are not declared MAY be organized randomly or based on the tools' logic. Each tag name in the list MUST be unique.
  /// # OAS 3.1
  /// A list of tags used by the document with additional metadata. The order of the tags can be used to reflect on their order by the parsing tools. Not all tags that are used by the [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#operation-object) must be declared. The tags that are not declared MAY be organized randomly or based on the tools' logic. Each tag name in the list MUST be unique.
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub tags: Vec<Tag>,
  /// Additional external documentation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub external_docs: Option<ExternalDocumentation>,
  /// # OAS 3.0
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  /// # OAS 3.1
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
pub enum VersionSpecificSchema {
  OAS3_0(ReferenceOr<Schema>),
  OAS3_1(Schema),
}

#[cfg(test)]
mod test {
  #![allow(clippy::expect_used)]

  use std::collections::BTreeMap;

  use indexmap::IndexMap;

  use crate::info::Info;
  use crate::paths::{Operation, OperationType, PathItem, Paths, Response, Responses};
  use crate::reference_or::ReferenceOr;
  use crate::server::Server;
  use crate::tag::Tag;
  use crate::{OpenApi, OpenApiVersion};

  #[test]
  fn empty_openapi_properly_generated() {
    let oas = OpenApi {
      openapi: OpenApiVersion::OAS3_0,
      info: Info {
        title: "Test".to_string(),
        description: Some("Description".to_string()),
        version: "1.0.0".to_string(),
        ..Default::default()
      },
      paths: Paths::default(),
      ..Default::default()
    };

    let oas_json = serde_json::to_string_pretty(&oas).expect("Error generating json for oas");
    assert_eq!(oas_json, include_str!("../test-assets/empty-openapi.json"));
  }

  #[test]
  fn openapi_properly_generated() {
    let oas = OpenApi {
      openapi: OpenApiVersion::OAS3_0,
      info: Info {
        title: "Test".to_string(),
        description: Some("Description".to_string()),
        version: "1.0.0".to_string(),
        ..Default::default()
      },
      servers: vec![Server {
        url: "https://google.com".to_string(),
        description: Some("A big search server".to_string()),
        ..Default::default()
      }],
      paths: Paths {
        paths: IndexMap::from_iter(vec![(
          "/search".to_string(),
          PathItem {
            operations: IndexMap::from_iter(vec![(
              OperationType::Get,
              Operation {
                tags: vec!["Search".to_string()],
                summary: Some("I don't know what this do".to_string()),
                operation_id: Some("get_search".to_string()),
                responses: Responses {
                  responses: BTreeMap::from_iter(vec![(
                    "200".to_string(),
                    ReferenceOr::Object(Response {
                      description: "A search thingy".to_string(),
                      ..Default::default()
                    }),
                  )]),
                  ..Default::default()
                },
                ..Default::default()
              },
            )]),
            ..Default::default()
          },
        )]),
        ..Default::default()
      },
      tags: vec![Tag {
        name: "Search".to_string(),
        ..Default::default()
      }],
      ..Default::default()
    };

    let oas_json = serde_json::to_string_pretty(&oas).expect("Error generating json for oas");
    assert_eq!(oas_json, include_str!("../test-assets/openapi.json"));
  }
}
