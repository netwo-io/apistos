use netwopenapi_models::paths::OperationType;

pub mod handler;
pub mod resource;
pub mod route;
pub mod scope;
pub mod service_config;

mod utils;

const METHODS: &[OperationType] = &[
  OperationType::Get,
  OperationType::Put,
  OperationType::Post,
  OperationType::Delete,
  OperationType::Options,
  OperationType::Head,
  OperationType::Patch,
];
