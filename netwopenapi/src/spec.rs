use netwopenapi_models::info::Info;
use netwopenapi_models::paths::ExternalDocumentation;
use netwopenapi_models::server::Server;
use netwopenapi_models::tag::Tag;

#[derive(Default)]
pub struct Spec {
  pub info: Info,
  pub default_tags: Vec<String>,
  /// See more details at <https://spec.openapis.org/oas/latest.html#tagObject>.
  pub tags: Vec<Tag>,
  /// See more details at <https://spec.openapis.org/oas/latest.html#external-documentation-object>.
  pub external_docs: Option<ExternalDocumentation>,
  /// See more details at <https://spec.openapis.org/oas/latest.html#server-object>.
  pub servers: Vec<Server>,
}
