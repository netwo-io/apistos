use actix_web::http::StatusCode;
use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
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
  pub status: Vec<ErrorDefinition>,
}

impl ToTokens for OpenapiErrorAttribute {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let defs = &self.status;
    tokens.extend(quote! {
      fn error_responses() -> Vec<(String, netwopenapi::paths::Response)> {
        let responses: Vec<((String, netwopenapi::paths::Response), Option<(String, netwopenapi::reference_or::ReferenceOr<netwopenapi::Schema>)>)> = vec![#(#defs,)*];
        responses.into_iter().map(|v| v.0).collect()
      }

      fn schemas_by_status_code() -> std::collections::BTreeMap<String, (String, netwopenapi::reference_or::ReferenceOr<netwopenapi::Schema>)> {
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
      quote! {
        content = {
          let settings = schemars::gen::SchemaSettings::openapi3();
          let mut gen = settings.into_generator();
          let schema = <Self as netwopenapi::JsonSchema>::json_schema(&mut gen);
          std::collections::BTreeMap::from_iter(vec![(
            "application/json".to_string(),
            netwopenapi::reference_or::ReferenceOr::Object(netwopenapi::paths::MediaType {
              schema: Some(netwopenapi::reference_or::ReferenceOr::Object(schema)),
              ..Default::default()
            }),
          )])
        },
      }
    } else {
      quote!()
    };
    let schema = if self.with_schema.unwrap_or_default() {
      quote! {
        {
          let schema_name = <Self as schemars::JsonSchema>::schema_name();
          let settings = schemars::gen::SchemaSettings::openapi3();
          let mut gen = settings.into_generator();
          let schema = <Self as schemars::JsonSchema>::json_schema(&mut gen);
          (schema_name, schema)
        }
      }
    } else {
      quote!(None)
    };
    tokens.extend(quote! {
      ((#code.to_string(), netwopenapi::paths::Response {
        description: #description.to_string(),
        #content
        ..Default::default()
      }), #schema)
    });
  }
}
