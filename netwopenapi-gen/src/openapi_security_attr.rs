use crate::internal::security::models::{ApiKey, Http, OAuth2, OpenIdConnect};
use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::Attribute;

pub fn parse_openapi_security_attrs(attrs: &[Attribute]) -> Option<SecurityScheme> {
  let security_schemes_res = attrs
    .iter()
    .filter(|attribute| attribute.path().is_ident("openapi_security"))
    .map(|attribute| SecurityScheme::from_meta(&attribute.meta))
    .collect::<darling::Result<Vec<SecurityScheme>>>();

  match security_schemes_res {
    Ok(security_schemes) if security_schemes.len() > 1 => {
      abort!(Span::call_site(), "Expected only one #[openapi_security] attribute")
    }
    Ok(security_schemes) => security_schemes.first().cloned(),
    Err(e) => abort!(e.span(), "Unable to parse #[openapi_security] attribute: {:?}", e),
  }
}

#[derive(FromMeta, Clone)]
#[darling(rename_all = "snake_case")]
pub enum SecurityScheme {
  OAuth2(OAuth2),
  ApiKey(ApiKey),
  Http(Http),
  OpenIdConnect(OpenIdConnect),
}

impl ToTokens for SecurityScheme {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let scheme_tokens = match self {
      SecurityScheme::OAuth2(v) => quote!(OAuth2(#v)),
      SecurityScheme::ApiKey(v) => quote!(ApiKey(#v)),
      SecurityScheme::Http(v) => quote!(Http(#v)),
      SecurityScheme::OpenIdConnect(v) => quote!(OpenIdConnect(#v)),
    };
    tokens.extend(quote! {
      utoipa::openapi::security::SecurityScheme::#scheme_tokens
    });
  }
}
