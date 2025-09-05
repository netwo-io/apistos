use crate::ApiComponent;

macro_rules! simple_modifier {
  ($ty:ty) => {
    impl ApiComponent for $ty {
      fn child_schemas() -> Vec<(
        String,
        apistos_models::reference_or::ReferenceOr<apistos_models::Schema>,
      )> {
        vec![]
      }
      fn raw_schema() -> Option<apistos_models::reference_or::ReferenceOr<apistos_models::Schema>> {
        let generator = schemars::r#gen::SchemaSettings::openapi3().into_generator();

        let schema: apistos_models::reference_or::ReferenceOr<apistos_models::Schema> =
          apistos_models::Schema::Object(generator.into_root_schema_for::<$ty>().schema).into();
        Some(schema)
      }
      fn schema() -> Option<(
        String,
        apistos_models::reference_or::ReferenceOr<apistos_models::Schema>,
      )> {
        None
      }
    }
  };
}

simple_modifier!(char);
simple_modifier!(&'static str);
simple_modifier!(&'static [u8]);
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
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U8);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U16);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U24);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U32);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U40);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U48);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U56);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U64);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U72);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U80);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U88);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U96);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U104);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U112);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U120);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U128);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U136);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U144);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U152);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U160);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U168);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U176);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U184);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U192);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U200);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U208);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U216);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U224);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U232);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U240);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U248);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::U256);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I8);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I16);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I24);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I32);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I40);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I48);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I56);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I64);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I72);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I80);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I88);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I96);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I104);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I112);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I120);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I128);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I136);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I144);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I152);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I160);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I168);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I176);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I184);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I192);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I200);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I208);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I216);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I224);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I232);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I240);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I248);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::I256);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::B8);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::B16);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::B32);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::B64);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::B96);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::B128);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::B160);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::B192);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::B224);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::aliases::B256);
#[cfg(feature = "alloy_primitives")]
simple_modifier!(alloy_primitives::Address);

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
    apistos_models::reference_or::ReferenceOr<apistos_models::Schema>,
  )> {
    vec![]
  }

  fn raw_schema() -> Option<apistos_models::reference_or::ReferenceOr<apistos_models::Schema>> {
    let generator = schemars::r#gen::SchemaSettings::openapi3().into_generator();

    let schema: apistos_models::reference_or::ReferenceOr<apistos_models::Schema> =
      apistos_models::Schema::Object(generator.into_root_schema_for::<chrono::DateTime<T>>().schema).into();
    Some(schema)
  }

  fn schema() -> Option<(
    String,
    apistos_models::reference_or::ReferenceOr<apistos_models::Schema>,
  )> {
    None
  }
}
