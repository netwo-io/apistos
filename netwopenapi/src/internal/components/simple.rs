use crate::internal::components::ApiComponent;

#[cfg(any(feature = "uuid", feature = "extras"))]
use uuid::Uuid;

macro_rules! simple_modifier {
  ($ty:ty) => {
    impl ApiComponent for $ty {
      fn child_schemas() -> Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)> {
        vec![]
      }
      fn raw_schema() -> Option<utoipa::openapi::RefOr<utoipa::openapi::Schema>> {
        let schema: utoipa::openapi::RefOr<utoipa::openapi::Schema> = utoipa::schema!(
          #[inline]
          $ty
        )
        .into();
        Some(schema)
      }
      fn schema() -> Option<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)> {
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
// #[cfg(feature = "chrono")]
// impl_simple!(chrono::NaiveDateTime);
// #[cfg(feature = "url")]
// impl_simple!(url::Url);

#[cfg(any(feature = "rust_decimal", feature = "extras"))]
simple_modifier!(rust_decimal::Decimal);
#[cfg(any(feature = "uuid", feature = "extras"))]
simple_modifier!(Uuid);
