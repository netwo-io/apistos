use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Type;

pub struct Operation<'a> {
  pub args: &'a [Type],
}

impl<'a> ToTokens for Operation<'a> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let args = self.args;
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
          // .responses(utoipa::openapi::ResponsesBuilder::new().build())
          // .operation_id(None)
          // .summary(None)
          // .description(None)
          // .tag("")
        operation_builder.build()
      }
    ))
  }
}
