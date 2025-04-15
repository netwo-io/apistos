pub(crate) mod custom;

#[cfg(feature = "ipnetwork")]
pub use custom::ipnetwork;
#[cfg(feature = "ipnetwork_0_20")]
pub use custom::ipnetwork_20;

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
#[cfg(feature = "actix-session")]
pub mod session;
pub mod simple;
