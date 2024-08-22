use darling::FromMeta;
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::{Attribute, ImplGenerics, TypeGenerics, WhereClause};

pub(crate) fn parse_openapi_type_attrs(
  attrs: &[Attribute],
  impl_generics: &ImplGenerics,
  ident: &Ident,
  ty_generics: &TypeGenerics,
  where_clause: Option<&WhereClause>,
) -> Option<ApiTypeDeclaration> {
  let type_declarations_res = attrs
    .iter()
    .filter(|attribute| attribute.path().is_ident("openapi_type"))
    .map(|attribute| ApiTypeDeclarationInternal::from_meta(&attribute.meta))
    .collect::<darling::Result<Vec<ApiTypeDeclarationInternal>>>();

  match type_declarations_res {
    Ok(type_declarations) if type_declarations.len() > 1 => {
      abort!(Span::call_site(), "Expected only one #[openapi_type] attribute")
    }
    Ok(type_declarations) => {
      let type_declaration = type_declarations.first().cloned();
      type_declaration.map(|s| ApiTypeDeclaration {
        impl_generics: quote!(#impl_generics),
        generics_ident: quote!(#ident #ty_generics #where_clause),
        _type: s._type,
        format: s.format,
      })
    }
    Err(e) => abort!(e.span(), "Unable to parse #[openapi_type] attribute: {:?}", e),
  }
}

#[derive(FromMeta, Clone)]
struct ApiTypeDeclarationInternal {
  #[darling(rename = "schema_type")]
  _type: String,
  format: Option<String>,
}

#[derive(Clone)]
pub(crate) struct ApiTypeDeclaration {
  pub(crate) impl_generics: TokenStream,
  pub(crate) generics_ident: TokenStream,
  pub(crate) _type: String,
  pub(crate) format: Option<String>,
}

impl ToTokens for ApiTypeDeclaration {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let impl_generics = self.impl_generics.clone();
    let generics_ident = self.generics_ident.clone();
    let _type = &self._type;
    let format = if let Some(format) = &self.format {
      quote!(Some(#format.to_string()))
    } else {
      quote!(None)
    };

    tokens.extend(quote! {
      #[automatically_derived]
      impl #impl_generics apistos::TypedSchema for #generics_ident {
        fn schema_type() -> String {
          #_type.to_string()
        }

        fn format() -> Option<String> {
          #format
        }
      }
    })
  }
}
