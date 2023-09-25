use netwopenapi_models::InstanceType;

pub trait TypedSchema {
  fn schema_type() -> InstanceType;
  fn format() -> Option<String>;
}
