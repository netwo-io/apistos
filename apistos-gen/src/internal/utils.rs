use darling::FromMeta;
use quote::ToTokens;
use syn::{Attribute, Expr, Meta, MetaList};

pub(crate) fn extract_deprecated_from_attr(attrs: &[Attribute]) -> Option<bool> {
  attrs.iter().find_map(|attr| {
    if !matches!(attr.path().get_ident(), Some(ident) if &*ident.to_string() == "deprecated") {
      None
    } else {
      Some(true)
    }
  })
}

pub(crate) fn from_meta_inner_flat<T: FromMeta>(item: &Meta, property_name: &str) -> darling::Result<Vec<T>> {
  let mut acc = Vec::new();

  let expr = Expr::from_meta(item)?;
  if let Expr::Array(arr) = &expr {
    for elem in &arr.elems {
      if let Expr::Call(call) = elem {
        if let Expr::Path(path) = &*call.func {
          if path.path.is_ident(property_name) {
            let mut tokens = proc_macro2::TokenStream::new();
            for (i, arg) in call.args.iter().enumerate() {
              if i > 0 {
                tokens.extend(quote::quote! { , });
              }
              arg.to_tokens(&mut tokens);
            }

            let meta_list = Meta::List(MetaList {
              path: path.path.clone(),
              delimiter: syn::MacroDelimiter::Paren(Default::default()),
              tokens,
            });

            let nested = darling::ast::NestedMeta::Meta(meta_list);
            acc.push(T::from_nested_meta(&nested)?);
          }
        }
      }
    }
  }

  Ok(acc)
}

pub(crate) fn from_list_inner<T: FromMeta>(
  items: &[darling::ast::NestedMeta],
  property_name: &str,
  struct_name: &str,
) -> darling::Result<Vec<T>> {
  let mut acc = Vec::new();

  for item in items {
    if let darling::ast::NestedMeta::Meta(Meta::NameValue(nv)) = item {
      if nv.path.is_ident(property_name) {
        if let Expr::Array(arr) = &nv.value {
          for elem in &arr.elems {
            if let Expr::Call(call) = elem {
              if let Expr::Path(path) = &*call.func {
                if path.path.is_ident(struct_name) {
                  let mut tokens = proc_macro2::TokenStream::new();
                  for (i, arg) in call.args.iter().enumerate() {
                    if i > 0 {
                      tokens.extend(quote::quote! { , });
                    }
                    arg.to_tokens(&mut tokens);
                  }

                  let meta_list = Meta::List(MetaList {
                    path: path.path.clone(),
                    delimiter: syn::MacroDelimiter::Paren(Default::default()),
                    tokens,
                  });

                  let nested = darling::ast::NestedMeta::Meta(meta_list);
                  acc.push(T::from_nested_meta(&nested)?);
                }
              }
            }
          }
        }
      }
    }
  }

  Ok(acc)
}
