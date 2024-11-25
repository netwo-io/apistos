use crate::OPENAPI_CALLBACK_STRUCT_PREFIX;
use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro_error2::abort;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};
use std::collections::BTreeMap;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub(crate) fn parse_openapi_operation_attrs(attrs: &[NestedMeta]) -> OperationAttr {
  match OperationAttrInternal::from_list(attrs) {
    Ok(operation) => operation.into(),
    Err(e) => abort!(e.span(), "Unable to parse #[api_operation] attribute: {:?}", e),
  }
}

#[derive(FromMeta, Clone)]
pub(crate) struct OperationAttrInternal {
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
  #[darling(multiple)]
  callbacks: Vec<NamedOperationCallbackInternal>,
}

#[derive(FromMeta, Clone)]
struct NamedOperationCallbackInternal {
  name: String,
  #[darling(multiple)]
  callback: Vec<OperationCallbackInternal>,
}

#[derive(FromMeta, Clone)]
struct OperationCallbackInternal {
  path: String,
  get: Option<Ident>,
  put: Option<Ident>,
  post: Option<Ident>,
  delete: Option<Ident>,
  options: Option<Ident>,
  head: Option<Ident>,
  patch: Option<Ident>,
  trace: Option<Ident>,
}

#[derive(FromMeta, Clone)]
struct SecurityScopes {
  name: String,
  #[darling(multiple, rename = "scope")]
  scopes: Vec<String>,
}

pub(crate) struct OperationCallbacks {
  pub(crate) name: String,
  pub(crate) callbacks: BTreeMap<String, BTreeMap<OperationType, Ident>>,
}

impl From<NamedOperationCallbackInternal> for OperationCallbacks {
  fn from(value: NamedOperationCallbackInternal) -> Self {
    let as_callback_fn_name = |ident: Ident| -> Ident {
      let callback_fn_name = format!("{OPENAPI_CALLBACK_STRUCT_PREFIX}{}", ident);
      Ident::new(&callback_fn_name, Span::call_site())
    };

    let mut callbacks = BTreeMap::default();
    for callback in value.callback {
      let mut operations = vec![];
      callback
        .get
        .into_iter()
        .for_each(|ident| operations.push((OperationType::Get, as_callback_fn_name(ident))));
      callback
        .put
        .into_iter()
        .for_each(|ident| operations.push((OperationType::Put, as_callback_fn_name(ident))));
      callback
        .post
        .into_iter()
        .for_each(|ident| operations.push((OperationType::Post, as_callback_fn_name(ident))));
      callback
        .delete
        .into_iter()
        .for_each(|ident| operations.push((OperationType::Delete, as_callback_fn_name(ident))));
      callback
        .options
        .into_iter()
        .for_each(|ident| operations.push((OperationType::Options, as_callback_fn_name(ident))));
      callback
        .head
        .into_iter()
        .for_each(|ident| operations.push((OperationType::Head, as_callback_fn_name(ident))));
      callback
        .patch
        .into_iter()
        .for_each(|ident| operations.push((OperationType::Patch, as_callback_fn_name(ident))));
      callback
        .trace
        .into_iter()
        .for_each(|ident| operations.push((OperationType::Trace, as_callback_fn_name(ident))));

      callbacks.insert(callback.path, BTreeMap::from_iter(operations));
    }

    Self {
      name: value.name,
      callbacks,
    }
  }
}

pub(crate) struct OperationAttr {
  pub(crate) skip: bool,
  pub(crate) deprecated: bool,
  pub(crate) operation_id: Option<String>,
  pub(crate) summary: Option<String>,
  pub(crate) description: Option<String>,
  pub(crate) tags: Vec<String>,
  pub(crate) callbacks: Vec<OperationCallbacks>,
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
      callbacks: value.callbacks.into_iter().map(Into::into).collect(),
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

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub(crate) enum OperationType {
  Get,
  Put,
  Post,
  Delete,
  Options,
  Head,
  Patch,
  Trace,
  Custom(String),
}

impl ToTokens for OperationType {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    match self {
      OperationType::Get => tokens.extend(quote!(apistos::paths::OperationType::Get)),
      OperationType::Put => tokens.extend(quote!(apistos::paths::OperationType::Put)),
      OperationType::Post => tokens.extend(quote!(apistos::paths::OperationType::Post)),
      OperationType::Delete => tokens.extend(quote!(apistos::paths::OperationType::Delete)),
      OperationType::Options => tokens.extend(quote!(apistos::paths::OperationType::Options)),
      OperationType::Head => tokens.extend(quote!(apistos::paths::OperationType::Head)),
      OperationType::Patch => tokens.extend(quote!(apistos::paths::OperationType::Patch)),
      OperationType::Trace => tokens.extend(quote!(apistos::paths::OperationType::Trace)),
      OperationType::Custom(_) => {}
    }
  }
}

impl FromStr for OperationType {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "get" => Ok(OperationType::Get),
      "put" => Ok(OperationType::Patch),
      "post" => Ok(OperationType::Post),
      "delete" => Ok(OperationType::Delete),
      "options" => Ok(OperationType::Options),
      "head" => Ok(OperationType::Head),
      "patch" => Ok(OperationType::Patch),
      "trace" => Ok(OperationType::Trace),
      str => Ok(OperationType::Custom(str.to_string())),
    }
  }
}

impl Display for OperationType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      OperationType::Get => write!(f, "get"),
      OperationType::Put => write!(f, "put"),
      OperationType::Post => write!(f, "post"),
      OperationType::Delete => write!(f, "delete"),
      OperationType::Options => write!(f, "options"),
      OperationType::Head => write!(f, "head"),
      OperationType::Patch => write!(f, "patch"),
      OperationType::Trace => write!(f, "trace"),
      OperationType::Custom(op) => write!(f, "{op}"),
    }
  }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub(crate) enum ActixOperationTypePath {
  Get,
  Put,
  Post,
  Delete,
  Options,
  Head,
  Patch,
  Trace,
}

impl ToTokens for ActixOperationTypePath {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    match self {
      ActixOperationTypePath::Get => tokens.extend(quote!(get)),
      ActixOperationTypePath::Put => tokens.extend(quote!(put)),
      ActixOperationTypePath::Post => tokens.extend(quote!(post)),
      ActixOperationTypePath::Delete => tokens.extend(quote!(delete)),
      ActixOperationTypePath::Options => tokens.extend(quote!(options)),
      ActixOperationTypePath::Head => tokens.extend(quote!(head)),
      ActixOperationTypePath::Patch => tokens.extend(quote!(patch)),
      ActixOperationTypePath::Trace => tokens.extend(quote!(trace)),
    }
  }
}

impl TryFrom<OperationType> for ActixOperationTypePath {
  type Error = syn::Error;

  fn try_from(value: OperationType) -> Result<Self, Self::Error> {
    match value {
      OperationType::Get => Ok(Self::Get),
      OperationType::Put => Ok(Self::Put),
      OperationType::Post => Ok(Self::Post),
      OperationType::Delete => Ok(Self::Delete),
      OperationType::Options => Ok(Self::Options),
      OperationType::Head => Ok(Self::Head),
      OperationType::Patch => Ok(Self::Patch),
      OperationType::Trace => Ok(Self::Trace),
      OperationType::Custom(_) => Err(syn::Error::new(Span::call_site(), "Custum operation type found")),
    }
  }
}
