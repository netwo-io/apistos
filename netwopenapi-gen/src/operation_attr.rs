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
  skip: bool,
  #[darling(default)]
  deprecated: bool,
  operation_id: Option<Expr>,
  summary: Option<String>,
  description: Option<String>,
  #[darling(multiple, rename = "tag")]
  tags: Vec<String>,
  #[darling(multiple, rename = "security_scope")]
  scopes: Vec<SecurityScopes>,
  #[darling(multiple, rename = "error_code")]
  error_codes: Vec<u16>,
}

#[derive(FromMeta, Clone)]
struct SecurityScopes {
  name: String,
  #[darling(multiple, rename = "scope")]
  scopes: Vec<String>,
}

pub(crate) struct OperationAttr {
  pub(crate) skip: bool,
  pub(crate) deprecated: bool,
  pub(crate) operation_id: Option<Expr>,
  pub(crate) summary: Option<String>,
  pub(crate) description: Option<String>,
  pub(crate) tags: Vec<String>,
  pub(crate) scopes: BTreeMap<String, Vec<String>>,
  pub(crate) error_codes: Vec<u16>,
}

impl From<OperationAttrInternal> for OperationAttr {
  fn from(value: OperationAttrInternal) -> Self {
    Self {
      skip: value.skip,
      deprecated: value.deprecated,
      operation_id: value.operation_id,
      summary: value.summary,
      description: value.description.map(|d| d.replace('\n', "\\\n")),
      tags: value.tags,
      scopes: value
        .scopes
        .into_iter()
        .map(|s| (s.name, s.scopes))
        .collect::<BTreeMap<_, _>>(),
      error_codes: value.error_codes,
    }
  }
}
