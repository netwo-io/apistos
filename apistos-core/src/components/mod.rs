pub(crate) mod custom;

#[cfg(feature = "ipnetwork")]
pub use custom::ipnetwork;

#[cfg(feature = "actix")]
pub mod empty;
#[cfg(feature = "actix")]
pub mod form;
#[cfg(feature = "actix")]
pub mod json;
#[cfg(feature = "multipart")]
pub mod multipart;
#[cfg(feature = "actix")]
pub mod parameters;
pub mod simple;
