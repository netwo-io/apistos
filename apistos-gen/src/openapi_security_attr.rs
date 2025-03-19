use crate::internal::security::models::{ApiKey, Http, OAuth2, OpenIdConnect};
use darling::FromMeta;
use proc_macro_error::abort;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
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
  name: Option<String>,
  scheme: SecurityScheme,
}

#[derive(FromMeta, Clone)]
pub(crate) struct SecurityDeclaration {
  pub(crate) name: String,
  pub(crate) scheme: SecurityScheme,
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
pub(crate) struct SecurityScheme {
  #[darling(rename = "security_type")]
  pub(crate) _type: SecurityType,
  pub(crate) description: Option<String>,
}

impl ToTokens for SecurityScheme {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let description = self
      .clone()
      .description
      .map(|d| quote!(Some(#d.to_string())))
      .unwrap_or_else(|| quote!(None));
    let _type = self._type.clone();
    tokens.extend(quote! {
      apistos::security::SecurityScheme {
        _type: #_type,
        description: #description,
        extensions: apistos::IndexMap::default()
      }
    });
  }
}

#[derive(FromMeta, Clone)]
#[darling(rename_all = "snake_case")]
pub(crate) enum SecurityType {
  #[darling(rename = "oauth2")]
  OAuth2(Box<OAuth2>),
  ApiKey(ApiKey),
  Http(Http),
  OpenIdConnect(OpenIdConnect),
}

impl ToTokens for SecurityType {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let scheme_tokens = match self {
      SecurityType::OAuth2(v) => {
        let v = *v.clone();
        quote!(OAuth2(#v))
      }
      SecurityType::ApiKey(v) => quote!(ApiKey(#v)),
      SecurityType::Http(v) => quote!(Http(#v)),
      SecurityType::OpenIdConnect(v) => quote!(OpenIdConnect(#v)),
    };
    tokens.extend(quote! {
      apistos::security::SecurityType::#scheme_tokens
    });
  }
}
