use crate::ApiComponent;

macro_rules! simple_modifier {
  ($ty:ty) => {
    impl ApiComponent for $ty {
      fn child_schemas() -> Vec<(
        String,
        netwopenapi_models::reference_or::ReferenceOr<netwopenapi_models::Schema>,
      )> {
        vec![]
      }
      fn raw_schema() -> Option<netwopenapi_models::reference_or::ReferenceOr<netwopenapi_models::Schema>> {
        let gen = schemars::gen::SchemaSettings::openapi3().into_generator();

        let schema: netwopenapi_models::reference_or::ReferenceOr<netwopenapi_models::Schema> =
          netwopenapi_models::Schema::Object(gen.into_root_schema_for::<$ty>().schema).into();
        Some(schema)
      }
      fn schema() -> Option<(
        String,
        netwopenapi_models::reference_or::ReferenceOr<netwopenapi_models::Schema>,
      )> {
        None
      }
    }
  };
}

simple_modifier!(char);
simple_modifier!(String);
simple_modifier!(bool);
simple_modifier!(f32);
simple_modifier!(f64);
simple_modifier!(i8);
simple_modifier!(i16);
simple_modifier!(i32);
simple_modifier!(u8);
simple_modifier!(u16);
simple_modifier!(u32);
simple_modifier!(i64);
simple_modifier!(i128);
simple_modifier!(isize);
simple_modifier!(u64);
simple_modifier!(u128);
simple_modifier!(usize);

#[cfg(feature = "chrono")]
simple_modifier!(chrono::NaiveDate);
#[cfg(feature = "chrono")]
simple_modifier!(chrono::NaiveTime);
#[cfg(feature = "chrono")]
simple_modifier!(chrono::NaiveDateTime);
#[cfg(feature = "rust_decimal")]
simple_modifier!(rust_decimal::Decimal);
#[cfg(feature = "uuid")]
simple_modifier!(uuid::Uuid);
#[cfg(feature = "url")]
simple_modifier!(url::Url);

#[cfg(feature = "chrono")]
impl<T: chrono::TimeZone> ApiComponent for chrono::DateTime<T> {
  fn child_schemas() -> Vec<(
    String,
    netwopenapi_models::reference_or::ReferenceOr<netwopenapi_models::Schema>,
  )> {
    vec![]
  }

  fn raw_schema() -> Option<netwopenapi_models::reference_or::ReferenceOr<netwopenapi_models::Schema>> {
    let gen = schemars::gen::SchemaSettings::openapi3().into_generator();

    let schema: netwopenapi_models::reference_or::ReferenceOr<netwopenapi_models::Schema> =
      netwopenapi_models::Schema::Object(gen.into_root_schema_for::<chrono::DateTime<T>>().schema).into();
    Some(schema)
  }

  fn schema() -> Option<(
    String,
    netwopenapi_models::reference_or::ReferenceOr<netwopenapi_models::Schema>,
  )> {
    None
  }
}
