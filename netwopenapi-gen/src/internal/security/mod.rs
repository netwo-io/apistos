use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::BTreeMap;
use syn::Type;

pub mod models;

pub struct Security<'a> {
  pub args: &'a [Type],
  pub scopes: &'a BTreeMap<String, Vec<String>>,
}

impl<'a> ToTokens for Security<'a> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let args = self.args;
    let scopes = if self.scopes.is_empty() {
      quote!(
        Default::default();
      )
    } else {
      let mut scopes_tokens = quote!();
      for (scope_name, scopes) in self.scopes {
        scopes_tokens.extend(quote!(
          (#scope_name.to_string(), vec![#(#scopes.to_string(),)*]),
        ));
      }
      quote! {
        std::collections::BTreeMap::from_iter([
          #scopes_tokens
        ]);
      }
    };

    tokens.extend(quote! {
      let mut needs_empty_security = false;
      let mut securities = vec![];
      let needed_scopes: std::collections::BTreeMap<String, Vec<String>> = #scopes
      #(
        if !<#args>::required() {
          needs_empty_security = true;
        }
        let mut security_requirements = vec![];
        if let Some(security_requirement_name) = <#args>::security_requirement_name() {
          let scopes: Vec<String> = needed_scopes.get(&security_requirement_name).cloned().unwrap_or_default();
          security_requirements.push(netwopenapi::security::SecurityRequirement {
            requirements: std::collections::BTreeMap::from_iter(vec![(security_requirement_name, scopes)]),
            ..Default::default()
          });
        }
        securities.append(&mut security_requirements);
      )*
      if needs_empty_security {
        securities.push(netwopenapi::security::SecurityRequirement::default());
      }
      securities
    });
  }
}
