use apistos::ApistosSchema;
use apistos::reference_or::ReferenceOr;

pub(crate) fn assert_schema(v: &ReferenceOr<ApistosSchema>) {
  assert!(matches!(v, &ReferenceOr::Object { .. }), "Not a schema");
}

use actix_multipart as _;
