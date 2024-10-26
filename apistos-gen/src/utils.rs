use syn::Path;
use crate::operation_attr::OperationType;

pub(crate) fn method_from_attr_path(path: &Path) -> Option<OperationType> {
  if path.is_ident("get") {
    return Some(OperationType::Get);
  } else if path.is_ident("put") {
    return Some(OperationType::Put)
  } else if path.is_ident("post") {
    return Some(OperationType::Post)
  } else if path.is_ident("delete") {
    return Some(OperationType::Delete)
  } else if path.is_ident("options") {
    return Some(OperationType::Options)
  } else if path.is_ident("head") {
    return Some(OperationType::Head)
  } else if path.is_ident("patch") {
    return Some(OperationType::Patch)
  } else if path.is_ident("trace") {
    return Some(OperationType::Trace)
  }
  None
}
