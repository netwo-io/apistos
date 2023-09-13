use crate::internal::security::Security;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use std::collections::BTreeMap;
use syn::{Expr, ExprLit, Lit, LitStr, Type};

pub struct Operation<'a> {
  pub args: &'a [Type],
  pub responder_wrapper: &'a TokenStream,
  pub fn_name: &'a str,
  pub operation_id: Option<Expr>,
  pub deprecated: Option<bool>,
  pub summary: Option<&'a String>,
  pub description: Option<&'a str>,
  pub tags: &'a [String],
  pub scopes: BTreeMap<String, Vec<String>>,
  pub error_codes: &'a [u16],
}

impl<'a> ToTokens for Operation<'a> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let args = self.args;
    let responder_wrapper = self.responder_wrapper;
    let operation_id = self
      .operation_id
      .clone()
      .or(Some(
        ExprLit {
          attrs: vec![],
          lit: Lit::Str(LitStr::new(self.fn_name, Span::call_site())),
        }
        .into(),
      ))
      .unwrap_or_else(|| {
        abort! {
            Span::call_site(), "operation id is not defined for path";
            help = r###"Try to define it in #[netwopenapi::api_operation(operation_id = {})]"###, &self.fn_name;
            help = "Did you define the #[netwopenapi::api_operation(...)] over function?"
        }
      });
    let deprecated = self
      .deprecated
      .map(|deprecated| quote!(Some(#deprecated)))
      .unwrap_or_else(|| quote!(None));
    let summary = match self.summary {
      None => quote!(),
      Some(s) => {
        quote!(operation_builder.summary = Some(#s.to_string());)
      }
    };
    let description = match self.description {
      None => quote!(),
      Some(d) => {
        quote!(operation_builder.description = Some(#d.to_string());)
      }
    };
    let tags = if self.tags.is_empty() {
      quote!()
    } else {
      let tags = self.tags;
      quote! {
        let tags = vec![
          #(#tags.to_owned(),)*
        ];
        operation_builder.tags = tags;
      }
    };
    let security = Security {
      args,
      scopes: &self.scopes,
    };
    let error_codes_filter = if self.error_codes.is_empty() {
      quote!()
    } else {
      let error_codes = self.error_codes;
      quote! {
        let available_error_codes = vec![#(#error_codes)*,];
        let responses = responses.responses
          .into_iter()
          .filter(|(status, _)| {
            use std::str::FromStr;
            let status = status.parse::<u16>();
            if let Ok(status) = status {
              if status >= 400 {
                return available_error_codes.contains(&status);
              }
            }
            true
          })
          .collect::<std::collections::BTreeMap<String, netwopenapi::reference_or::ReferenceOr<netwopenapi::paths::Response>>>();
       let responses = netwopenapi::paths::Responses {
          responses: std::collections::BTreeMap::from_iter(responses),
          ..Default::default()
        };
      }
    };

    tokens.extend(quote!(
      fn operation() -> netwopenapi::paths::Operation {
        use netwopenapi::ApiComponent;
        let mut operation_builder = netwopenapi::paths::Operation::default();

        let mut body_requests = vec![];
        #(
          body_requests.push(<#args>::request_body());
        )*
        let body_requests = body_requests.into_iter().flatten().collect::<Vec<netwopenapi::paths::RequestBody>>();
        for body_request in body_requests {
          operation_builder.request_body = Some(netwopenapi::reference_or::ReferenceOr::Object(body_request));
        }

        let mut parameters = vec![];
        #(
          parameters.append(&mut <#args>::parameters());
        )*
        if !parameters.is_empty() {
          operation_builder.parameters = parameters.into_iter().map(netwopenapi::reference_or::ReferenceOr::Object).collect();
        }

        if let Some(responses) = <#responder_wrapper>::responses() {
          #error_codes_filter
          operation_builder.responses = responses;
        }

        let securities = {
          #security
        };
        if !securities.is_empty() {
          operation_builder.security = securities;
        }

        operation_builder.operation_id = Some(#operation_id.to_string());

        operation_builder.deprecated = #deprecated;

        #summary
        #description

        #tags

        operation_builder
      }
    ))
  }
}
