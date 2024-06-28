use std::collections::BTreeMap;

use darling::FromMeta;
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::{abort, OptionExt};
use quote::{quote, ToTokens};
use syn::{Attribute, Data, DataEnum, Error};

use crate::callback_attr::{CallbackAttr, CallbackAttrInternal};

pub(crate) fn parse_openapi_derive_webhook_attrs(attrs: &[Attribute], data: &Data, ident: &Ident) -> WebhookAttr {
  match data {
    Data::Struct(_) => parse_openapi_derive_struct_webhook_attrs(attrs, ident).expect_or_abort(
      "expected #[openapi_webhook(...)] attribute to be present when used with ApiWebhookComponent derive trait",
    ),
    Data::Enum(_enum) => {
      let struct_attrs = parse_openapi_derive_struct_webhook_attrs(attrs, ident)
        .map(|w| w.0.clone())
        .and_then(|w| w.first_key_value().map(|(_, struct_attrs)| struct_attrs).cloned());
      parse_openapi_derive_enum_webhook_attrs(_enum, struct_attrs.as_ref())
    }
    Data::Union(_) => abort!(
      Span::call_site(),
      "Union are not supported by ApiWebhookComponent derive"
    ),
  }
}

fn parse_openapi_derive_struct_webhook_attrs(attrs: &[Attribute], ident: &Ident) -> Option<WebhookAttr> {
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

      match webhook_attribute
        .map(|w| w.into_webhook_attr(ident.to_string()))
        .transpose()
      {
        Ok(webhook) => webhook,
        Err(e) => abort!(e.span(), "Error parsing openapi_webhook attributes: {}", e),
      }
    }
    Err(e) => abort!(e.span(), "Unable to parse #[openapi_webhook] attribute: {:?}", e),
  }
}

fn parse_openapi_derive_enum_webhook_attrs(_enum: &DataEnum, struct_attrs: Option<&CallbackAttr>) -> WebhookAttr {
  _enum
    .variants
    .iter()
    .map(|v| {
      let variant_name = v.ident.to_string();
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
          let webhook_attribute = webhook_attribute.map(|w| w.into_webhook_attr(struct_attrs, variant_name.clone()))
            .transpose()
            .map(|attrs| attrs.or_else(||
              struct_attrs.map(|struct_webhook_attrs| {
                let mut webhook = BTreeMap::default();
                webhook.insert(variant_name.clone(), struct_webhook_attrs.clone());
                WebhookAttr(webhook)
              })
            ))
            .transpose()
            .expect_or_abort(
              "expected #[openapi_webhook(...)] attribute to be present when used with ApiWebhookComponent derive trait",
            );

          match webhook_attribute {
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
  name: Option<String>,
  #[darling(default, multiple, rename = "tag")]
  tags: Vec<String>,
  #[darling(flatten)]
  attr: CallbackAttrInternal,
}

#[derive(FromMeta, Clone)]
struct WebhookAttrEnumInternal {
  #[darling(default)]
  skip: bool,
  #[darling(default, multiple, rename = "tag")]
  tags: Vec<String>,
  name: Option<String>,
  #[darling(flatten, default)]
  attr: CallbackAttrInternal,
}

#[derive(Clone)]
pub(crate) struct WebhookAttr(BTreeMap<String, CallbackAttr>);

impl WebhookAttrInternal {
  pub(crate) fn into_webhook_attr(self, default_name: String) -> Result<WebhookAttr, Error> {
    if self.attr.responses.is_empty() {
      return Err(Error::new(
        Span::call_site(),
        "Webhook should define at least one response",
      ));
    }

    let mut callback_attr: CallbackAttr = self.attr.try_into()?;
    callback_attr.tags = self.tags;
    Ok(WebhookAttr(BTreeMap::from_iter(vec![(
      self.name.unwrap_or(default_name),
      callback_attr,
    )])))
  }
}

impl WebhookAttrEnumInternal {
  pub(crate) fn into_webhook_attr(
    mut self,
    struct_attrs: Option<&CallbackAttr>,
    default_name: String,
  ) -> Result<WebhookAttr, Error> {
    if self.skip {
      return Ok(WebhookAttr(Default::default()));
    }

    if let Some(struct_attrs) = struct_attrs {
      if self.attr.responses.is_empty() {
        self.attr.responses = struct_attrs.responses.clone();
      }
      if self.attr.components.is_empty() {
        self.attr.components = struct_attrs.components.clone();
      }
    }

    if self.attr.responses.is_empty() {
      return Err(Error::new(
        Span::call_site(),
        "Webhook should define at least one response.",
      ));
    }

    let mut callback_attr: CallbackAttr = self.attr.try_into()?;
    if self.tags.is_empty() {
      callback_attr.tags = struct_attrs.map(|t| t.tags.clone()).unwrap_or_default();
    } else {
      callback_attr.tags = self.tags;
    }
    Ok(WebhookAttr(BTreeMap::from_iter(vec![(
      self.name.unwrap_or(default_name),
      callback_attr,
    )])))
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
      fn webhooks(&self, oas_version: apistos::OpenApiVersion) -> std::collections::BTreeMap<String, apistos::reference_or::ReferenceOr<apistos::paths::PathItem>> {
        if matches!(oas_version, apistos::OpenApiVersion::OAS3_0) {
          return Default::default();
        }

        let mut webhooks = vec![];
        #(webhooks.push(#operations);)*

        std::collections::BTreeMap::from_iter(webhooks)
      }

      fn components(&self, oas_version: apistos::OpenApiVersion) -> Vec<apistos::components::Components> {
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

      fn get_def(oas_version: apistos::OpenApiVersion) -> apistos::ApiWebhookDef {
        if matches!(oas_version, apistos::OpenApiVersion::OAS3_0) {
          return Default::default();
        }

        let mut components: Vec<apistos::components::Components> = vec![];
        #(
        let operation_components = {
          #components
        };
        components.extend(operation_components);
        )*

        let mut webhooks = vec![];
        #(webhooks.push(#operations);)*

        apistos::ApiWebhookDef {
          components,
          webhooks: std::collections::BTreeMap::from_iter(webhooks)
        }
      }
    ))
  }
}
