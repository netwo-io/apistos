use crate::internal::security::models::{ApiKey, Http, OAuth2, OpenIdConnect};
use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::Attribute;

pub(crate) fn parse_openapi_security_attrs(attrs: &[Attribute], struct_name: String) -> Option<SecurityDeclaration> {
  let security_declarations_res = attrs
    .iter()
    .filter(|attribute| attribute.path().is_ident("openapi_security"))
    .map(|attribute| SecurityDeclarationInternal::from_meta(&attribute.meta))
    .collect::<darling::Result<Vec<SecurityDeclarationInternal>>>();

  match security_declarations_res {
    Ok(security_declarations) if security_declarations.len() > 1 => {
      abort!(Span::call_site(), "Expected only one #[openapi_security] attribute")
    }
    Ok(security_declarations) => {
      let security_declaration = security_declarations.first().cloned();
      security_declaration.map(|s| SecurityDeclaration {
        name: s.name.unwrap_or(struct_name),
        scheme: s.scheme,
      })
    }
    Err(e) => abort!(e.span(), "Unable to parse #[openapi_security] attribute: {:?}", e),
  }
}

#[derive(FromMeta, Clone)]
struct SecurityDeclarationInternal {
  pub name: Option<String>,
  pub scheme: SecurityScheme,
}

#[derive(FromMeta, Clone)]
pub(crate) struct SecurityDeclaration {
  pub name: String,
  pub scheme: SecurityScheme,
}

impl ToTokens for SecurityDeclaration {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let name = &self.name;
    let scheme = &self.scheme;
    tokens.extend(quote! {
      std::collections::BTreeMap::from_iter(
        vec![(
          #name.to_string(),
          #scheme
        )]
      )
    })
  }
}

#[derive(FromMeta, Clone)]
#[darling(rename_all = "snake_case")]
pub(crate) enum SecurityScheme {
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
