use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Type;

pub struct Operation<'a> {
  pub args: &'a [Type],
  pub responder_wrapper: &'a proc_macro2::TokenStream,
}

impl<'a> ToTokens for Operation<'a> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let args = self.args;
    let responder_wrapper = self.responder_wrapper;
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

          // .operation_id(None)
          // .summary(None)
          // .description(None)
          // .tag("")
        operation_builder.build()
      }
    ))
  }
}
