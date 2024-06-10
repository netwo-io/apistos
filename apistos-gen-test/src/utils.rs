use apistos::reference_or::ReferenceOr;
use apistos::ApistosSchema;

pub(crate) fn assert_schema(v: &ReferenceOr<ApistosSchema>) {
  assert!(matches!(v, &ReferenceOr::Object { .. }), "Not a schema");
}
