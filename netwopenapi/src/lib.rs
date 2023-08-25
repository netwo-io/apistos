mod internal;

pub mod actix;

pub mod app;
pub mod path_item_definition;
pub mod spec;
pub mod web;

pub use internal::components::ApiComponent;
pub use netwopenapi_gen::{api_operation, ApiComponent, ApiSecurity};
