use crate::internal::components::Components;
use crate::internal::extract_fn_arguments_types;
use crate::internal::operation::Operation;
use crate::operation_attr::OperationAttr;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Expr, ImplGenerics, ItemFn, Lit, Meta, TypeGenerics, WhereClause};

pub(crate) struct PathItem<'a> {
  pub(crate) source: SourceDefinitionKind<'a>,
  pub(crate) openapi_struct: &'a Ident,
  pub(crate) item_ast: &'a ItemFn,
  pub(crate) operation_attribute: OperationAttr,
  pub(crate) responder_wrapper: &'a TokenStream,
}

pub(crate) enum SourceDefinitionKind<'a> {
  ActixMacros,
  Apistos {
    impl_generics: ImplGenerics<'a>,
    ty_generics: TypeGenerics<'a>,
    where_clause: Option<&'a WhereClause>,
    openapi_struct_def: &'a TokenStream,
  },
}

impl<'a> ToTokens for PathItem<'a> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let openapi_struct = self.openapi_struct.clone();
    let path_item_def_impl = if self.operation_attribute.skip {
      quote! {
        fn is_visible() -> bool {
          false
        }
      }
    } else {
      let args = extract_fn_arguments_types(self.item_ast, &self.operation_attribute.skip_args);
      let deprecated = self.item_ast.attrs.iter().find_map(|attr| {
        if !matches!(attr.path().get_ident(), Some(ident) if &*ident.to_string() == "deprecated") {
          None
        } else {
          Some(true)
        }
      });

      let doc_comments: Vec<String> = self
        .item_ast
        .attrs
        .iter()
        .filter(|attr| match attr.path().get_ident() {
          None => false,
          Some(attr) => attr == "doc",
        })
        .filter_map(|attr| match &attr.meta {
          Meta::NameValue(nv) => {
            if let Expr::Lit(ref doc_comment) = nv.value {
              if let Lit::Str(ref comment) = doc_comment.lit {
                Some(comment.value().trim().to_string())
              } else {
                None
              }
            } else {
              None
            }
          }
          Meta::Path(_) | Meta::List(_) => None,
        })
        .collect();
      let description = &*doc_comments
        .clone()
        .into_iter()
        .skip(1)
        .collect::<Vec<String>>()
        .join("\\\n");

      let operation = Operation {
        args: &args,
        responder_wrapper: self.responder_wrapper,
        operation_id: self.operation_attribute.operation_id.as_ref(),
        deprecated: Some(self.operation_attribute.deprecated || deprecated.unwrap_or_default()),
        summary: self
          .operation_attribute
          .summary
          .as_ref()
          .or_else(|| doc_comments.first()),
        description: self.operation_attribute.description.as_deref().or({
          if description.is_empty() {
            None
          } else {
            Some(description)
          }
        }),
        tags: &self.operation_attribute.tags,
        scopes: self.operation_attribute.scopes.clone(),
        callbacks: &self.operation_attribute.callbacks,
        error_codes: &self.operation_attribute.error_codes,
        consumes: self.operation_attribute.consumes.as_ref(),
        produces: self.operation_attribute.produces.as_ref(),
      };
      let components = Components {
        args: &args,
        responder_wrapper: self.responder_wrapper,
        error_codes: &self.operation_attribute.error_codes,
        callbacks: &self.operation_attribute.callbacks,
      };

      quote!(
        fn is_visible() -> bool {
          true
        }
        #operation
        #components
      )
    };

    match &self.source {
      SourceDefinitionKind::Apistos {
        impl_generics,
        ty_generics,
        where_clause,
        openapi_struct_def,
      } => {
        tokens.extend(quote! {
          #[expect(non_camel_case_types)]
          #[doc(hidden)]
          #openapi_struct_def

          #[automatically_derived]
          impl #impl_generics apistos::PathItemDefinition for #openapi_struct #ty_generics #where_clause {
            #path_item_def_impl
          }
        });
      }
      SourceDefinitionKind::ActixMacros => tokens.extend(quote! {
        #[automatically_derived]
        impl apistos::PathItemDefinition for #openapi_struct {
          #path_item_def_impl
        }
      }),
    }
  }
}
