use syn::{Attribute, Data, Type};

pub(crate) fn extract_deprecated_from_attr(attrs: &[Attribute]) -> Option<bool> {
  attrs.iter().find_map(|attr| {
    if !matches!(attr.path().get_ident(), Some(ident) if &*ident.to_string() == "deprecated") {
      None
    } else {
      Some(true)
    }
  })
}

pub(crate) fn child_types_from_data(data: Data) -> Vec<Type> {
  match data {
    Data::Struct(s) => s.fields.into_iter().map(|f| f.ty).collect(),
    Data::Enum(e) => e
      .variants
      .into_iter()
      .map(|v| v.fields.into_iter().map(|f| f.ty))
      .flatten()
      .collect(),
    Data::Union(u) => u.fields.named.into_iter().map(|f| f.ty).collect(),
  }
}
