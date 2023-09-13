use crate::api::error::ErrorResponse;
use crate::api::models::{OrganizationSlug, Pet, QueryStatus, QueryTag, Realm};
use crate::api::security::ApiKey;
use actix_web::web::{Header, Json, Path, Query};
use actix_web::Error;
use netwopenapi::actix::{CreatedJson, NoContent};
use netwopenapi::api_operation;
use serde_qs::actix::QsQuery;
use std::collections::HashMap;
use uuid::Uuid;

#[api_operation(tag = "pet")]
pub(crate) async fn update_pet(
  // Create a new pet in the store
  body: Json<Pet>,
  _key: ApiKey,
) -> Result<Json<Pet>, ErrorResponse> {
  Ok(body)
}

#[api_operation(
  tag = "pet",
  summary = "Add a new pet to the store",
  description = r###"Add a new pet to the store
  Plop"###,
  error_code = 405
)]
pub(crate) async fn add_pet(
  // Create a new pet in the store
  body: Json<Pet>,
  _key: ApiKey,
) -> Result<CreatedJson<Pet>, ErrorResponse> {
  Ok(CreatedJson(body.0))
}

/// Find pet by ID
/// Returns a single pet
#[api_operation(tag = "pet", security_scope(name = "api_key", scope = "read:pets"))]
pub(crate) async fn get_pet(
  // Create a new pet in the store
  _pet_id: Path<Uuid>,
  _key: Option<ApiKey>,
) -> Result<Option<Json<Pet>>, Error> // default undocumented error
{
  Ok(None)
}

/// Delete pet by ID
#[api_operation(tag = "pet", security_scope(name = "api_key", scope = "read:pets"))]
pub(crate) async fn delete_pet(
  // Create a new pet in the store
  _pet_id: Path<Uuid>,
  _key: Option<ApiKey>,
) -> Result<NoContent, Error> // default undocumented error
{
  Ok(NoContent)
}

/// Find pet by ID
/// Returns a single pet
#[api_operation(tag = "pet", security_scope(name = "api_key", scope = "read:pets"))]
pub(crate) async fn find_by_status(
  // Create a new pet in the store
  _status: Query<QueryStatus>,
  _key: ApiKey,
) -> Result<Option<Json<Pet>>, ErrorResponse> {
  todo!()
}

/// Find pet by ID
/// Returns a single pet
#[deprecated]
#[api_operation(tag = "pet", security_scope(name = "api_key", scope = "read:pets"))]
pub(crate) async fn find_by_tags(
  // Create a new pet in the store
  _tags: QsQuery<QueryTag>,
  _key: ApiKey,
) -> Result<Option<Json<Pet>>, ErrorResponse> {
  todo!()
}

/// Updates a pet in the store with form data
#[api_operation(
  tag = "pet",
  security_scope(name = "api_key", scope = "write:pets", scope = "read:pets")
)]
pub(crate) async fn update_pet_with_form(
  // ID of pet that needs to be updated
  _pet_id: Path<Uuid>,
  // query: Query<PetUpdatesQuery>,
  _query: Query<HashMap<String, String>>,
  _slug: Header<OrganizationSlug>,
  _realm: Realm,
  _key: ApiKey,
) -> Result<Option<Json<Pet>>, ErrorResponse> {
  todo!()
}
