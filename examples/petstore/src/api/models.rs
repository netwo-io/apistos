use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub(crate) struct Pet {
  // #[openapi(example = 10)]
  pub(crate) id: Option<i64>,
  // #[openapi(example = "doggie")]
  pub(crate) name: String,
  pub(crate) category: Option<Category>,
  // #[openapi(rename = "photoUrls")]
  pub(crate) photo_urls: Vec<String>,
  pub(crate) tags: Option<Vec<Tag>>,
  /// pet status in the store
  pub(crate) status: Option<Status>,
}

/// A category for a pet
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub(crate) struct Category {
  // #[openapi(example = 1)]
  pub(crate) id: Option<i64>,
  // #[openapi(example = "Dogs")]
  pub(crate) name: Option<String>,
}

/// pet status in the store
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Status {
  Available,
  Pending,
  Sold,
}

/// A tag for a pet
// #[openapi(rename = "Pet tag")]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub(crate) struct Tag {
  pub(crate) id: Option<i64>,
  pub(crate) name: Option<String>,
}
