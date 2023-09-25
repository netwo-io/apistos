use netwopenapi_core::ApiComponent;
use netwopenapi_gen::ApiComponent;
use schemars::JsonSchema;

#[test]
fn api_component_derive() {
  #[derive(JsonSchema, ApiComponent)]
  struct Name {
    name: String,
  }

  let name_schema = <Name as ApiComponent>::schema();
}
