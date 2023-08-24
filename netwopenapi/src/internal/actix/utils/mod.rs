use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::borrow::Cow;
use utoipa::openapi::path::{Operation, ParameterIn};

/// Regex that can be used for fetching templated path parameters.
static PATH_TEMPLATE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{(.*?)\}").expect("path template regex"));

pub trait OperationUpdater {
  fn update_path_parameter_name_from_path(&mut self, path: &str);
}

impl OperationUpdater for Operation {
  fn update_path_parameter_name_from_path(&mut self, path: &str) {
    self.parameters.as_mut().map(|params| {
      let mut param_names = vec![];
      PATH_TEMPLATE_REGEX.replace_all(path, |c: &Captures| {
        param_names.push(c[1].to_owned());
        Into::<Cow<'static, str>>::into(":")
      });

      for param in params.iter_mut().filter(|p| p.parameter_in == ParameterIn::Path) {
        if let Some(n) = param_names.pop() {
          if let Some((name, _pattern)) = n.split_once(':') {
            param.name = name.to_string();
            //@todo done for type format to handle regex. we need to thing about that one
            // match param.schema {
            //   None => {}
            //   Some(RefOr::T(t)) if matches!(t,) => t,
            // }
            // param.pattern = Some(pattern.to_string());
          } else {
            param.name = n;
          }
        } else {
          break;
        }
      }
    });
  }
}
