use apistos_models::paths::{Operation, ParameterIn};
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
    param_names.reverse();

    for param in self
      .parameters
      .iter_mut()
      .filter_map(|p| p.get_object_mut())
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

#[cfg(test)]
mod tests {
  use crate::internal::actix::utils::OperationUpdater;
  use apistos_models::paths::{Operation, Parameter, ParameterIn};
  use apistos_models::reference_or::ReferenceOr;

  #[test]
  fn simple_path_parameter_name_replacement() {
    let mut operation = Operation {
      parameters: vec![ReferenceOr::Object(Parameter {
        name: "".to_string(),
        _in: ParameterIn::Path,
        ..Default::default()
      })],
      ..Default::default()
    };

    operation.update_path_parameter_name_from_path("/test/{plop_id}/plop");

    let first_parameter_name = operation
      .parameters
      .first()
      .and_then(|p| p.clone().get_object())
      .map(|p| p.name.clone())
      .unwrap_or_default();
    assert_eq!(first_parameter_name, "plop_id".to_string());
  }

  #[test]
  fn multiple_path_parameter_name_replacement() {
    let mut operation = Operation {
      parameters: vec![
        ReferenceOr::Object(Parameter {
          name: "".to_string(),
          _in: ParameterIn::Path,
          ..Default::default()
        }),
        ReferenceOr::Object(Parameter {
          name: "".to_string(),
          _in: ParameterIn::Path,
          ..Default::default()
        }),
      ],
      ..Default::default()
    };

    operation.update_path_parameter_name_from_path("/test/{plop_id}/plop/{clap_id}");

    let first_parameter_name = operation
      .parameters
      .first()
      .and_then(|p| p.clone().get_object())
      .map(|p| p.name.clone())
      .unwrap_or_default();
    let second_parameter_name = operation
      .parameters
      .last()
      .and_then(|p| p.clone().get_object())
      .map(|p| p.name.clone())
      .unwrap_or_default();
    assert_eq!(first_parameter_name, "plop_id".to_string());
    assert_eq!(second_parameter_name, "clap_id".to_string());
  }
}
