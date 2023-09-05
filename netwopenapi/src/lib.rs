mod internal;

pub mod actix;

pub mod app;
pub mod path_item_definition;
pub mod spec;
pub mod web;

pub use internal::components::error::ApiErrorComponent;
pub use internal::components::parameters::header::ApiHeader;
pub use internal::components::{ApiComponent, TypedSchema};
pub use netwopenapi_gen::{api_operation, ApiComponent, ApiCookie, ApiErrorComponent, ApiHeader, ApiSecurity, ApiType};
