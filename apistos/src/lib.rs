mod internal;

pub mod actix;

pub mod app;
pub mod spec;
pub mod web;

pub use apistos_core::parameters::header::ApiHeader;
pub use apistos_core::PathItemDefinition;
pub use apistos_core::{ApiComponent, ApiErrorComponent, TypedSchema};
pub use apistos_gen::{api_operation, ApiComponent, ApiCookie, ApiErrorComponent, ApiHeader, ApiSecurity, ApiType};

pub use apistos_models::*;
pub use indexmap::IndexMap;

// Imports bellow aim at making clippy happy. Those dependencies are necessary for integration-test.
use schemars as _;
