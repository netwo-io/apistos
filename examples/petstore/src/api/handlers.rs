use crate::api::error::ErrorResponse;
use crate::api::models::{Category, Pet, Status, Tag};
use actix_web::web::Json;
use actix_web::Error;
use netwopenapi::actix::ResponseWrapper;
use netwopenapi::path_item_definition::PathItemDefinition;
use netwopenapi::ApiComponent;
use netwopenapi_gen::api_operation;
use std::collections::BTreeMap;
use std::sync::Arc;
use utoipa::openapi::path::Operation;
use utoipa::openapi::{Components, ComponentsBuilder, PathItem};

#[api_operation]
pub(crate) async fn update_pet(
  // Create a new pet in the store
  body: Json<Pet>,
) -> Result<Json<Pet>, Error> {
  Ok(body)
}

// summary Add a new pet to the store
// description Add a new pet to the store
// operationId addPet
pub(crate) async fn add_pet(
  // Create a new pet in the store
  body: Json<Pet>,
) -> Result<Json<Pet>, Error> {
  Ok(body)
}
