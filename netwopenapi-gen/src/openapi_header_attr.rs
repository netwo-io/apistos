use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::Attribute;

pub(crate) const RESERVED_HEADERS: &[&str] = &["Accept", "Content-Type", "Authorization"];

pub(crate) fn parse_openapi_header_attrs(
  attrs: &[Attribute],
  deprecated: Option<bool>,
) -> Option<OpenapiHeaderAttribute> {
  let header_attribute = attrs
    .iter()
    .filter(|attribute| attribute.path().is_ident("openapi_header"))
    .map(|attribute| OpenapiHeaderAttribute::from_meta(&attribute.meta))
    .collect::<darling::Result<Vec<OpenapiHeaderAttribute>>>();

  match header_attribute {
    Ok(header_attributes) if header_attributes.len() > 1 => {
      abort!(Span::call_site(), "Expected only one #[openapi_header] attribute")
    }
    Ok(header_attributes) => {
      let mut header_attribute = header_attributes.first().cloned();
      if let Some(header_attribute) = &mut header_attribute {
        header_attribute.deprecated = header_attribute.deprecated.or_else(|| deprecated)
      }
      header_attribute
    }
    Err(e) => abort!(e.span(), "Unable to parse #[openapi_header] attribute: {:?}", e),
  }
}

#[derive(FromMeta, Clone)]
pub(crate) struct OpenapiHeaderAttribute {
  pub name: String,
  pub description: Option<String>,
  pub required: Option<bool>,
  pub deprecated: Option<bool>,
}

impl ToTokens for OpenapiHeaderAttribute {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let name = self.name.as_str();
    if RESERVED_HEADERS.contains(&name) {
      abort!(
        Span::call_site(),
        format!("Header name can't be any of {:?} (reserved headers)", RESERVED_HEADERS)
      );
    }
    let description = match &self.description {
      None => quote!(None),
      Some(desc) => quote!(Some(#desc.to_string())),
    };
    let required = if self.required.unwrap_or_default() {
      quote!(utoipa::openapi::Required::True)
    } else {
      quote!(utoipa::openapi::Required::False)
    };
    let deprecated = if self.deprecated.unwrap_or_default() {
      quote!(utoipa::openapi::Deprecated::True)
    } else {
      quote!(utoipa::openapi::Deprecated::False)
    };

    tokens.extend(quote! {
      fn name() -> String {
        #name.to_string()
      }

      fn description() -> Option<String> {
        #description
      }

      fn required() -> utoipa::openapi::Required {
        #required
      }

      fn deprecated() -> utoipa::openapi::Deprecated {
        #deprecated
      }
    })
  }
}
