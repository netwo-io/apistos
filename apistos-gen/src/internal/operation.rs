use std::collections::BTreeMap;

use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::Type;

use crate::internal::security::Security;
use crate::operation_attr::OperationCallbacks;

pub(crate) struct Operation<'a> {
  pub(crate) args: &'a [(Option<Ident>, Type)],
  pub(crate) responder_wrapper: &'a TokenStream,
  pub(crate) operation_id: Option<&'a String>,
  pub(crate) deprecated: Option<bool>,
  pub(crate) summary: Option<&'a String>,
  pub(crate) description: Option<&'a str>,
  pub(crate) success_description: Option<&'a str>,
  pub(crate) body_description: Option<&'a str>,
  pub(crate) parameter_description: BTreeMap<String, String>,
  pub(crate) tags: &'a [String],
  pub(crate) scopes: BTreeMap<String, Vec<String>>,
  pub(crate) callbacks: &'a [OperationCallbacks],
  pub(crate) error_codes: &'a [u16],
  pub(crate) consumes: Option<&'a String>,
  pub(crate) produces: Option<&'a String>,
}

impl ToTokens for Operation<'_> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let args = self.args;
    let responder_wrapper = self.responder_wrapper;
    let operation_id = self.operation_id;
    let operation_id = match operation_id {
      None => quote!(None),
      Some(op_id) => quote!(Some(#op_id.to_string())),
    };
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
    let success_description = match self.success_description {
      None => quote!(None),
      Some(d) => {
        quote!(Some(#d.to_string()))
      }
    };
    let body_description = match self.body_description {
      None => quote!(None),
      Some(d) => {
        quote!(Some(#d.to_string()))
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
    let mut parameters = quote!(let mut parameters = vec![];);
    for (parameter_ident, parameter_type) in args {
      let parameter_description = parameter_ident
        .clone()
        .and_then(|ident| self.parameter_description.get(&ident.to_string()));
      let parameter_description = if let Some(parameter_description) = parameter_description {
        quote!(Some(#parameter_description.to_string()))
      } else {
        quote!(None)
      };
      parameters
        .extend(quote!(parameters.append(&mut <#parameter_type>::parameters(oas_version, #parameter_description));));
    }

    let args = &args.iter().map(|(_, _type)| _type).cloned().collect::<Vec<Type>>();
    let security = Security {
      args,
      scopes: &self.scopes,
    };
    let error_codes_filter = if self.error_codes.is_empty() {
      quote!()
    } else {
      let error_codes = self.error_codes;
      quote! {
        let available_error_codes = [#(#error_codes,)*];
        let mut responses = responses.responses
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
          .collect::<std::collections::BTreeMap<String, apistos::reference_or::ReferenceOr<apistos::paths::Response>>>();
        let responses = apistos::paths::Responses {
          responses: std::collections::BTreeMap::from_iter(responses),
          ..Default::default()
        };
      }
    };

    let callbacks = if self.callbacks.is_empty() {
      quote!()
    } else {
      let mut callbacks_tokens = vec![];
      for callback in self.callbacks {
        let name = &callback.name;
        let mut callbacks_ops_tokens = vec![];
        for (path, operations) in &callback.callbacks {
          let mut ops_tokens = vec![];
          for (operation_type, fn_name) in operations {
            ops_tokens.push(quote!(
              (#operation_type, <#fn_name>::operation(oas_version))
            ));
          }
          callbacks_ops_tokens.push(quote!((#path.to_string(), apistos::paths::PathItem {
            operations: apistos::IndexMap::from_iter(vec![#(#ops_tokens,)*]),
            ..Default::default()
          })))
        }
        callbacks_tokens.push(quote!({
          (
            #name.to_string(),
            apistos::reference_or::ReferenceOr::Object(
              apistos::paths::Callback {
                callbacks: std::collections::BTreeMap::from_iter(vec![#(#callbacks_ops_tokens,)*]),
                ..Default::default()
              }
            )
          )
        }));
      }
      quote! {
        let callbacks = vec![
          #(#callbacks_tokens,)*
        ];
        operation_builder.callbacks = std::collections::BTreeMap::from_iter(callbacks);
      }
    };

    let consumes = if let Some(consumes) = self.consumes {
      quote!(Some(#consumes.to_string()))
    } else {
      quote!(None)
    };
    let produces = if let Some(produces) = self.produces {
      quote!(Some(#produces.to_string()))
    } else {
      quote!(None)
    };

    tokens.extend(quote!(
      fn operation(oas_version: apistos::OpenApiVersion) -> apistos::paths::Operation {
        use apistos::ApiComponent;
        let mut operation_builder = apistos::paths::Operation::default();

        let mut body_requests: Vec<std::option::Option<apistos::paths::RequestBody>> = vec![];
        #(
          let mut request_body = <#args>::request_body(oas_version, #body_description);
          let consumes: Option<String> = #consumes;
          if let Some(consumes) = consumes {
            request_body
              .as_mut()
              .map(|t|
                t.content = t
                  .content
                  .values()
                  .map(|v| (consumes.clone(), v.clone())).collect::<std::collections::BTreeMap<String, apistos::paths::MediaType>>()
              );
          }
          body_requests.push(request_body);
        )*
        let body_requests = body_requests.into_iter().flatten().collect::<Vec<apistos::paths::RequestBody>>();
        for body_request in body_requests {
          operation_builder.request_body = Some(apistos::reference_or::ReferenceOr::Object(body_request));
        }

        #parameters
        if !parameters.is_empty() {
          operation_builder.parameters = parameters.into_iter().map(apistos::reference_or::ReferenceOr::Object).collect();
        }

        if let Some(responses) = <#responder_wrapper>::responses(oas_version, #produces, #success_description) {
          #error_codes_filter
          operation_builder.responses = responses;
        }

        let securities = {
          #security
        };
        if !securities.is_empty() {
          operation_builder.security = securities;
        }

        #callbacks

        operation_builder.operation_id = #operation_id;

        operation_builder.deprecated = #deprecated;

        #summary
        #description

        #tags

        operation_builder
      }
    ))
  }
}
