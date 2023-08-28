use crate::api::models::{Pet, QueryStatus, QueryTag, Status, Tag};
use crate::api::security::ApiKey;
use actix_web::web::{Json, Path, Query};
use actix_web::Error;
use netwopenapi::actix::CreatedJson;
use netwopenapi::api_operation;
use netwopenapi::ApiComponent;
use uuid::Uuid;

#[api_operation]
pub(crate) async fn update_pet(
  // Create a new pet in the store
  body: Json<Pet>,
  key: ApiKey,
) -> Result<Json<Pet>, Error> {
  Ok(body)
}

#[api_operation(
  summary = "Add a new pet to the store",
  description = r###"Add a new pet to the store
  Plop"###
)]
pub(crate) async fn add_pet(
  // Create a new pet in the store
  body: Json<Pet>,
  key: ApiKey,
) -> Result<CreatedJson<Pet>, Error> {
  Ok(CreatedJson(body.0))
}

/// Find pet by ID
/// Returns a single pet
#[api_operation(tags = ["pet", "test"], scopes = [("api_key" = ["pet:read"])])]
pub(crate) async fn get_pet(
  // Create a new pet in the store
  pet_id: Path<Uuid>,
  key: Option<ApiKey>,
) -> Result<Option<Json<Pet>>, Error> {
  Ok(None)
}

/// Find pet by ID
/// Returns a single pet
#[api_operation(tags = ["pet", "test"], scopes = [("api_key" = ["pet:read"])])]
pub(crate) async fn find_by_status(
  // Create a new pet in the store
  status: Query<QueryStatus>,
  key: Option<ApiKey>,
) -> Result<Option<Json<Pet>>, Error> {
  todo!()
}

/// Find pet by ID
/// Returns a single pet
#[deprecated]
#[api_operation(tags = ["pet", "test"], scopes = [("api_key" = ["pet:read"])])]
pub(crate) async fn find_by_tags(
  // Create a new pet in the store
  tags: Query<QueryTag>, //@todo add serde_qs
  key: Option<ApiKey>,
) -> Result<Option<Json<Pet>>, Error> {
  todo!()
}
