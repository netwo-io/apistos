use crate::internal::components::ApiComponent;

macro_rules! simple_modifier {
  ($ty:ty) => {
    impl ApiComponent for $ty {
      fn child_schemas() -> Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)> {
        vec![]
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
// #[cfg(feature = "rust_decimal")]
// impl_simple!(rust_decimal::Decimal);
// #[cfg(feature = "url")]
// impl_simple!(url::Url);
// #[cfg(feature = "uuid0")]
// impl_simple!(uuid0_dep::Uuid);
// #[cfg(feature = "uuid1")]
// impl_simple!(uuid1_dep::Uuid);
