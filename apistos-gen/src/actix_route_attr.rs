use crate::operation_attr::{OperationAttr, OperationAttrInternal};
use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro_error2::abort;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub(crate) fn parse_actix_route_attrs(attrs: &[NestedMeta]) -> ActixRouteAttr {
  match ActixRouteAttrInternal::from_list(attrs).and_then(|op| op.try_into().map_err(Into::into)) {
    Ok(operation) => operation,
    Err(e) => abort!(e.span(), "Unable to parse #[route] attribute: {:?}", e),
  }
}

#[derive(FromMeta, Clone)]
pub(crate) struct ActixRouteAttrInternal {
  pub(crate) path: String,
  #[darling(multiple, rename = "method")]
  pub(crate) methods: Vec<ActixRouteMethodAttrInternal>,
  pub(crate) name: Option<String>,
  pub(crate) guard: Option<String>, // @todo vec ?
  pub(crate) wrap: Option<String>,  // @todo vec ?
}

#[derive(FromMeta, Clone)]
pub(crate) struct ActixRouteMethodAttrInternal {
  method: String,
  #[darling(flatten)]
  operation: OperationAttrInternal,
}

impl ToTokens for ActixRouteMethodAttrInternal {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let ActixRouteMethodAttrInternal { method, operation } = self;
    tokens.extend(quote!(method(method = #method, #operation)))
  }
}

impl TryFrom<ActixRouteAttrInternal> for ActixRouteAttr {
  type Error = syn::Error;

  fn try_from(value: ActixRouteAttrInternal) -> Result<Self, Self::Error> {
    Ok(Self {
      path: value.path,
      methods: value
        .methods
        .into_iter()
        .map(|m| {
          m.operation.try_into().map(|operation_attr| ActixRouteMethodAttr {
            method: m.method,
            operation_attr,
          })
        })
        .collect::<Result<Vec<ActixRouteMethodAttr>, syn::Error>>()?,
      name: value.name,
      guard: value.guard,
      wrap: value.wrap,
    })
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
