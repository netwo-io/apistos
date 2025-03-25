use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

#[derive(FromMeta, Clone)]
#[darling(rename_all = "snake_case")]
pub(crate) struct ApiKey {
  pub(crate) name: String,
  #[darling(rename = "api_key_in")]
  pub(crate) _in: ApiKeyIn,
}

impl ToTokens for ApiKey {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let name = &self.name;
    let _in = self._in.clone();
    tokens.extend(quote! {
      apistos::security::ApiKey {
        name: #name.to_string(),
        _in: #_in
      }
    });
  }
}

#[derive(FromMeta, Clone)]
#[darling(rename_all = "snake_case")]
pub(crate) enum ApiKeyIn {
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
    tokens.extend(quote!(apistos::security::ApiKeyIn::#v))
  }
}

#[derive(FromMeta, Clone)]
pub(crate) struct Http {
  pub(crate) scheme: String,
  pub(crate) bearer_format: Option<String>,
}

impl ToTokens for Http {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let scheme = &self.scheme;
    let bearer_format = if let Some(bearer_format) = &self.bearer_format {
      quote!(Some(#bearer_format.to_string()))
    } else {
      quote!(None)
    };
    tokens.extend(quote! {
      apistos::security::Http {
        scheme: #scheme.to_string(),
        bearer_format: #bearer_format
      }
    });
  }
}

#[derive(FromMeta, Clone)]
pub(crate) struct OpenIdConnect {
  pub(crate) open_id_connect_url: String,
}

impl ToTokens for OpenIdConnect {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let open_id_connect_url = &self.open_id_connect_url;
    tokens.extend(quote! {
      apistos::security::OpenIdConnect {
        open_id_connect_url: #open_id_connect_url.to_string()
      }
    });
  }
}

#[derive(FromMeta, Clone)]
pub(crate) struct OAuth2 {
  pub(crate) flows: OauthFlows,
}

impl ToTokens for OAuth2 {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let flows = &self.flows;
    tokens.extend(quote! {
      apistos::security::OAuth2 {
        flows: #flows
      }
    });
  }
}

#[derive(FromMeta, Clone)]
#[darling(rename_all = "snake_case")]
pub(crate) struct OauthFlows {
  pub(crate) implicit: Option<OauthImplicit>,
  pub(crate) password: Option<OauthToken>,
  pub(crate) client_credentials: Option<OauthToken>,
  pub(crate) authorization_code: Option<OauthToken>,
}

impl ToTokens for OauthFlows {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let implicit = if let Some(implicit) = &self.implicit {
      quote!(Some(#implicit))
    } else {
      quote!(None)
    };
    let password = if let Some(password) = &self.password {
      quote!(Some(#password))
    } else {
      quote!(None)
    };
    let client_credentials = if let Some(credentials) = &self.client_credentials {
      quote!(Some(#credentials))
    } else {
      quote!(None)
    };
    let authorization_code = if let Some(code) = &self.authorization_code {
      quote!(Some(#code))
    } else {
      quote!(None)
    };
    tokens.extend(quote! {
      apistos::security::OauthFlows {
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
pub(crate) struct OauthImplicit {
  pub(crate) authorization_url: String,
  pub(crate) refresh_url: Option<String>,
  #[darling(multiple)]
  pub(crate) scopes: Vec<Scope>,
}

impl ToTokens for OauthImplicit {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let authorization_url = &self.authorization_url;
    let refresh_url = self
      .refresh_url
      .clone()
      .map(|r| quote!(Some(#r.to_string())))
      .unwrap_or_else(|| quote!(None));
    let scopes = if self.scopes.is_empty() {
      quote!(std::collections::BTreeMap::default())
    } else {
      let scopes = &self.scopes;
      quote! {
        std::collections::BTreeMap::from_iter([
          #(#scopes,)*
        ])
      }
    };
    tokens.extend(quote! {
      apistos::security::OauthImplicit {
        authorization_url: #authorization_url.to_string(),
        refresh_url: #refresh_url,
        scopes: #scopes,
      }
    });
  }
}

#[derive(FromMeta, Clone)]
pub(crate) struct OauthToken {
  pub(crate) token_url: String,
  pub(crate) refresh_url: Option<String>,
  #[darling(multiple)]
  pub(crate) scopes: Vec<Scope>,
}

impl ToTokens for OauthToken {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let token_url = &self.token_url;
    let refresh_url = self
      .refresh_url
      .clone()
      .map(|r| quote!(Some(#r.to_string())))
      .unwrap_or_else(|| quote!(None));
    let scopes = if self.scopes.is_empty() {
      quote!(std::collections::BTreeMap::default())
    } else {
      let scopes = &self.scopes;
      quote! {
        std::collections::BTreeMap::from_iter([
          #(#scopes,)*
        ])
      }
    };
    tokens.extend(quote! {
      apistos::security::OauthToken {
        token_url: #token_url.to_string(),
        refresh_url: #refresh_url,
        scopes: #scopes,
      }
    });
  }
}

#[derive(FromMeta, Clone)]
pub(crate) struct Scope {
  pub(crate) scope: String,
  pub(crate) description: String,
}

impl ToTokens for Scope {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let scope = &*self.scope;
    let description = &*self.description;
    tokens.extend(quote! {
      (#scope.to_string(), #description.to_string())
    });
  }
}
