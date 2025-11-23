use crate::actix_operation_attr::ActixOperationAttrInternal;
use crate::actix_route_attr::ActixRouteAttrInternal;
use crate::operation_attr::OperationType;
use darling::FromMeta;
use proc_macro_error2::abort;
use quote::{ToTokens, quote};
use syn::{Attribute, Expr, Meta, MetaList, Path};

pub(crate) fn method_from_attr_path(path: &Path) -> Option<OperationType> {
  if path.is_ident("get") {
    return Some(OperationType::Get);
  } else if path.is_ident("put") {
    return Some(OperationType::Put);
  } else if path.is_ident("post") {
    return Some(OperationType::Post);
  } else if path.is_ident("delete") {
    return Some(OperationType::Delete);
  } else if path.is_ident("options") {
    return Some(OperationType::Options);
  } else if path.is_ident("head") {
    return Some(OperationType::Head);
  } else if path.is_ident("patch") {
    return Some(OperationType::Patch);
  } else if path.is_ident("trace") {
    return Some(OperationType::Trace);
  } else if path.is_ident("connect") {
    return Some(OperationType::Connect);
  }
  None
}

pub(crate) fn modify_attribute_with_scope(attr: &Attribute, scope_path: &str) -> Attribute {
  let attr_path = attr.path();

  if attr_path.is_ident("route") {
    let attr_meta_list = match attr.meta.require_list() {
      Ok(v) => v,
      Err(e) => abort!(e.span(), "Expected attr meta list"),
    };
    let route_attr = ActixRouteAttrInternal::from_meta(&attr.meta);
    match route_attr {
      Ok(route) => {
        let ActixRouteAttrInternal {
          path,
          methods,
          name,
          guard,
          wrap,
        } = route;
        let modified_path = format!("{}{}", scope_path, path);
        let name = name.map(|n| quote!(name = #n, )).unwrap_or_default();
        let guard = guard.map(|g| quote!(guard = #g, )).unwrap_or_default();
        let wrap = wrap.map(|w| quote!(wrap = #w, )).unwrap_or_default();
        Attribute {
          meta: Meta::List(MetaList {
            tokens: quote!( path = #modified_path, #(#methods, )* #name #guard #wrap ),
            ..attr_meta_list.clone()
          }),
          ..attr.clone()
        }
      }
      Err(_) => attr.clone(),
    }
  } else {
    let method_attr = ActixOperationAttrInternal::from_meta(&attr.meta);
    match method_attr {
      Ok(method) => {
        let attr_meta_list = match attr.meta.require_list() {
          Ok(v) => v,
          Err(e) => abort!(e.span(), "Expected attr meta list"),
        };
        let ActixOperationAttrInternal {
          path,
          name,
          guard,
          wrap,
          operation,
        } = method;
        let modified_path = format!("{}{}", scope_path, path);
        let name = name.map(|n| quote!(name = #n, )).unwrap_or_default();
        let guard = guard.map(|g| quote!(guard = #g, )).unwrap_or_default();
        let wrap = wrap.map(|w| quote!(wrap = #w, )).unwrap_or_default();
        Attribute {
          meta: Meta::List(MetaList {
            tokens: quote!( path = #modified_path, #operation #name #guard #wrap ),
            ..attr_meta_list.clone()
          }),
          ..attr.clone()
        }
      }
      Err(_) => attr.clone(),
    }
  }
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
