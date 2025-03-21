use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::collections::BTreeMap;
use syn::Type;

pub(crate) mod models;

pub(crate) struct Security<'a> {
  pub(crate) args: &'a [Type],
  pub(crate) scopes: &'a BTreeMap<String, Vec<String>>,
}

impl ToTokens for Security<'_> {
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
          security_requirements.push(apistos::security::SecurityRequirement {
            requirements: std::collections::BTreeMap::from_iter(vec![(security_requirement_name, scopes)]),
          });
        }
        securities.append(&mut security_requirements);
      )*
      if needs_empty_security {
        securities.push(apistos::security::SecurityRequirement::default());
      }
      securities
    });
  }
}
