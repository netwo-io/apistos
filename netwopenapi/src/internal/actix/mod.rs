use utoipa::openapi::PathItemType;

pub mod handler;
pub mod resource;
pub mod route;
pub mod scope;
pub mod service_config;

mod utils;

const METHODS: &[PathItemType] = &[
  PathItemType::Get,
  PathItemType::Put,
  PathItemType::Post,
  PathItemType::Delete,
  PathItemType::Options,
  PathItemType::Head,
  PathItemType::Patch,
];
