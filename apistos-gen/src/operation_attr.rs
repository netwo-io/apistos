use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro_error::abort;
use proc_macro2::Ident;
use std::collections::BTreeMap;

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
  operation_id: Option<String>,
  summary: Option<String>,
  description: Option<String>,
  #[darling(multiple, rename = "tag")]
  tags: Vec<String>,
  #[darling(multiple, rename = "security_scope")]
  scopes: Vec<SecurityScopes>,
  #[darling(multiple, rename = "error_code")]
  error_codes: Vec<u16>,
  consumes: Option<String>,
  produces: Option<String>,
  #[darling(multiple)]
  skip_args: Vec<Ident>,
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
  pub(crate) operation_id: Option<String>,
  pub(crate) summary: Option<String>,
  pub(crate) description: Option<String>,
  pub(crate) tags: Vec<String>,
  pub(crate) scopes: BTreeMap<String, Vec<String>>,
  pub(crate) error_codes: Vec<u16>,
  pub(crate) consumes: Option<String>,
  pub(crate) produces: Option<String>,
  pub(crate) skip_args: Vec<Ident>,
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
      consumes: value.consumes,
      produces: value.produces,
      skip_args: value.skip_args,
    }
  }
}
