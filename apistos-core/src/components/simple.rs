macro_rules! simple_modifier {
  ($ty:ty) => {
    impl crate::ApiComponent for $ty {
      fn child_schemas(
        _: apistos_models::OpenApiVersion,
      ) -> Vec<(
        String,
        apistos_models::reference_or::ReferenceOr<apistos_models::ApistosSchema>,
      )> {
        vec![]
      }
      fn raw_schema(
        oas_version: apistos_models::OpenApiVersion,
      ) -> Option<apistos_models::reference_or::ReferenceOr<apistos_models::ApistosSchema>> {
        let schema_settings = oas_version.get_schema_settings();
        let generator = schema_settings.into_generator();

        let schema: apistos_models::reference_or::ReferenceOr<apistos_models::ApistosSchema> =
          apistos_models::ApistosSchema::new(generator.into_root_schema_for::<$ty>(), oas_version).into();
        Some(schema)
      }
      fn schema(
        _: apistos_models::OpenApiVersion,
      ) -> Option<(
        String,
        apistos_models::reference_or::ReferenceOr<apistos_models::ApistosSchema>,
      )> {
        None
      }
    }
  };
}
pub(crate) use simple_modifier;

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
impl<T: chrono::TimeZone> crate::ApiComponent for chrono::DateTime<T> {
  fn child_schemas(
    _: apistos_models::OpenApiVersion,
  ) -> Vec<(
    String,
    apistos_models::reference_or::ReferenceOr<apistos_models::ApistosSchema>,
  )> {
    vec![]
  }

  fn raw_schema(
    oas_version: apistos_models::OpenApiVersion,
  ) -> Option<apistos_models::reference_or::ReferenceOr<apistos_models::ApistosSchema>> {
    let schema_settings = oas_version.get_schema_settings();
    let generator = schema_settings.into_generator();

    let schema: apistos_models::reference_or::ReferenceOr<apistos_models::ApistosSchema> =
      apistos_models::ApistosSchema::new(generator.into_root_schema_for::<chrono::DateTime<T>>(), oas_version).into();
    Some(schema)
  }

  fn schema(
    _: apistos_models::OpenApiVersion,
  ) -> Option<(
    String,
    apistos_models::reference_or::ReferenceOr<apistos_models::ApistosSchema>,
  )> {
    None
  }
}
