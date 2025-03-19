use actix_web::http::StatusCode;
use darling::FromMeta;
use proc_macro_error::abort;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::Attribute;

pub(crate) fn parse_openapi_error_attrs(attrs: &[Attribute]) -> Option<OpenapiErrorAttribute> {
  let error_attribute = attrs
    .iter()
    .filter(|attribute| attribute.path().is_ident("openapi_error"))
    .map(|attribute| OpenapiErrorAttribute::from_meta(&attribute.meta))
    .collect::<darling::Result<Vec<OpenapiErrorAttribute>>>();

  match error_attribute {
    Ok(error_attributes) if error_attributes.len() > 1 => {
      abort!(Span::call_site(), "Expected only one #[openapi_error] attribute")
    }
    Ok(error_attributes) => error_attributes.first().cloned(),
    Err(e) => abort!(e.span(), "Unable to parse #[openapi_error] attribute: {:?}", e),
  }
}

#[derive(FromMeta, Clone)]
pub(crate) struct OpenapiErrorAttribute {
  #[darling(multiple)]
  pub(crate) status: Vec<ErrorDefinition>,
}

impl ToTokens for OpenapiErrorAttribute {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let defs = &self.status;
    tokens.extend(quote! {
      fn error_responses() -> Vec<(String, apistos::paths::Response)> {
        let responses: Vec<((String, apistos::paths::Response), Option<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)>)> = vec![#(#defs,)*];
        responses.into_iter().map(|v| v.0).collect()
      }

      fn schemas_by_status_code() -> std::collections::BTreeMap<String, (String, apistos::reference_or::ReferenceOr<apistos::Schema>)> {
        let mut schemas = std::collections::BTreeMap::default();
        for ((status_code, _), schema) in [#(#defs,)*] {
          if let Some(schema) = schema {
            schemas.insert(status_code, schema);
          }
        }
        schemas
      }
    })
  }
}

#[derive(FromMeta, Clone)]
pub(crate) struct ErrorDefinition {
  pub(crate) code: u16,
  pub(crate) description: Option<String>,
}

impl ToTokens for ErrorDefinition {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let code = self.code;
    let default_description = match StatusCode::from_u16(code) {
      Ok(status_code) => status_code.canonical_reason().unwrap_or_default(),
      Err(e) => abort!(Span::call_site(), format!("{e}")),
    };
    let description = self.description.as_deref().unwrap_or(default_description);
    tokens.extend(quote! {
      ((#code.to_string(), apistos::paths::Response {
        description: #description.to_string(),
        ..Default::default()
      }), None)
    });
  }
}
