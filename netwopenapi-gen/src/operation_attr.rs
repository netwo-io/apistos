use proc_macro2::Ident;
use proc_macro_error::abort;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token};

#[derive(Default)]
pub struct OperationAttr {
  pub skip: bool,
  pub deprecated: bool,
  pub operation_id: Option<Expr>,
  pub summary: Option<String>,
  pub description: Option<String>,
  pub tags: Vec<String>,
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
      } else if attribute_name == "description" {
        match input.parse::<Token![=]>() {
          Ok(_) => (),
          Err(e) => abort!(e.span(), "Missing = before value assignment"),
        };
        let description = Expr::parse(input)?.to_token_stream().to_string();
        let description = description
          .trim_end_matches(|c| c == '"')
          .trim_start_matches(|c| c == '"');
        operation_attr.description = Some(description.to_owned());
      } else if attribute_name == "summary" {
        match input.parse::<Token![=]>() {
          Ok(_) => (),
          Err(e) => abort!(e.span(), "Missing = before value assignment"),
        };
        let summary = Expr::parse(input)?.to_token_stream().to_string();
        let summary = summary.trim_end_matches(|c| c == '"').trim_start_matches(|c| c == '"');
        operation_attr.summary = Some(summary.to_owned());
      } else if attribute_name == "tags" {
        match input.parse::<Token![=]>() {
          Ok(_) => (),
          Err(e) => abort!(e.span(), "Missing = before value assignment"),
        };
        let summary = Expr::parse(input)?.to_token_stream().to_string();
        let summary = summary.trim_end_matches(|c| c == '"').trim_start_matches(|c| c == '"');
        operation_attr.summary = Some(summary.to_owned());
      }

      if !input.is_empty() {
        input.parse::<Token![,]>()?;
      }
    }

    Ok(operation_attr)
  }
}
