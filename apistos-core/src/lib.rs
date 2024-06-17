mod api_component;
mod components;
mod error_component;
mod path_item_definition;
#[cfg(feature = "actix")]
mod wrappers;

pub mod __internal;

pub use api_component::ApiComponent;
pub use components::*;
pub use error_component::ApiErrorComponent;
pub use path_item_definition::PathItemDefinition;
#[cfg(feature = "actix")]
pub use wrappers::{ResponderWrapper, ResponseWrapper};

pub trait TypedSchema {
  fn schema_type() -> String;
  fn format() -> Option<String>;
}
