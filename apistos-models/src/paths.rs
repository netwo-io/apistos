use crate::reference_or::ReferenceOr;
use crate::security::SecurityRequirement;
use crate::server::Server;
use indexmap::IndexMap;
use schemars::schema::Schema;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Paths {
  #[serde(flatten)]
  pub paths: IndexMap<String, PathItem>,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

/// Describes the operations available on a single path. A Path Item MAY be empty, due to [ACL constraints](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#security-filtering). The path itself is still exposed to the documentation viewer but they will not know which operations and parameters are available.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct PathItem {
  /// An optional, string summary, intended to apply to all operations in this path.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub summary: Option<String>,
  /// An optional, string description, intended to apply to all operations in this path. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(flatten)]
  pub operations: IndexMap<OperationType, Operation>,
  /// An alternative `server` array to service all operations in this path.
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub server: Vec<Server>,
  /// A list of parameters that are applicable for all the operations described under this path. These parameters can be overridden at the operation level, but cannot be removed there. The list MUST NOT include duplicated parameters. A unique parameter is defined by a combination of a [name](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterName) and [location](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterIn). The list can use the [Reference Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#reference-object) to link to parameters that are defined at the [OpenAPI Object's components/parameters](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#componentsParameters).
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub parameters: Vec<ReferenceOr<Parameter>>,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

#[derive(Serialize, Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize))]
#[serde(rename_all = "lowercase")]
pub enum OperationType {
  /// A definition of a GET operation on this path.
  Get,
  /// A definition of a PUT operation on this path.
  Put,
  /// A definition of a POST operation on this path.
  Post,
  /// A definition of a DELETE operation on this path.
  Delete,
  /// A definition of a OPTIONS operation on this path.
  Options,
  /// A definition of a HEAD operation on this path.
  Head,
  /// A definition of a PATCH operation on this path.
  Patch,
  /// A definition of a TRACE operation on this path.
  Trace,
}

/// Describes a single API operation on a path.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Operation {
  /// A list of tags for API documentation control. Tags can be used for logical grouping of operations by resources or any other qualifier.
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub tags: Vec<String>,
  /// A short summary of what the operation does.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub summary: Option<String>,
  /// A verbose explanation of the operation behavior. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  /// Additional external documentation for this operation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub external_docs: Option<ExternalDocumentation>,
  /// Unique string used to identify the operation. The id MUST be unique among all operations described in the API. The operationId value is case-sensitive. Tools and libraries MAY use the operationId to uniquely identify an operation, therefore, it is RECOMMENDED to follow common programming naming conventions.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub operation_id: Option<String>,
  /// A list of parameters that are applicable for this operation. If a parameter is already defined at the [Path Item](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#pathItemParameters), the new definition will override it but can never remove it. The list MUST NOT include duplicated parameters. A unique parameter is defined by a combination of a [name](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterName) and [location](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterIn). The list can use the [Reference Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#reference-object) to link to parameters that are defined at the [OpenAPI Object's components/parameters](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#componentsParameters).
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub parameters: Vec<ReferenceOr<Parameter>>,
  /// The request body applicable for this operation. The `requestBody` is only supported in HTTP methods where the HTTP 1.1 specification [RFC7231](https://datatracker.ietf.org/doc/html/rfc7231#section-4.3.1) has explicitly defined semantics for request bodies. In other cases where the HTTP spec is vague, `requestBody` SHALL be ignored by consumers.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub request_body: Option<ReferenceOr<RequestBody>>,
  /// The list of possible responses as they are returned from executing this operation.
  pub responses: Responses,
  /// A map of possible out-of band callbacks related to the parent operation. The key is a unique identifier for the [Callback Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#callback-object). Each value in the map is a Callback Object that describes a request that may be initiated by the API provider and the expected responses.
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub callbacks: BTreeMap<String, ReferenceOr<Callback>>,
  /// Declares this operation to be deprecated. Consumers SHOULD refrain from usage of the declared operation. Default value is `false`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deprecated: Option<bool>,
  /// A declaration of which security mechanisms can be used for this operation. The list of values includes alternative security requirement objects that can be used. Only one of the security requirement objects need to be satisfied to authorize a request. To make security optional, an empty security requirement (`{}`) can be included in the array. This definition overrides any declared top-level [`security`](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#oasSecurity). To remove a top-level security declaration, an empty array can be used.
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub security: Vec<SecurityRequirement>,
  /// An alternative `server` array to service this operation. If an alternative `server` object is specified at the Path Item Object or Root level, it will be overridden by this value.
  #[serde(skip_serializing_if = "Vec::is_empty", default)]
  pub servers: Vec<Server>,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

/// Allows referencing an external resource for extended documentation.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct ExternalDocumentation {
  /// A short description of the target documentation. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  /// The URL for the target documentation. Value MUST be in the format of a URL.
  pub url: String,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

/// Describes a single operation parameter.
/// A unique parameter is defined by a combination of a [name](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterName) and [location](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterIn).
/// Parameter Locations
/// # There are four possible parameter locations specified by the in field:
///
/// - path - Used together with [Path Templating](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#path-templating), where the parameter value is actually part of the operation's URL. This does not include the host or base path of the API. For example, in `/items/{itemId}`, the path parameter is `itemId`.
/// - query - Parameters that are appended to the URL. For example, in `/items?id=###`, the query parameter is `id`.
/// - header - Custom headers that are expected as part of the request. Note that [RFC7230](https://datatracker.ietf.org/doc/html/rfc7230#page-22) states header names are case insensitive.
/// - cookie - Used to pass a specific cookie value to the API.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
  /// The name of the parameter. Parameter names are case sensitive.
  /// - If [`in`](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterIn) is `"path"`, the `name` field MUST correspond to a template expression occurring within the [path](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#pathsPath) field in the [Paths Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#paths-object). See [Path Templating](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#path-templating) for further information.
  /// - If [`in`](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterIn) is `"header"` and the `name` field is `"Accept"`, `"Content-Type"` or `"Authorization"`, the parameter definition SHALL be ignored.
  /// - For all other cases, the `name` corresponds to the parameter name used by the [`in`](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterIn) property.
  pub name: String,
  /// The location of the parameter. Possible values are `"query"`, `"header"`, `"path"` or `"cookie"`.
  #[serde(rename = "in")]
  pub _in: ParameterIn,
  /// A brief description of the parameter. This could contain examples of use. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  /// Determines whether this parameter is mandatory. If the [parameter location](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterIn) is `"path"`, this property is **REQUIRED** and its value MUST be `true`. Otherwise, the property MAY be included and its default value is `false`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub required: Option<bool>,
  /// Specifies that a parameter is deprecated and SHOULD be transitioned out of usage. Default value is `false`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deprecated: Option<bool>,
  /// Sets the ability to pass empty-valued parameters. This is valid only for `query` parameters and allows sending a parameter with an empty value. Default value is `false`. If [`style`](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterStyle) is used, and if behavior is `n/a` (cannot be serialized), the value of `allowEmptyValue` SHALL be ignored. Use of this property is NOT RECOMMENDED, as it is likely to be removed in a later revision.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub allow_empty_value: Option<bool>,
  /// Describes how the parameter value will be serialized depending on the type of the parameter value. Default values (based on value of `in`): for `query` - `form`; for `path` - `simple`; for `header` - `simple`; for `cookie` - `form`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub style: Option<ParameterStyle>,
  /// When this is true, parameter values of type `array` or `object` generate separate parameters for each value of the array or key-value pair of the map. For other types of parameters this property has no effect. When [`style`](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterStyle) is `form`, the default value is `true`. For all other styles, the default value is `false`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub explode: Option<bool>,
  /// Determines whether the parameter value SHOULD allow reserved characters, as defined by [RFC3986](https://datatracker.ietf.org/doc/html/rfc3986#section-2.2) `:/?#[]@!$&'()*+,;=` to be included without percent-encoding. This property only applies to parameters with an `in` value of `query`. The default value is `false`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub allow_reserved: Option<bool>,
  #[serde(flatten, skip_serializing_if = "Option::is_none")]
  pub definition: Option<ParameterDefinition>,
  #[serde(flatten, skip_serializing_if = "Option::is_none")]
  pub example: Option<Examples>,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "lowercase")]
pub enum ParameterDefinition {
  /// The schema defining the type used for the parameter.
  Schema(ReferenceOr<Schema>),
  /// A map containing the representations for the parameter. The key is the media type and the value describes it. The map MUST only contain one entry.
  Content(BTreeMap<String, MediaType>),
}

/// Each Media Type Object provides schema and examples for the media type identified by its key.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct MediaType {
  /// The schema defining the content of the request, response, or parameter.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub schema: Option<ReferenceOr<Schema>>,
  #[serde(flatten)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub example: Option<Examples>,
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub encoding: BTreeMap<String, Encoding>,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

/// A single encoding definition applied to a single schema property.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Encoding {
  /// The Content-Type for encoding a specific property. Default value depends on the property type: for `string` with `format` being `binary` – `application/octet-stream`; for other primitive types – `text/plain`; for object - `application/json`; for `array` – the default is defined based on the inner type. The value can be a specific media type (e.g. `application/json`), a wildcard media type (e.g. `image/*`), or a comma-separated list of the two types.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_type: Option<String>,
  /// A map allowing additional information to be provided as headers, for example `Content-Disposition`. `Content-Type` is described separately and SHALL be ignored in this section. This property SHALL be ignored if the request body media type is not a `multipart`.
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub headers: BTreeMap<String, ReferenceOr<Header>>,
  /// Describes how a specific property value will be serialized depending on its type. See [Parameter Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameter-object) for details on the [`style`](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterStyle) property. The behavior follows the same values as `query` parameters, including default values. This property SHALL be ignored if the request body media type is not `application/x-www-form-urlencoded`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub style: Option<ParameterStyle>,
  /// When this is true, property values of type `array` or `object` generate separate parameters for each value of the array, or key-value-pair of the map. For other types of properties this property has no effect. When [`style`](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterStyle) is `form`, the default value is `true`. For all other styles, the default value is `false`. This property SHALL be ignored if the request body media type is not `application/x-www-form-urlencoded`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub explode: Option<bool>,
  /// Determines whether the parameter value SHOULD allow reserved characters, as defined by [RFC3986](https://datatracker.ietf.org/doc/html/rfc3986#section-2.2) `:/?#[]@!$&'()*+,;=` to be included without percent-encoding. The default value is `false`. This property SHALL be ignored if the request body media type is not `application/x-www-form-urlencoded`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub allow_reserved: Option<bool>,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

/// The Header Object follows the structure of the [Parameter Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameter-object) with the following changes:
///
/// 1. `name` MUST NOT be specified, it is given in the corresponding `headers` map.
/// 2. `in` MUST NOT be specified, it is implicitly in header.
/// 3. All traits that are affected by the location MUST be applicable to a location of `header` (for example, [`style`](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterStyle)).
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Header {
  /// Determines whether this parameter is mandatory. If the [parameter location](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterIn) is `"path"`, this property is **REQUIRED** and its value MUST be `true`. Otherwise, the property MAY be included and its default value is `false`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub required: Option<bool>,
  /// Specifies that a parameter is deprecated and SHOULD be transitioned out of usage. Default value is `false`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deprecated: Option<bool>,
  /// A brief description of the parameter. This could contain examples of use. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(flatten, skip_serializing_if = "Option::is_none")]
  pub definition: Option<ParameterDefinition>,
  /// Describes how the parameter value will be serialized depending on the type of the parameter value. Default values (based on value of `in`): for `query` - `form`; for `path` - `simple`; for `header` - `simple`; for `cookie` - `form`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub style: Option<ParameterStyle>,
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize))]
#[serde(rename_all = "lowercase")]
pub enum ParameterIn {
  Query,
  Header,
  Path,
  Cookie,
}

impl Default for ParameterIn {
  fn default() -> Self {
    Self::Path
  }
}

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub enum ParameterStyle {
  /// Path-style parameters defined by [RFC6570](https://datatracker.ietf.org/doc/html/rfc6570#section-3.2.7)
  Matrix,
  /// Label style parameters defined by [RFC6570](https://datatracker.ietf.org/doc/html/rfc6570#section-3.2.5)
  Label,
  /// Form style parameters defined by [RFC6570](https://datatracker.ietf.org/doc/html/rfc6570#section-3.2.8). This option replaces `collecztionFormat` with a `csv` (when `explode` is false) or `multi` (when `explode` is true) value from OpenAPI 2.0.
  Form,
  /// Simple style parameters defined by [RFC6570](https://datatracker.ietf.org/doc/html/rfc6570#section-3.2.2). This option replaces `collectionFormat` with a `csv` value from OpenAPI 2.0.
  Simple,
  /// Space separated array values. This option replaces `collectionFormat` equal to `ssv` from OpenAPI 2.0.
  SpaceDelimited,
  /// Pipe separated array values. This option replaces `collectionFormat` equal to `pipes` from OpenAPI 2.0.
  PipeDelimited,
  /// Provides a simple way of rendering nested objects using form parameters.
  DeepObject,
}

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub enum Examples {
  /// Example of the parameter's potential value. The example SHOULD match the specified schema and encoding properties if present. The `example` field is mutually exclusive of the `examples` field. Furthermore, if referencing a `schema` that contains an example, the example value SHALL override the example provided by the schema. To represent examples of media types that cannot naturally be represented in JSON or YAML, a string value can contain the example with escaping where necessary.
  Example(Value),
  /// Examples of the parameter's potential value. Each example SHOULD contain a value in the correct format as specified in the parameter encoding. The `examples` field is mutually exclusive of the `example` field. Furthermore, if referencing a `schema` that contains an example, the examples value SHALL override the `example` provided by the schema.
  Examples(BTreeMap<String, ReferenceOr<Example>>),
}

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Example {
  /// Short description for the example.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub summary: Option<String>,
  /// Long description for the example. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  /// Embedded literal example. The `value` field and `externalValue` field are mutually exclusive. To represent examples of media types that cannot naturally represented in JSON or YAML, use a string value to contain the example, escaping where necessary.
  #[serde(flatten)]
  pub value: ExampleValue,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub enum ExampleValue {
  /// Embedded literal example. The `value` field and `externalValue` field are mutually exclusive. To represent examples of media types that cannot naturally represented in JSON or YAML, use a string value to contain the example, escaping where necessary.
  Value(Value),
  /// A URL that points to the literal example. This provides the capability to reference examples that cannot easily be included in JSON or YAML documents. The `value` field and `externalValue` field are mutually exclusive.
  ExternalValue(String),
}

/// Describes a single request body.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
  /// A brief description of the request body. This could contain examples of use. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  /// The content of the request body. The key is a media type or [media type range](https://datatracker.ietf.org/doc/html/rfc7231#appendix-D) and the value describes it. For requests that match multiple keys, only the most specific key is applicable. e.g. text/plain overrides text/*
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub content: BTreeMap<String, MediaType>,
  /// Determines if the request body is required in the request. Defaults to `false`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub required: Option<bool>,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

/// A container for the expected responses of an operation. The container maps a HTTP response code to the expected response.
///
/// The documentation is not necessarily expected to cover all possible HTTP response codes because they may not be known in advance. However, documentation is expected to cover a successful operation response and any known errors.
///
/// The `default` MAY be used as a default response object for all HTTP codes that are not covered individually by the specification.
///
/// The `Responses Object` MUST contain at least one response code, and it SHOULD be the response for a successful operation call.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Responses {
  /// The documentation of responses other than the ones declared for specific HTTP response codes. Use this field to cover undeclared responses. A [Reference Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#reference-object) can link to a response that the [OpenAPI Object's components/responses](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#componentsResponses) section defines.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub default: Option<ReferenceOr<Response>>,
  #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
  pub responses: BTreeMap<String, ReferenceOr<Response>>,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

/// Describes a single response from an API Operation, including design-time, static `links` to operations based on the response.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Response {
  /// A short description of the response. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
  pub description: String,
  /// Maps a header name to its definition. [RFC7230](https://datatracker.ietf.org/doc/html/rfc7230#page-22) states header names are case insensitive. If a response header is defined with the name `"Content-Type"`, it SHALL be ignored.
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub headers: BTreeMap<String, ReferenceOr<Header>>,

  /// A map containing descriptions of potential response payloads. The key is a media type or [media type range](https://datatracker.ietf.org/doc/html/rfc7231#appendix-D) and the value describes it. For responses that match multiple keys, only the most specific key is applicable. e.g. text/plain overrides text/*
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub content: BTreeMap<String, MediaType>,
  /// A map of operations links that can be followed from the response. The key of the map is a short name for the link, following the naming constraints of the names for [Component Objects](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#components-object).
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub links: BTreeMap<String, ReferenceOr<Link>>,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

/// The `Link object` represents a possible design-time link for a response. The presence of a link does not guarantee the caller's ability to successfully invoke it, rather it provides a known relationship and traversal mechanism between responses and other operations.
///
/// Unlike dynamic links (i.e. links provided in the response payload), the OAS linking mechanism does not require link information in the runtime response.
///
/// For computing links, and providing instructions to execute them, a [runtime expression](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#runtime-expressions) is used for accessing values in an operation and using them as parameters while invoking the linked operation.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Link {
  #[serde(flatten, skip_serializing_if = "Option::is_none")]
  pub operation_identifier: Option<OperationIdentifier>,
  /// A map representing parameters to pass to an operation as specified with `operationId` or identified via `operationRef`. The key is the parameter name to be used, whereas the value can be a constant or an expression to be evaluated and passed to the linked operation. The parameter name can be qualified using the [parameter location](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#parameterIn) `[{in}.]{name}` for operations that use the same parameter name in different locations (e.g. path.id).
  #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
  pub parameters: BTreeMap<String, AnyOrExpression>,
  /// A literal value or [{expression}](/https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#runtime-expressions) to use as a request body when calling the target operation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub request_body: Option<AnyOrExpression>,
  /// A description of the link. [CommonMark syntax](https://spec.commonmark.org/) MAY be used for rich text representation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  /// A server object to be used by the target operation.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub server: Option<Server>,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

/// A map of possible out-of band callbacks related to the parent operation. Each value in the map is a [Path Item Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#path-item-object) that describes a set of requests that may be initiated by the API provider and the expected responses. The key value used to identify the path item object is an expression, evaluated at runtime, that identifies a URL to use for the callback operation.
#[derive(Serialize, Clone, Debug, Default)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub struct Callback {
  /// A Path Item Object used to define a callback request and expected responses. A [complete example](https://github.com/OAI/OpenAPI-Specification/blob/main/examples/v3.0/callback-example.yaml) is available.
  #[serde(flatten, skip_serializing_if = "BTreeMap::is_empty")]
  pub callbacks: BTreeMap<String, PathItem>,
  /// This object MAY be extended with [Specification Extensions](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#specification-extensions).
  #[serde(flatten, skip_serializing_if = "IndexMap::is_empty", skip_deserializing)]
  pub extensions: IndexMap<String, Value>,
}

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(untagged)]
pub enum AnyOrExpression {
  Any(Value),
  /// [{expression}](/https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#runtime-expressions)
  Expression(String),
}

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "deserialize"), derive(serde::Deserialize, PartialEq))]
#[serde(rename_all = "camelCase")]
pub enum OperationIdentifier {
  /// A relative or absolute URI reference to an OAS operation. This field is mutually exclusive of the `operationId` field, and MUST point to an [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#operation-object). Relative `operationRef` values MAY be used to locate an existing [Operation Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.3.md#operation-object) in the OpenAPI definition.
  OperationRef(String),
  /// The name of an existing, resolvable OAS operation, as defined with a unique `operationId`. This field is mutually exclusive of the `operationRef` field.
  OperationId(String),
}
