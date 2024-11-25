use crate::operation_attr::{OperationAttr, OperationAttrInternal};
use darling::ast::NestedMeta;
use darling::FromMeta;
use proc_macro_error2::abort;

pub(crate) fn parse_actix_route_attrs(attrs: &[NestedMeta]) -> ActixRouteAttr {
  match ActixRouteAttrInternal::from_list(attrs) {
    Ok(operation) => operation.into(),
    Err(e) => abort!(e.span(), "Unable to parse #[route] attribute: {:?}", e),
  }
}

#[derive(FromMeta, Clone)]
struct ActixRouteAttrInternal {
  path: String,
  #[darling(multiple, rename = "method")]
  methods: Vec<ActixRouteMethodAttrInternal>,
  name: Option<String>,
  guard: Option<String>, // @todo vec ?
  wrap: Option<String>,  // @todo vec ?
}

#[derive(FromMeta, Clone)]
pub(crate) struct ActixRouteMethodAttrInternal {
  method: String,
  #[darling(flatten)]
  operation: OperationAttrInternal,
}

impl From<ActixRouteAttrInternal> for ActixRouteAttr {
  fn from(value: ActixRouteAttrInternal) -> Self {
    Self {
      path: value.path,
      methods: value
        .methods
        .into_iter()
        .map(|m| ActixRouteMethodAttr {
          method: m.method,
          operation_attr: m.operation.into(),
        })
        .collect(),
      name: value.name,
      guard: value.guard,
      wrap: value.wrap,
    }
  }
}

pub(crate) struct ActixRouteAttr {
  pub(crate) path: String,
  pub(crate) methods: Vec<ActixRouteMethodAttr>,
  pub(crate) name: Option<String>,
  pub(crate) guard: Option<String>, // @todo vec ?
  pub(crate) wrap: Option<String>,  // @todo vec ?
}

pub(crate) struct ActixRouteMethodAttr {
  pub(crate) method: String,
  pub(crate) operation_attr: OperationAttr,
}
