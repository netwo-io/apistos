use darling::ast::NestedMeta;
use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::Type;

pub(crate) fn parse_openapi_callback_attrs(attrs: &[NestedMeta]) -> CallbackAttr {
  match CallbackAttrInternal::from_list(attrs) {
    Ok(callback) => match callback.try_into() {
      Ok(callback) => callback,
      Err(e) => abort!(e.span(), "Error parsing callback attributes: {}", e),
    },
    Err(e) => abort!(e.span(), "Unable to parse #[api_callback] attribute: {}", e),
  }
}

#[derive(FromMeta, Clone, Default)]
pub(crate) struct CallbackAttrInternal {
  #[darling(default)]
  pub(crate) deprecated: Option<bool>,
  pub(crate) summary: Option<String>,
  pub(crate) description: Option<String>,
  #[darling(default, multiple, rename = "component")]
  pub(crate) components: Vec<Type>,
  #[darling(multiple, rename = "response")]
  pub(crate) responses: Vec<CallbackResponseDefinition>,
}

#[derive(FromMeta, Clone)]
pub(crate) struct CallbackResponseDefinition {
  pub(crate) code: u16,
  #[darling(default)]
  pub(crate) component: Option<Type>,
}

#[derive(Clone)]
pub(crate) struct CallbackAttr {
  deprecated: Option<bool>,
  summary: Option<String>,
  description: Option<String>,
  components: Vec<Type>,
  responses: Vec<CallbackResponseDefinition>,
}

impl TryFrom<CallbackAttrInternal> for CallbackAttr {
  type Error = syn::Error;

  fn try_from(value: CallbackAttrInternal) -> Result<Self, Self::Error> {
    if value.responses.is_empty() {
      return Err(syn::Error::new(
        Span::call_site(),
        "Callback should define at least one response",
      ));
    }

    Ok(Self {
      deprecated: value.deprecated,
      summary: value.summary,
      description: value.description,
      components: value.components,
      responses: value.responses,
    })
  }
}

impl CallbackAttr {
  pub(crate) fn operation(&self) -> TokenStream {
    let components = &self.components;
    let deprecated = self
      .deprecated
      .map(|deprecated| quote!(Some(#deprecated)))
      .unwrap_or_else(|| quote!(None));
    let summary = match &self.summary {
      None => quote!(),
      Some(s) => {
        quote!(operation_builder.summary = Some(#s.to_string());)
      }
    };
    let description = match &self.description {
      None => quote!(),
      Some(d) => {
        quote!(operation_builder.description = Some(#d.to_string());)
      }
    };
    let mut responses = vec![];
    for response in &self.responses {
      let status = response.code;
      let (response_type_schema, response_type_raw_schema) = match &response.component {
        None => (quote!(None), quote!(None)),
        Some(response_type) => (
          quote!(<#response_type>::schema(oas_version)),
          quote!(<#response_type>::raw_schema(oas_version)),
        ),
      };
      responses.push(quote!({
        apistos::__internal::response_from_schema(oas_version, #status.to_string().as_str(), #response_type_schema)
            .or_else(|| apistos::__internal::response_from_raw_schema(oas_version, #status.to_string().as_str(), #response_type_raw_schema))
            .unwrap_or_else(|| apistos::__internal::response_for_status(#status.to_string().as_str()))
      }));
    }

    quote!(
      use apistos::ApiComponent;
      let mut operation_builder = apistos::paths::Operation::default();

      let mut body_requests: Vec<std::option::Option<apistos::paths::RequestBody>> = vec![];
      #(
        let mut request_body = <#components>::request_body(oas_version);
        body_requests.push(request_body);
      )*
      let body_requests = body_requests.into_iter().flatten().collect::<Vec<apistos::paths::RequestBody>>();
      for body_request in body_requests {
        operation_builder.request_body = Some(apistos::reference_or::ReferenceOr::Object(body_request));
      }

      let mut parameters: Vec<apistos::paths::Parameter> = vec![];
      #(
        parameters.append(&mut <#components>::parameters(oas_version));
      )*
      if !parameters.is_empty() {
        operation_builder.parameters = parameters.into_iter().map(apistos::reference_or::ReferenceOr::Object).collect();
      }

      let mut responses: std::collections::BTreeMap<String, apistos::reference_or::ReferenceOr<apistos::paths::Response>> = std::collections::BTreeMap::default();
      #(
        responses.append(&mut #responses.responses);
      )*
      let responses = apistos::paths::Responses {
        responses: responses,
        ..Default::default()
      };
      operation_builder.responses = responses;

      operation_builder.deprecated = #deprecated;

      #summary
      #description

      operation_builder
    )
  }

  pub(crate) fn components(&self) -> TokenStream {
    let components = &self.components;

    let responses_components: Vec<Type> = self.responses.iter().filter_map(|r| r.component.clone()).collect();

    quote!(
      use apistos::ApiComponent;
      let mut component_builder = apistos::components::Components::default();

      #(
        for (name, security) in <#components>::securities() {
          component_builder.security_schemes.insert(
            name, apistos::reference_or::ReferenceOr::Object(security)
          );
        }
      )*

      let mut schemas: Vec<Option<(String, apistos::reference_or::ReferenceOr<apistos::ApistosSchema>)>> = vec![];
      let mut schemas = if oas_version == apistos::OpenApiVersion::OAS3_0 {
        #(
          schemas.push(<#components>::schema(oas_version));
        )*
        #(
          schemas.push(<#responses_components>::schema(oas_version));
        )*
        schemas.into_iter().flatten().collect::<Vec<(String, apistos::reference_or::ReferenceOr<apistos::ApistosSchema>)>>()
      } else {
        vec![]
      };
      #(
        schemas.append(&mut <#components>::child_schemas(oas_version));
      )*
      #(
        schemas.append(&mut <#responses_components>::child_schemas(oas_version));
      )*
      component_builder.schemas = std::collections::BTreeMap::from_iter(schemas);
      vec![component_builder]
    )
  }
}

impl ToTokens for CallbackAttr {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let components = self.components();
    let operation = self.operation();
    tokens.extend(quote!(
      fn components(oas_version: apistos::OpenApiVersion) -> Vec<apistos::components::Components> {
        #components
      }

      fn operation(oas_version: apistos::OpenApiVersion) -> apistos::paths::Operation {
        #operation
      }
    ))
  }
}
