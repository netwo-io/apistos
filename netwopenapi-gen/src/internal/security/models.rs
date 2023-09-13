use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(FromMeta, Clone)]
#[darling(rename_all = "snake_case")]
pub struct ApiKey {
  pub name: String,
  #[darling(rename = "api_key_in")]
  pub _in: ApiKeyIn,
}

impl ToTokens for ApiKey {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let name = &self.name;
    let _in = self._in.clone();
    tokens.extend(quote! {
      netwopenapi::security::ApiKey {
        name: #name.to_string(),
        _in: #_in
      }
    });
  }
}

#[derive(FromMeta, Clone)]
#[darling(rename_all = "snake_case")]
pub enum ApiKeyIn {
  Query,
  Header,
  Cookie,
}

impl ToTokens for ApiKeyIn {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let v = match self {
      ApiKeyIn::Query => quote!(Query),
      ApiKeyIn::Header => quote!(Header),
      ApiKeyIn::Cookie => quote!(Cookie),
    };
    tokens.extend(quote!(netwopenapi::security::ApiKeyIn::#v))
  }
}

#[derive(FromMeta, Clone)]
pub struct Http {
  pub scheme: String,
  pub bearer_format: String,
}

impl ToTokens for Http {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let scheme = &self.scheme;
    let bearer_format = &self.bearer_format;
    tokens.extend(quote! {
      netwopenapi::security::Http {
        scheme: #scheme.to_string(),
        bearer_format: #bearer_format.to_string()
      }
    });
  }
}

#[derive(FromMeta, Clone)]
pub struct OpenIdConnect {
  pub open_id_connect_url: String,
}

impl ToTokens for OpenIdConnect {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let open_id_connect_url = &self.open_id_connect_url;
    tokens.extend(quote! {
      netwopenapi::security::OpenIdConnect {
        open_id_connect_url: #open_id_connect_url.to_string()
      }
    });
  }
}

#[derive(FromMeta, Clone)]
pub struct OAuth2 {
  pub flows: OauthFlows,
}

impl ToTokens for OAuth2 {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let flows = &self.flows;
    tokens.extend(quote! {
      netwopenapi::security::OAuth2 {
        flows: #flows
      }
    });
  }
}

#[derive(FromMeta, Clone)]
#[darling(rename_all = "snake_case")]
pub struct OauthFlows {
  pub implicit: OauthImplicit,
  pub password: OauthToken,
  pub client_credentials: OauthToken,
  pub authorization_code: OauthToken,
}

impl ToTokens for OauthFlows {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let implicit = &self.implicit;
    let password = &self.password;
    let client_credentials = &self.client_credentials;
    let authorization_code = &self.authorization_code;
    tokens.extend(quote! {
      netwopenapi::security::OauthImplicit {
        implicit: #implicit,
        password: #password,
        client_credentials: #client_credentials,
        authorization_code: #authorization_code,
        ..Default::default()
      }
    });
  }
}

#[derive(FromMeta, Clone)]
#[darling(rename_all = "snake_case")]
pub struct OauthImplicit {
  pub authorization_url: String,
  pub refresh_url: Option<String>,
  pub scopes: Scopes,
}

impl ToTokens for OauthImplicit {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let authorization_url = &self.authorization_url;
    let refresh_url = self
      .refresh_url
      .clone()
      .map(|r| quote!(Some(#r.to_string())))
      .unwrap_or_else(|| quote!(None));
    let scopes = &self.scopes;
    tokens.extend(quote! {
      authorization_url: #authorization_url.to_string(),
      refresh_url: #refresh_url,
      scopes: #scopes,
    });
  }
}

#[derive(FromMeta, Clone)]
pub struct OauthToken {
  pub token_url: String,
  pub refresh_url: Option<String>,
  pub scopes: Scopes,
}

impl ToTokens for OauthToken {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let token_url = &self.token_url;
    let refresh_url = self
      .refresh_url
      .clone()
      .map(|r| quote!(Some(#r.to_string())))
      .unwrap_or_else(|| quote!(None));
    let scopes = &self.scopes;
    tokens.extend(quote! {
      token_url: #token_url.to_string(),
      refresh_url: #refresh_url,
      scopes: #scopes,
    });
  }
}

#[derive(FromMeta, Clone)]
pub struct Scopes {
  #[darling(multiple)]
  pub scopes: Vec<Scope>,
}

impl ToTokens for Scopes {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let scopes = &self.scopes;
    if scopes.is_empty() {
      tokens.extend(quote!(std::collections::BTreeMap::default()))
    } else {
      tokens.extend(quote! {
        std::collections::BTreeMap::from_iter([
          #(#scopes,)*
        ])
      });
    }
  }
}

#[derive(FromMeta, Clone)]
pub struct Scope {
  pub scope: String,
  pub description: String,
}

impl ToTokens for Scope {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let scope = &*self.scope;
    let description = &*self.description;
    tokens.extend(quote! {
      (#scope, #description)
    });
  }
}
