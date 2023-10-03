mod internal;

pub mod actix;

pub mod app;
pub mod spec;
pub mod web;

pub use netwopenapi_core::parameters::header::ApiHeader;
pub use netwopenapi_core::PathItemDefinition;
pub use netwopenapi_core::{ApiComponent, ApiErrorComponent, TypedSchema};
pub use netwopenapi_gen::{api_operation, ApiComponent, ApiCookie, ApiErrorComponent, ApiHeader, ApiSecurity, ApiType};

pub use indexmap::IndexMap;
pub use netwopenapi_models::*;

// Imports bellow aim at making cargo-cranky happy. Those dependencies are necessary for integration-test.
use schemars as _;
