use schemars::Schema;

use apistos_core::ApiComponent;
use apistos_models::info::Info;
use apistos_models::paths::{ExternalDocumentation, Parameter};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::server::Server;
use apistos_models::tag::Tag;

/// Defines an accessor for `DefaultParameters`
pub trait DefaultParameterAccessor {
  fn get_default_parameter() -> DefaultParameters;
}

impl<T> DefaultParameterAccessor for T
where
  T: ApiComponent,
{
  fn get_default_parameter() -> DefaultParameters {
    let mut components = T::child_schemas();
    if let Some(sch) = T::schema() {
      components.push(sch)
    }
    DefaultParameters {
      parameters: T::parameters(),
      components,
    }
  }
}

/// Defines default parameters with their associated components. Can be built from a type implementing `ApiComponent` using the `DefaultParameterAccessor` trait
#[derive(Default, Clone)]
pub struct DefaultParameters {
  pub parameters: Vec<Parameter>,
  pub components: Vec<(String, ReferenceOr<Schema>)>,
}

#[derive(Default, Clone)]
pub struct Spec {
  pub info: Info,
  pub default_tags: Vec<String>,
  /// See more details at <https://spec.openapis.org/oas/latest.html#tagObject>.
  pub tags: Vec<Tag>,
  /// See more details at <https://spec.openapis.org/oas/latest.html#external-documentation-object>.
  pub external_docs: Option<ExternalDocumentation>,
  /// See more details at <https://spec.openapis.org/oas/latest.html#server-object>.
  pub servers: Vec<Server>,
  /// Default parameters to be added to each operation. This only serves for documentation purpose.
  pub default_parameters: Vec<DefaultParameters>,
}
