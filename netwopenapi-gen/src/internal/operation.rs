use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::{Expr, ExprLit, Lit, LitStr, Type};

pub struct Operation<'a> {
  pub args: &'a [Type],
  pub responder_wrapper: &'a proc_macro2::TokenStream,
  pub fn_name: &'a str,
  pub operation_id: Option<Expr>,
  pub deprecated: Option<bool>,
  pub summary: Option<&'a String>,
  pub description: Option<&'a str>,
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
      .map(|deprecated| match deprecated {
        true => quote!(Some(utoipa::openapi::Deprecated::True)),
        false => quote!(Some(utoipa::openapi::Deprecated::False)),
      })
      .unwrap_or_else(|| quote!(None));
    let summary = match self.summary {
      None => quote!(),
      Some(s) => {
        quote!(operation_builder = operation_builder.summary(Some(#s));)
      }
    };
    let description = match self.description {
      None => quote!(),
      Some(d) => {
        quote!(operation_builder = operation_builder.description(Some(#d));)
      }
    };

    tokens.extend(quote!(
      fn operation() -> utoipa::openapi::path::Operation {
        use netwopenapi::ApiComponent;
        let mut operation_builder = utoipa::openapi::path::OperationBuilder::new();

        let mut body_requests = vec![];
        #(
          body_requests.push(<#args>::request_body());
        )*
        let body_requests = body_requests.into_iter().flatten().collect::<Vec<utoipa::openapi::request_body::RequestBody>>();
        for body_request in body_requests {
          operation_builder = operation_builder.request_body(Some(body_request));
        }

        let mut parameters = vec![];
        #(
          parameters.append(&mut <#args>::parameters());
        )*
        if !parameters.is_empty() {
          operation_builder = operation_builder.parameters(Some(parameters));
        }

        if let Some(responses) = <#responder_wrapper>::responses() {
          operation_builder = operation_builder.responses(responses);
        }

        operation_builder = operation_builder.operation_id(Some(#operation_id));

        operation_builder = operation_builder.deprecated(#deprecated);

        #summary
        #description

          // .securities(None)
          // .tag("")

        operation_builder.build()
      }
    ))
  }
}
