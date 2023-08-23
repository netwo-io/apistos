use proc_macro2::Ident;
use proc_macro_error::abort;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token};

#[derive(Default)]
pub(crate) struct OperationAttr {
  pub(crate) skip: bool,
  pub(crate) deprecated: bool,
  pub(crate) operation_id: Option<Expr>,
}

impl Parse for OperationAttr {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut operation_attr = OperationAttr::default();

    while !input.is_empty() {
      let ident = input
        .parse::<Ident>()
        .map_err(|e| syn::Error::new(e.span(), format!("{e}")))?;
      let attribute_name = &*ident.to_string();

      if attribute_name == "skip" {
        operation_attr.skip = true;
      } else if attribute_name == "deprecated" {
        operation_attr.deprecated = true;
      } else if attribute_name == "operation_id" {
        match input.parse::<Token![=]>() {
          Ok(_) => (),
          Err(e) => abort!(e.span(), "Missing = before value assignment"),
        };
        let operation_id = Expr::parse(input)?;
        operation_attr.operation_id = Some(operation_id);
      }

      if !input.is_empty() {
        input.parse::<Token![,]>()?;
      }
    }

    Ok(operation_attr)
  }
}
