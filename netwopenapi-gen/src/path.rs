// use std::borrow::Cow;
// use std::ops::Deref;
// use std::{io::Error, str::FromStr};
//
// use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
// use proc_macro_error::abort;
// use quote::{format_ident, quote, quote_spanned, ToTokens};
// use syn::punctuated::Punctuated;
// use syn::spanned::Spanned;
// use syn::token::Paren;
// use syn::{parenthesized, parse::Parse, Token};
// use syn::{Expr, ExprLit, Lit, LitStr, Type};
//
// use crate::component::{GenericType, TypeTree};
// use crate::operation::OperationAttr;
// use crate::path::request_body::RequestBody;
// use crate::{parse_utils, Deprecated};
// use crate::{schema_type::SchemaType, security_requirement::SecurityRequirementAttr, Array};
//
// use self::response::Response;
// use self::{parameter::Parameter, request_body::RequestBodyAttr, response::Responses};
//
// pub struct Path {
//   operation_attr: OperationAttr,
//   fn_name: String,
//   path_operation: Option<PathOperation>,
//   path: Option<String>,
//   doc_comments: Option<Vec<String>>,
//   deprecated: Option<bool>,
// }
//
// impl Path {
//   pub fn new(operation_attr: OperationAttr, fn_name: &str) -> Self {
//     Self {
//       operation_attr,
//       fn_name: fn_name.to_string(),
//       path_operation: None,
//       path: None,
//       doc_comments: None,
//       deprecated: None,
//     }
//   }
//
//   pub fn path_operation(mut self, path_operation: Option<PathOperation>) -> Self {
//     self.path_operation = path_operation;
//
//     self
//   }
//
//   pub fn path(mut self, path_provider: impl FnOnce() -> Option<String>) -> Self {
//     self.path = path_provider();
//
//     self
//   }
//
//   pub fn doc_comments(mut self, doc_comments: Vec<String>) -> Self {
//     self.doc_comments = Some(doc_comments);
//
//     self
//   }
//
//   pub fn deprecated(mut self, deprecated: Option<bool>) -> Self {
//     self.deprecated = deprecated;
//
//     self
//   }
// }
//
// impl ToTokens for Path {
//   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//     let path_struct = format_ident!("{}{}", PATH_STRUCT_PREFIX, self.fn_name);
//     let operation_id = self
//       .path_attr
//       .operation_id
//       .clone()
//       .or(Some(
//         ExprLit {
//           attrs: vec![],
//           lit: Lit::Str(LitStr::new(&self.fn_name, Span::call_site())),
//         }
//         .into(),
//       ))
//       .unwrap_or_else(|| {
//         abort! {
//             Span::call_site(), "operation id is not defined for path";
//             help = r###"Try to define it in #[utoipa::path(operation_id = {})]"###, &self.fn_name;
//             help = "Did you define the #[utoipa::path(...)] over function?"
//         }
//       });
//     let tag = &*self.path_attr.tag.as_ref().map(ToOwned::to_owned).unwrap_or_default();
//     let path_operation = self
//       .path_attr
//       .path_operation
//       .as_ref()
//       .or(self.path_operation.as_ref())
//       .unwrap_or_else(|| {
//         #[cfg(any(feature = "actix_extras", feature = "rocket_extras"))]
//         let help = Some("Did you forget to define operation path attribute macro e.g #[get(...)]");
//
//         #[cfg(not(any(feature = "actix_extras", feature = "rocket_extras")))]
//         let help = None::<&str>;
//
//         abort! {
//             Span::call_site(), "path operation is not defined for path";
//             help = "Did you forget to define it in #[utoipa::path(get,...)]";
//             help =? help
//         }
//       });
//
//     let path = self.path_attr.path.as_ref().or(self.path.as_ref()).unwrap_or_else(|| {
//       #[cfg(any(feature = "actix_extras", feature = "rocket_extras"))]
//       let help = Some("Did you forget to define operation path attribute macro e.g #[get(...)]");
//
//       #[cfg(not(any(feature = "actix_extras", feature = "rocket_extras")))]
//       let help = None::<&str>;
//
//       abort! {
//           Span::call_site(), "path is not defined for path";
//           help = r###"Did you forget to define it in #[utoipa::path(path = "...")]"###;
//           help =? help
//       }
//     });
//
//     let path_with_context_path = self
//       .path_attr
//       .context_path
//       .as_ref()
//       .map(|context_path| format!("{context_path}{path}"))
//       .unwrap_or_else(|| path.to_string());
//
//     let operation: Operation = Operation {
//       deprecated: &self.deprecated,
//       operation_id,
//       summary: self.doc_comments.as_ref().and_then(|comments| comments.iter().next()),
//       description: self.doc_comments.as_ref(),
//       parameters: self.path_attr.params.as_ref(),
//       request_body: self.path_attr.request_body.as_ref(),
//       responses: self.path_attr.responses.as_ref(),
//       security: self.path_attr.security.as_ref(),
//     };
//
//     tokens.extend(quote! {
//         #[allow(non_camel_case_types)]
//         #[doc(hidden)]
//         pub struct #path_struct;
//
//         impl utoipa::Path for #path_struct {
//             fn path() -> &'static str {
//                 #path_with_context_path
//             }
//
//             fn path_item(default_tag: Option<&str>) -> utoipa::openapi::path::PathItem {
//                 use utoipa::openapi::ToArray;
//                 use std::iter::FromIterator;
//                 utoipa::openapi::PathItem::new(
//                     #path_operation,
//                     #operation.tag(*[Some(#tag), default_tag, Some("crate")].iter()
//                         .flatten()
//                         .find(|t| !t.is_empty()).unwrap()
//                     )
//                 )
//             }
//         }
//     });
//   }
// }
//
// // #[cfg_attr(feature = "debug", derive(Debug))]
// // struct Operation<'a> {
// //   operation_id: Expr,
// //   summary: Option<&'a String>,
// //   description: Option<&'a Vec<String>>,
// //   deprecated: &'a Option<bool>,
// //   parameters: &'a Vec<Parameter<'a>>,
// //   request_body: Option<&'a RequestBody<'a>>,
// //   responses: &'a Vec<Response<'a>>,
// //   security: Option<&'a Array<'a, SecurityRequirementAttr>>,
// // }
// //
// // impl ToTokens for Operation<'_> {
// //   fn to_tokens(&self, tokens: &mut TokenStream2) {
// //     tokens.extend(quote! { utoipa::openapi::path::OperationBuilder::new() });
// //
// //     if let Some(request_body) = self.request_body {
// //       tokens.extend(quote! {
// //           .request_body(Some(#request_body))
// //       })
// //     }
// //
// //     let responses = Responses(self.responses);
// //     tokens.extend(quote! {
// //         .responses(#responses)
// //     });
// //     if let Some(security_requirements) = self.security {
// //       tokens.extend(quote! {
// //           .securities(Some(#security_requirements))
// //       })
// //     }
// //     let operation_id = &self.operation_id;
// //     tokens.extend(quote_spanned! { operation_id.span() =>
// //         .operation_id(Some(#operation_id))
// //     });
// //
// //     if let Some(deprecated) = self.deprecated.map(Into::<Deprecated>::into) {
// //       tokens.extend(quote!( .deprecated(Some(#deprecated))))
// //     }
// //
// //     if let Some(summary) = self.summary {
// //       tokens.extend(quote! {
// //           .summary(Some(#summary))
// //       })
// //     }
// //
// //     if let Some(description) = self.description {
// //       let description = description.join("\n");
// //
// //       if !description.is_empty() {
// //         tokens.extend(quote! {
// //             .description(Some(#description))
// //         })
// //       }
// //     }
// //
// //     self.parameters.iter().for_each(|parameter| parameter.to_tokens(tokens));
// //   }
// // }
