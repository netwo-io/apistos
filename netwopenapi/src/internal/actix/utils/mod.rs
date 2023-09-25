use netwopenapi_models::paths::{Operation, ParameterIn};
use netwopenapi_models::reference_or::ReferenceOr;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::borrow::Cow;

/// Regex that can be used for fetching templated path parameters.
#[allow(clippy::expect_used)]
static PATH_TEMPLATE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{(.*?)\}").expect("path template regex"));

pub(crate) trait OperationUpdater {
  fn update_path_parameter_name_from_path(&mut self, path: &str);
}

impl OperationUpdater for Operation {
  fn update_path_parameter_name_from_path(&mut self, path: &str) {
    let mut param_names = vec![];
    PATH_TEMPLATE_REGEX.replace_all(path, |c: &Captures| {
      param_names.push(c[1].to_owned());
      Into::<Cow<'static, str>>::into(":")
    });

    for param in self
      .parameters
      .iter_mut()
      .filter_map(|p| match p {
        ReferenceOr::Reference { .. } => None,
        ReferenceOr::Object(p) => Some(p),
      })
      .filter(|p| p._in == ParameterIn::Path)
    {
      if let Some(n) = param_names.pop() {
        if let Some((name, _pattern)) = n.split_once(':') {
          param.name = name.to_string();
        } else {
          param.name = n;
        }
      } else {
        break;
      }
    }
  }
}
