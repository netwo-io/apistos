use crate::api::error::ErrorResponse;
use crate::api::models::{OrganizationSlug, Pet, PetUpdatesQuery, QueryStatus, QueryTag, Realm, Status, Tag};
use crate::api::security::ApiKey;
use actix_web::web::{Header, Json, Path, Query};
use actix_web::Error;
use netwopenapi::actix::CreatedJson;
use netwopenapi::api_operation;
use netwopenapi::ApiComponent;
use std::collections::HashMap;
use uuid::Uuid;

#[api_operation(tags = ["pet"])]
pub(crate) async fn update_pet(
  // Create a new pet in the store
  body: Json<Pet>,
  key: ApiKey,
) -> Result<Json<Pet>, ErrorResponse> {
  Ok(body)
}

//@todo use multipart form instead
#[api_operation(
  tags = ["pet"],
  summary = "Add a new pet to the store",
  description = r###"Add a new pet to the store
  Plop"###,
  error_codes = [405]
)]
pub(crate) async fn add_pet(
  // Create a new pet in the store
  body: Json<Pet>,
  key: ApiKey,
) -> Result<CreatedJson<Pet>, ErrorResponse> {
  Ok(CreatedJson(body.0))
}

/// Find pet by ID
/// Returns a single pet
#[api_operation(tags = ["pet"], scopes = [("api_key" = ["read:pets"])])]
pub(crate) async fn get_pet(
  // Create a new pet in the store
  pet_id: Path<Uuid>,
  key: Option<ApiKey>,
) -> Result<Option<Json<Pet>>, Error> // default undocumented error
{
  Ok(None)
}

/// Find pet by ID
/// Returns a single pet
#[api_operation(tags = ["pet"], scopes = [("api_key" = ["read:pets"])])]
pub(crate) async fn find_by_status(
  // Create a new pet in the store
  status: Query<QueryStatus>,
  key: ApiKey,
) -> Result<Option<Json<Pet>>, ErrorResponse> {
  todo!()
}

/// Find pet by ID
/// Returns a single pet
#[deprecated]
#[api_operation(tags = ["pet"], scopes = [("api_key" = ["read:pets"])])]
pub(crate) async fn find_by_tags(
  // Create a new pet in the store
  tags: Query<QueryTag>, //@todo add serde_qs
  key: ApiKey,
) -> Result<Option<Json<Pet>>, ErrorResponse> {
  todo!()
}

/// Updates a pet in the store with form data
#[api_operation(tags = ["pet"], scopes = [("api_key" = ["write:pets", "read:pets"])])]
pub(crate) async fn update_pet_with_form(
  // ID of pet that needs to be updated
  pet_id: Path<Uuid>,
  // query: Query<PetUpdatesQuery>,
  query: Query<HashMap<String, String>>,
  slug: Header<OrganizationSlug>,
  realm: Realm,
  key: ApiKey,
) -> Result<Option<Json<Pet>>, ErrorResponse> {
  todo!()
}
