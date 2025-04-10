use apistos_models::paths::OperationType;

pub(crate) mod handler;
pub(crate) mod redirect;
pub(crate) mod resource;
pub(crate) mod route;
pub(crate) mod scope;
pub(crate) mod service_config;

mod utils;

pub(super) const METHODS: &[OperationType] = &[
  OperationType::Get,
  OperationType::Put,
  OperationType::Post,
  OperationType::Delete,
  OperationType::Options,
  OperationType::Head,
  OperationType::Patch,
];
