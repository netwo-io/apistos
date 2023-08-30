use crate::internal::components::ApiComponent;
use chrono::TimeZone;
use utoipa::openapi::{RefOr, Schema};

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
// #[cfg(feature = "url")]
// impl_simple!(url::Url);

#[cfg(any(feature = "chrono", feature = "extras"))]
simple_modifier!(chrono::NaiveDate);
#[cfg(any(feature = "chrono", feature = "extras"))]
simple_modifier!(chrono::NaiveTime);
#[cfg(any(feature = "chrono", feature = "extras"))]
simple_modifier!(chrono::NaiveDateTime);
#[cfg(any(feature = "chrono", feature = "extras"))]
simple_modifier!(chrono::Duration);
#[cfg(any(feature = "rust_decimal", feature = "extras"))]
simple_modifier!(rust_decimal::Decimal);
#[cfg(any(feature = "uuid", feature = "extras"))]
simple_modifier!(uuid::Uuid);

#[cfg(any(feature = "chrono", feature = "extras"))]
impl<T: TimeZone> ApiComponent for chrono::DateTime<T> {
  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    vec![]
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    let schema: RefOr<Schema> = utoipa::schema!(
      #[inline]
      chrono::DateTime
    )
    .into();
    Some(schema)
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    None
  }
}

#[cfg(any(feature = "chrono", feature = "extras"))]
impl<T: TimeZone> ApiComponent for chrono::Date<T> {
  fn child_schemas() -> Vec<(String, RefOr<Schema>)> {
    vec![]
  }

  fn raw_schema() -> Option<RefOr<Schema>> {
    let schema: RefOr<Schema> = utoipa::schema!(
      #[inline]
      chrono::Date
    )
    .into();
    Some(schema)
  }

  fn schema() -> Option<(String, RefOr<Schema>)> {
    None
  }
}
