use apistos::reference_or::ReferenceOr;
use schemars::Schema;

pub(crate) fn assert_schema(v: &ReferenceOr<Schema>) {
  assert!(matches!(v, &ReferenceOr::Object { .. }), "Not a schema");
}
