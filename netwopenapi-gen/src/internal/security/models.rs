use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(FromMeta, Clone)]
#[darling(rename_all = "snake_case")]
pub enum ApiKey {
  Header(ApiKeyValue),
  Query(ApiKeyValue),
  Cookie(ApiKeyValue),
}

impl ToTokens for ApiKey {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let api_key_tokens = match self {
      ApiKey::Header(v) => quote!(Header(#v)),
      ApiKey::Query(v) => quote!(Query(#v)),
      ApiKey::Cookie(v) => quote!(Cookie(#v)),
    };
    tokens.extend(quote! {
      utoipa::openapi::security::ApiKey::#api_key_tokens
    });
  }
}

#[derive(FromMeta, Clone)]
pub struct ApiKeyValue {
  pub name: String,
  pub description: Option<String>,
}

impl ToTokens for ApiKeyValue {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let name = &self.name;
    let api_key_value_tokens = match &self.description {
      None => quote!(new(#name)),
      Some(d) => quote!(with_description(#name, #d)),
    };
    tokens.extend(quote! {
      utoipa::openapi::security::ApiKeyValue::#api_key_value_tokens
    });
  }
}

#[derive(FromMeta, Clone)]
pub struct Http {
  pub scheme: HttpAuthScheme,
  pub bearer_format: Option<String>,
  pub description: Option<String>,
}

impl ToTokens for Http {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let scheme = &self.scheme;
    let bearer_format = match &self.bearer_format {
      None => quote!(),
      Some(b) => quote!(.bearer_format(#b)),
    };
    let description = match &self.description {
      None => quote!(),
      Some(d) => quote!(.bearer_format(#d)),
    };
    tokens.extend(quote! {
      utoipa::openapi::security::HttpBuilder::new()
        .scheme(#scheme)
        #bearer_format
        #description
        .build()
    });
  }
}

#[derive(FromMeta, Clone)]
#[darling(rename_all = "snake_case")]
pub enum HttpAuthScheme {
  Basic,
  Bearer,
  Digest,
  Hoba,
  Mutual,
  Negotiate,
  OAuth,
  ScramSha1,
  ScramSha256,
  Vapid,
}

impl ToTokens for HttpAuthScheme {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let scheme = match self {
      HttpAuthScheme::Basic => quote!(Basic),
      HttpAuthScheme::Bearer => quote!(Bearer),
      HttpAuthScheme::Digest => quote!(Digest),
      HttpAuthScheme::Hoba => quote!(Hoba),
      HttpAuthScheme::Mutual => quote!(Mutual),
      HttpAuthScheme::Negotiate => quote!(Negotiate),
      HttpAuthScheme::OAuth => quote!(OAuth),
      HttpAuthScheme::ScramSha1 => quote!(ScramSha1),
      HttpAuthScheme::ScramSha256 => quote!(ScramSha256),
      HttpAuthScheme::Vapid => quote!(Vapid),
    };
    tokens.extend(quote! {
      utoipa::openapi::security::HttpAuthScheme::#scheme
    });
  }
}

#[derive(FromMeta, Clone)]
pub struct OpenIdConnect {
  pub open_id_connect_url: String,
  pub description: Option<String>,
}

impl ToTokens for OpenIdConnect {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let open_id_connect_url = &self.open_id_connect_url;
    let open_id_connect_tokens = match &self.description {
      None => quote!(new(#open_id_connect_url)),
      Some(d) => quote!(with_description(#open_id_connect_url, #d)),
    };
    tokens.extend(quote! {
      utoipa::openapi::security::OpenIdConnect::#open_id_connect_tokens
    });
  }
}

#[derive(FromMeta, Clone)]
pub struct OAuth2 {
  #[darling(multiple)]
  pub flows: Vec<Flow>,
  pub description: Option<String>,
}

impl ToTokens for OAuth2 {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let flows = &self.flows;
    let o_auth_2_tokens = match &self.description {
      None => quote!(new(#(#flows)*)),
      Some(d) => quote!(with_description(#(#flows)*, #d)),
    };
    tokens.extend(quote! {
      utoipa::openapi::security::OAuth2::#o_auth_2_tokens
    });
  }
}

#[derive(FromMeta, Clone)]
#[darling(rename_all = "snake_case")]
pub enum Flow {
  Implicit(Implicit),
  Password(Password),
  ClientCredentials(ClientCredentials),
  AuthorizationCode(AuthorizationCode),
}

impl ToTokens for Flow {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let flow_tokens = match self {
      Flow::Implicit(v) => quote!(Implicit(#v)),
      Flow::Password(v) => quote!(Password(#v)),
      Flow::ClientCredentials(v) => quote!(ClientCredentials(#v)),
      Flow::AuthorizationCode(v) => quote!(AuthorizationCode(#v)),
    };
    tokens.extend(quote! {
      utoipa::openapi::security::Flow::#flow_tokens
    });
  }
}

#[derive(FromMeta, Clone)]
pub struct Implicit {
  pub authorization_url: String,
  pub refresh_url: Option<String>,
  pub scopes: Scopes,
}

impl ToTokens for Implicit {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let authorization_url = &self.authorization_url;
    let scopes = &self.scopes;
    let implicit_tokens = match &self.refresh_url {
      None => quote!(new(#authorization_url, #scopes)),
      Some(r) => quote!(with_refresh_url(#authorization_url, #scopes, #r)),
    };
    tokens.extend(quote! {
      utoipa::openapi::security::Implicit::#implicit_tokens
    });
  }
}

#[derive(FromMeta, Clone)]
pub struct AuthorizationCode {
  pub authorization_url: String,
  pub token_url: String,
  pub refresh_url: Option<String>,
  pub scopes: Scopes,
}

impl ToTokens for AuthorizationCode {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let authorization_url = &self.authorization_url;
    let token_url = &self.token_url;
    let scopes = &self.scopes;
    let authorization_tokens = match &self.refresh_url {
      None => quote!(new(#authorization_url, #token_url, #scopes)),
      Some(r) => quote!(with_refresh_url(#authorization_url, #token_url, #scopes, #r)),
    };
    tokens.extend(quote! {
      utoipa::openapi::security::AuthorizationCode::#authorization_tokens
    });
  }
}

#[derive(FromMeta, Clone)]
pub struct Password {
  pub token_url: String,
  pub refresh_url: Option<String>,
  pub scopes: Scopes,
}

impl ToTokens for Password {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let token_url = &self.token_url;
    let scopes = &self.scopes;
    let password_tokens = match &self.refresh_url {
      None => quote!(new(#token_url, #scopes)),
      Some(r) => quote!(with_refresh_url(#token_url, #scopes, #r)),
    };
    tokens.extend(quote! {
      utoipa::openapi::security::Password::#password_tokens
    });
  }
}

#[derive(FromMeta, Clone)]
pub struct ClientCredentials {
  pub token_url: String,
  pub refresh_url: Option<String>,
  pub scopes: Scopes,
}

impl ToTokens for ClientCredentials {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let token_url = &self.token_url;
    let scopes = &self.scopes;
    let password_tokens = match &self.refresh_url {
      None => quote!(new(#token_url, #scopes)),
      Some(r) => quote!(with_refresh_url(#token_url, #scopes, #r)),
    };
    tokens.extend(quote! {
      utoipa::openapi::security::ClientCredentials::#password_tokens
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
      tokens.extend(quote!(utoipa::openapi::security::Scopes::new()))
    } else {
      tokens.extend(quote! {
        utoipa::openapi::security::Scopes::from_iter([
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
