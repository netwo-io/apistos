use netwopenapi::reference_or::ReferenceOr;
use schemars::schema::Schema;

mod api_component_derive;
mod api_cookie_derive;
mod api_error_derive;
mod api_header_derive;
mod api_operation;
mod api_security_derive;
mod api_type_derive;

pub(crate) fn assert_schema(v: ReferenceOr<Schema>) {
  if !matches!(v, ReferenceOr::Object(_)) {
    panic!("Not a schema")
  }
}

pub(crate) fn assert_ref(v: ReferenceOr<Schema>) {
  if !matches!(v, ReferenceOr::Reference { .. }) {
    panic!("Not a reference")
  }
}
