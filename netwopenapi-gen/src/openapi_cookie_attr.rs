use crate::internal::schemas::Schemas;
use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::Attribute;

pub(crate) fn parse_openapi_cookie_attrs(
  attrs: &[Attribute],
  deprecated: Option<bool>,
) -> Option<OpenapiCookieAttributeExtended> {
  let cookie_attribute = attrs
    .iter()
    .filter(|attribute| attribute.path().is_ident("openapi_cookie"))
    .map(|attribute| OpenapiCookieAttribute::from_meta(&attribute.meta))
    .collect::<darling::Result<Vec<OpenapiCookieAttribute>>>();

  match cookie_attribute {
    Ok(cookie_attributes) if cookie_attributes.len() > 1 => {
      abort!(Span::call_site(), "Expected only one #[openapi_cookie] attribute")
    }
    Ok(cookie_attributes) => {
      let cookie_attribute = cookie_attributes.first().cloned();
      cookie_attribute.map(|attr| OpenapiCookieAttributeExtended {
        name: attr.name,
        description: attr.description,
        required: attr.required,
        deprecated: attr.deprecated.or(deprecated),
      })
    }
    Err(e) => abort!(e.span(), "Unable to parse #[openapi_cookie] attribute: {:?}", e),
  }
}

#[derive(FromMeta, Clone)]
struct OpenapiCookieAttribute {
  pub name: String,
  pub description: Option<String>,
  pub required: Option<bool>,
  pub deprecated: Option<bool>,
}

#[derive(Clone)]
pub(crate) struct OpenapiCookieAttributeExtended {
  pub name: String,
  pub description: Option<String>,
  pub required: Option<bool>,
  pub deprecated: Option<bool>,
}

impl ToTokens for OpenapiCookieAttributeExtended {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let name = self.name.as_str();
    let description = match &self.description {
      None => quote!(None),
      Some(desc) => quote!(Some(#desc.to_string())),
    };
    let required = self.required.unwrap_or_default();
    let required = quote!(#required);
    let deprecated = self.deprecated.unwrap_or_default();
    let deprecated = quote!(#deprecated);

    let schema_impl = Schemas;
    tokens.extend(quote! {
      #schema_impl

      fn request_body() -> Option<netwopenapi::paths::RequestBody> {
        None
      }

      fn parameters() -> Vec<netwopenapi::paths::Parameter> {
        vec![
          netwopenapi::paths::Parameter {
            name: #name.to_string(),
            description: #description,
            _in: netwopenapi::paths::ParameterIn::Cookie,
            required: Some(#required),
            deprecated: Some(#deprecated),
            definition: <Self as netwopenapi::ApiComponent>::schema()
              .map(|(_, schema)| schema)
              .or_else(|| Self::raw_schema())
              .map(netwopenapi::paths::ParameterDefinition::Schema),
            ..Default::default()
          }
        ]
      }
    })
  }
}
