use std::collections::BTreeMap;

use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::{abort, OptionExt};
use quote::{quote, ToTokens};
use syn::{Attribute, Data, DataEnum};

use crate::callback_attr::{CallbackAttr, CallbackAttrInternal};

pub(crate) fn parse_openapi_derive_webhook_attrs(attrs: &[Attribute], data: &Data) -> WebhookAttr {
  match data {
    Data::Struct(_) => parse_openapi_derive_struct_webhook_attrs(attrs),
    Data::Enum(_enum) => parse_openapi_derive_enum_webhook_attrs(_enum),
    Data::Union(_) => abort!(
      Span::call_site(),
      "Union are not supported by ApiWebhookComponent derive"
    ),
  }
}

fn parse_openapi_derive_struct_webhook_attrs(attrs: &[Attribute]) -> WebhookAttr {
  let webhook_attribute = attrs
    .iter()
    .filter(|attribute| attribute.path().is_ident("openapi_webhook"))
    .map(|attribute| WebhookAttrInternal::from_meta(&attribute.meta))
    .collect::<darling::Result<Vec<WebhookAttrInternal>>>();

  match webhook_attribute {
    Ok(webhook_attribute) if webhook_attribute.len() > 1 => {
      abort!(Span::call_site(), "Expected only one #[openapi_webhook] attribute")
    }
    Ok(webhook_attribute) => {
      let webhook_attribute = webhook_attribute.first().cloned();

      let webhook_attribute = webhook_attribute.expect_or_abort(
        "expected #[openapi_webhook(...)] attribute to be present when used with ApiWebhookComponent derive trait",
      );
      match webhook_attribute.try_into() {
        Ok(webhook) => webhook,
        Err(e) => abort!(e.span(), "Error parsing openapi_webhook attributes: {}", e),
      }
    }
    Err(e) => abort!(e.span(), "Unable to parse #[openapi_webhook] attribute: {:?}", e),
  }
}

fn parse_openapi_derive_enum_webhook_attrs(_enum: &DataEnum) -> WebhookAttr {
  _enum
    .variants
    .iter()
    .map(|v| {
      let webhook_attribute = v
        .attrs
        .iter()
        .filter(|attribute| attribute.path().is_ident("openapi_webhook"))
        .map(|attribute| WebhookAttrEnumInternal::from_meta(&attribute.meta))
        .collect::<darling::Result<Vec<WebhookAttrEnumInternal>>>();

      match webhook_attribute {
        Ok(webhook_attribute) if webhook_attribute.len() > 1 => {
          abort!(Span::call_site(), "Expected only one #[openapi_webhook] attribute")
        }
        Ok(webhook_attribute) => {
          let webhook_attribute = webhook_attribute.first().cloned();

          let webhook_attribute = webhook_attribute.expect_or_abort(
            "expected #[openapi_webhook(...)] attribute to be present when used with ApiWebhookComponent derive trait",
          );
          match WebhookAttr::try_from(webhook_attribute) {
            Ok(webhook) => webhook,
            Err(e) => abort!(e.span(), "Error parsing openapi_webhook attributes: {}", e),
          }
        }
        Err(e) => abort!(e.span(), "Unable to parse #[openapi_webhook] attribute: {:?}", e),
      }
    })
    .reduce(|acc, attr| {
      let mut attrs = acc.0;
      attrs.extend(attr.0);
      WebhookAttr(attrs)
    })
    .expect_or_abort(
      "expected #[openapi_webhook(...)] attribute to be present when used with ApiWebhookComponent derive trait",
    )
}

#[derive(FromMeta, Clone)]
struct WebhookAttrInternal {
  name: String,
  #[darling(flatten)]
  attr: CallbackAttrInternal,
}

#[derive(FromMeta, Clone)]
struct WebhookAttrEnumInternal {
  #[darling(default)]
  skip: bool,
  name: Option<String>,
  #[darling(flatten, default)]
  attr: CallbackAttrInternal,
}

#[derive(Clone)]
pub(crate) struct WebhookAttr(BTreeMap<String, CallbackAttr>);

impl TryFrom<WebhookAttrInternal> for WebhookAttr {
  type Error = syn::Error;

  fn try_from(value: WebhookAttrInternal) -> Result<Self, Self::Error> {
    if value.attr.responses.is_empty() {
      return Err(syn::Error::new(
        Span::call_site(),
        "Webhook should define at least one response",
      ));
    }

    Ok(Self(BTreeMap::from_iter(vec![(value.name, value.attr.try_into()?)])))
  }
}

impl TryFrom<WebhookAttrEnumInternal> for WebhookAttr {
  type Error = syn::Error;

  fn try_from(value: WebhookAttrEnumInternal) -> Result<Self, Self::Error> {
    if value.skip {
      return Ok(WebhookAttr(Default::default()));
    }

    match value.name {
      None => {
        return Err(syn::Error::new(
          Span::call_site(),
          "Missing name for webhook definition.",
        ))
      }
      Some(name) => {
        if value.attr.responses.is_empty() {
          return Err(syn::Error::new(
            Span::call_site(),
            "Webhook should define at least one response.",
          ));
        }

        Ok(Self(BTreeMap::from_iter(vec![(name, value.attr.try_into()?)])))
      }
    }
  }
}

impl ToTokens for WebhookAttr {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let mut components = vec![];
    let mut operations = vec![];

    for (name, callback_attr) in &self.0 {
      components.push(callback_attr.components());

      let operation = callback_attr.operation();
      operations.push(quote!(
        (#name.to_string(), apistos::reference_or::ReferenceOr::Object(apistos::paths::PathItem {
          operations: apistos::IndexMap::from_iter(vec![(apistos::paths::OperationType::Post, {
            #operation
          })]),
          ..Default::default()
        }))
      ));
    }

    tokens.extend(quote!(
      fn webhooks(oas_version: apistos::OpenApiVersion) -> std::collections::BTreeMap<String, apistos::reference_or::ReferenceOr<apistos::paths::PathItem>> {
        if matches!(oas_version, apistos::OpenApiVersion::OAS3_0) {
          return Default::default();
        }

        let mut webhooks = vec![];
        #(webhooks.push(#operations);)*

        std::collections::BTreeMap::from_iter(webhooks)
      }

      fn components(oas_version: apistos::OpenApiVersion) -> Vec<apistos::components::Components> {
        if matches!(oas_version, apistos::OpenApiVersion::OAS3_0) {
          return vec![];
        }

        let mut components: Vec<apistos::components::Components> = vec![];
        #(
        let operation_components = {
          #components
        };
        components.extend(operation_components);
        )*
        components
      }
    ))
  }
}
