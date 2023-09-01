use darling::ast::NestedMeta;
use darling::FromMeta;
use proc_macro_error::abort;
use std::collections::BTreeMap;
use syn::Expr;

pub(crate) fn parse_openapi_operation_attrs(attrs: &[NestedMeta]) -> OperationAttr {
  match OperationAttrInternal::from_list(attrs) {
    Ok(operation) => operation.into(),
    Err(e) => abort!(e.span(), "Unable to parse #[api_operation] attribute: {:?}", e),
  }
}

#[derive(FromMeta, Clone)]
struct OperationAttrInternal {
  #[darling(default)]
  pub skip: bool,
  #[darling(default)]
  pub deprecated: bool,
  pub operation_id: Option<Expr>,
  pub summary: Option<String>,
  pub description: Option<String>,
  #[darling(multiple, rename = "tag")]
  pub tags: Vec<String>,
  #[darling(multiple, rename = "security_scope")]
  pub scopes: Vec<SecurityScopes>,
  #[darling(multiple, rename = "error_code")]
  pub error_codes: Vec<u16>,
}

#[derive(FromMeta, Clone)]
struct SecurityScopes {
  name: String,
  #[darling(multiple, rename = "scope")]
  scopes: Vec<String>,
}

pub(crate) struct OperationAttr {
  pub skip: bool,
  pub deprecated: bool,
  pub operation_id: Option<Expr>,
  pub summary: Option<String>,
  pub description: Option<String>,
  pub tags: Vec<String>,
  pub scopes: BTreeMap<String, Vec<String>>,
  pub error_codes: Vec<u16>,
}

impl From<OperationAttrInternal> for OperationAttr {
  fn from(value: OperationAttrInternal) -> Self {
    Self {
      skip: value.skip,
      deprecated: value.deprecated,
      operation_id: value.operation_id,
      summary: value.summary,
      description: value.description.map(|d| d.replace("\n", "\\\n")),
      tags: value.tags,
      scopes: BTreeMap::from_iter(value.scopes.into_iter().map(|s| (s.name, s.scopes)).collect::<Vec<_>>()),
      error_codes: value.error_codes,
    }
  }
}
