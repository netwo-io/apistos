use crate::ApiComponent;

macro_rules! simple_modifier {
  ($ty:ty) => {
    impl ApiComponent for $ty {
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
        let schema_settings = match oas_version {
          apistos_models::OpenApiVersion::OAS3_0 => schemars::gen::SchemaSettings::openapi3(),
          apistos_models::OpenApiVersion::OAS3_1 => schemars::gen::SchemaSettings::draft2020_12(),
        };
        let gen = schema_settings.into_generator();

        let schema: apistos_models::reference_or::ReferenceOr<apistos_models::ApistosSchema> =
          apistos_models::ApistosSchema::new(gen.into_root_schema_for::<$ty>(), oas_version).into();
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
    let schema_settings = match oas_version {
      apistos_models::OpenApiVersion::OAS3_0 => schemars::gen::SchemaSettings::openapi3(),
      apistos_models::OpenApiVersion::OAS3_1 => schemars::gen::SchemaSettings::draft2020_12(),
    };
    let gen = schema_settings.into_generator();

    let schema: apistos_models::reference_or::ReferenceOr<apistos_models::ApistosSchema> =
      apistos_models::ApistosSchema::new(gen.into_root_schema_for::<chrono::DateTime<T>>(), oas_version).into();
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
