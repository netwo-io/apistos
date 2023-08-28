use actix_web::http::StatusCode;
use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::Attribute;

pub fn parse_openapi_error_attrs(attrs: &[Attribute]) -> Option<OpenapiErrorAttribute> {
  let security_schemes_res = attrs
    .iter()
    .filter(|attribute| attribute.path().is_ident("openapi_error"))
    .map(|attribute| OpenapiErrorAttribute::from_meta(&attribute.meta))
    .collect::<darling::Result<Vec<OpenapiErrorAttribute>>>();

  match security_schemes_res {
    Ok(security_schemes) if security_schemes.len() > 1 => {
      abort!(Span::call_site(), "Expected only one #[openapi_error] attribute")
    }
    Ok(security_schemes) => security_schemes.first().cloned(),
    Err(e) => abort!(e.span(), "Unable to parse #[openapi_error] attribute: {:?}", e),
  }
}

#[derive(FromMeta, Clone)]
pub struct OpenapiErrorAttribute {
  #[darling(multiple)]
  pub status: Vec<ErrorDefinition>,
}

impl ToTokens for OpenapiErrorAttribute {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let defs = &self.status;
    tokens.extend(quote! {
      fn error_responses() -> Vec<(String, utoipa::openapi::Response)> {
        let responses: Vec<((String, utoipa::openapi::Response), Option<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)>)> = vec![#(#defs,)*];
        responses.into_iter().map(|v| v.0).collect()
      }

      fn schemas() -> Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)> {
        let mut schemas = vec![#(#defs.1,)*];
        schemas.into_iter().flatten().collect::<Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)>>()
      }
    })
  }
}

#[derive(FromMeta, Clone)]
pub struct ErrorDefinition {
  pub code: u16,
  pub description: Option<String>,
  pub with_schema: Option<bool>,
}

impl ToTokens for ErrorDefinition {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let code = self.code;
    let default_description = match StatusCode::from_u16(code) {
      Ok(status_code) => status_code.canonical_reason().unwrap_or_default(),
      Err(e) => abort!(Span::call_site(), format!("{e}")),
    };
    let description = self.description.as_deref().unwrap_or_else(|| default_description);
    let content = if self.with_schema.unwrap_or_default() {
      quote!(.content(
        "application/json",
        utoipa::openapi::ContentBuilder::new().schema({
          let (_, schema) = <Self as utoipa::ToSchema<'_>>::schema();
          schema
        }).build(),
      ))
    } else {
      quote!()
    };
    let schema = if self.with_schema.unwrap_or_default() {
      quote!(<Self as utoipa::ToSchema<'_>>::schema())
    } else {
      quote!(None)
    };
    tokens.extend(quote! {
      ((#code.to_string(), utoipa::openapi::ResponseBuilder::new()#content.description(#description).build()), #schema)
    });
  }
}
