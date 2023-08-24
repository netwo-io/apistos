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

      match attribute_name {
        "skip" => {
          operation_attr.skip = true;
        }
        "deprecated" => {
          operation_attr.deprecated = true;
        }
        "operation_id" => {
          match input.parse::<Token![=]>() {
            Ok(_) => (),
            Err(e) => abort!(e.span(), "Missing = before value assignment"),
          };
          let operation_id = Expr::parse(input)?;
          operation_attr.operation_id = Some(operation_id);
        }
        "description" => {
          match input.parse::<Token![=]>() {
            Ok(_) => (),
            Err(e) => abort!(e.span(), "Missing = before value assignment"),
          };
          let description = Expr::parse(input)?.to_token_stream().to_string();
          let description = description
            .trim_end_matches(|c| c == '"')
            .trim_start_matches(|c| c == '"');
          operation_attr.description = Some(description.to_owned());
        }
        "summary" => {
          match input.parse::<Token![=]>() {
            Ok(_) => (),
            Err(e) => abort!(e.span(), "Missing = before value assignment"),
          };
          let summary = Expr::parse(input)?.to_token_stream().to_string();
          let summary = summary.trim_end_matches(|c| c == '"').trim_start_matches(|c| c == '"');
          operation_attr.summary = Some(summary.to_owned());
        }
        "tags" => {
          match input.parse::<Token![=]>() {
            Ok(_) => (),
            Err(e) => abort!(e.span(), "Missing = before value assignment"),
          };
          let tags = Expr::parse(input)?;
          match tags {
            Expr::Array(arr) => {
              operation_attr.tags = arr
                .elems
                .iter()
                .map(|expr| {
                  let tag = expr.to_token_stream().to_string();
                  tag
                    .trim_end_matches(|c| c == '"')
                    .trim_start_matches(|c| c == '"')
                    .to_string()
                })
                .collect()
            }
            _ => {
              return Err(syn::Error::new(
                ident.span(),
                "unexpected value, expect an array for tags",
              ));
            }
          }
        }
        _ => {
          //@todo fix message
          return Err(syn::Error::new(ident.span(), "unexpected identifier, expected any of: operation_id, path, get, post, put, delete, options, head, patch, trace, connect, request_body, responses, params, tag, security, context_path"));
        }
      }

      if !input.is_empty() {
        input.parse::<Token![,]>()?;
      }
    }

    Ok(operation_attr)
  }
}
