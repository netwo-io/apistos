use syn::Attribute;

pub(crate) fn extract_deprecated_from_attr(attrs: &[Attribute]) -> Option<bool> {
  attrs.iter().find_map(|attr| {
    if !matches!(attr.path().get_ident(), Some(ident) if &*ident.to_string() == "deprecated") {
      None
    } else {
      Some(true)
    }
  })
}
