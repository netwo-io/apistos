use proc_macro2::{Group, Ident};
use proc_macro_error::abort;
use std::collections::BTreeMap;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{bracketed, Expr, LitInt, LitStr, Token};

#[derive(Default)]
pub struct OperationAttr {
  pub skip: bool,
  pub deprecated: bool,
  pub operation_id: Option<Expr>,
  pub summary: Option<String>,
  pub description: Option<String>,
  pub tags: Vec<String>,
  pub scopes: BTreeMap<String, Vec<String>>,
  pub error_codes: Vec<u16>,
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
          let description = input.parse::<LitStr>()?.value().replace('\n', "\\\n");
          operation_attr.description = Some(description);
        }
        "summary" => {
          match input.parse::<Token![=]>() {
            Ok(_) => (),
            Err(e) => abort!(e.span(), "Missing = before value assignment"),
          };
          let summary = input.parse::<LitStr>()?.value();
          operation_attr.summary = Some(summary);
        }
        "tags" => {
          match input.parse::<Token![=]>() {
            Ok(_) => (),
            Err(e) => abort!(e.span(), "Missing = before value assignment"),
          };
          let tags;
          bracketed!(tags in input);
          let tags = Punctuated::<LitStr, Comma>::parse_terminated(&tags)?
            .iter()
            .map(LitStr::value)
            .collect::<Vec<_>>();
          operation_attr.tags = tags;
        }
        "scopes" => {
          match input.parse::<Token![=]>() {
            Ok(_) => (),
            Err(e) => abort!(e.span(), "Missing = before value assignment"),
          };
          let scope_groups;
          bracketed!(scope_groups in input);
          let scope_groups: Vec<(String, Vec<String>)> = Punctuated::<Group, Comma>::parse_terminated(&scope_groups)?
            .into_iter()
            .map(|group| {
              let input = group.stream();
              if input.is_empty() {
                return Ok(None);
              }
              syn::parse2::<SecurityScopes>(input)
                .map(|SecurityScopes { name, scopes }| (name.clone(), scopes.clone()))
                .map(Some)
            })
            .collect::<syn::Result<Vec<Option<_>>>>()?
            .into_iter()
            .flatten()
            .collect();
          let scopes = BTreeMap::from_iter(scope_groups);
          operation_attr.scopes = scopes;
        }
        "error_codes" => {
          match input.parse::<Token![=]>() {
            Ok(_) => (),
            Err(e) => abort!(e.span(), "Missing = before value assignment"),
          };
          let error_codes;
          bracketed!(error_codes in input);
          let error_codes = Punctuated::<LitInt, Comma>::parse_terminated(&error_codes)?
            .iter()
            .map(LitInt::base10_parse)
            .collect::<syn::Result<Vec<u16>>>();
          let error_codes = match error_codes {
            Ok(error_codes) => error_codes,
            Err(e) => abort!(e.span(), "Expected u16 status code"),
          };
          operation_attr.error_codes = error_codes;
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

pub struct SecurityScopes {
  name: String,
  scopes: Vec<String>,
}

impl Parse for SecurityScopes {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let name = input.parse::<LitStr>()?.value();
    input.parse::<Token![=]>()?;
    let scopes;
    bracketed!(scopes in input);
    let scopes = Punctuated::<LitStr, Comma>::parse_terminated(&scopes)?
      .iter()
      .map(LitStr::value)
      .collect::<Vec<_>>();
    Ok(Self { name, scopes })
  }
}
