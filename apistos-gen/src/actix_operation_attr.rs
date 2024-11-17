use crate::operation_attr::{OperationAttr, OperationAttrInternal};
use darling::ast::NestedMeta;
use darling::FromMeta;
use proc_macro_error2::abort;

pub(crate) fn parse_actix_openapi_operation_attrs(attrs: &[NestedMeta], macro_identifier: &str) -> ActixOperationAttr {
  match ActixOperationAttrInternal::from_list(attrs) {
    Ok(operation) => operation.into(),
    Err(e) => abort!(e.span(), "Unable to parse #[{macro_identifier}] attribute: {:?}", e),
  }
}

#[derive(FromMeta, Clone)]
pub(crate) struct ActixOperationAttrInternal {
  path: String,
  name: Option<String>,
  guard: Option<String>, // @todo vec ?
  wrap: Option<String>,  // @todo vec ?
  #[darling(flatten)]
  operation: OperationAttrInternal,
}

impl From<ActixOperationAttrInternal> for ActixOperationAttr {
  fn from(value: ActixOperationAttrInternal) -> Self {
    Self {
      path: value.path,
      name: value.name,
      guard: value.guard,
      wrap: value.wrap,
      operation: value.operation.into(),
    }
  }
}

pub(crate) struct ActixOperationAttr {
  pub(crate) path: String,
  pub(crate) name: Option<String>,
  pub(crate) guard: Option<String>,
  pub(crate) wrap: Option<String>,
  pub(crate) operation: OperationAttr,
}
