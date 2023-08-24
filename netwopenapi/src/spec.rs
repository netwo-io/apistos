use utoipa::openapi::Info;

#[derive(Default)]
pub struct Spec {
  pub info: Info,
  pub default_tags: Vec<String>,
}
