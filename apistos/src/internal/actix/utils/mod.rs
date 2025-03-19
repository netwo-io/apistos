use apistos_models::paths::{Operation, ParameterDefinition, ParameterIn};
use apistos_models::reference_or::ReferenceOr;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use schemars::schema::{Schema, SchemaObject, StringValidation};
use std::borrow::Cow;

/// Regex that can be used to fetch templated path parameters.
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
        if let Some((name, pattern)) = n.split_once(':') {
          param.name = name.to_string();
          param.definition = Some(ParameterDefinition::Schema(ReferenceOr::Object(Schema::Object(
            SchemaObject {
              string: Some(Box::new(StringValidation {
                pattern: Some(pattern.to_string()),
                ..Default::default()
              })),
              ..Default::default()
            },
          ))))
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
mod test {
  #![allow(clippy::panic)]

  use crate::internal::actix::utils::OperationUpdater;
  use apistos_models::Schema;
  use apistos_models::paths::{Operation, Parameter, ParameterDefinition, ParameterIn};
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

  #[test]
  fn regex_path_parameter_name_replacement() {
    let mut operation = Operation {
      parameters: vec![ReferenceOr::Object(Parameter {
        name: "".to_string(),
        _in: ParameterIn::Path,
        ..Default::default()
      })],
      ..Default::default()
    };

    operation.update_path_parameter_name_from_path("/test/{plop_id:.+}/plop");

    let first_parameter = operation.parameters.first().and_then(|p| p.clone().get_object());

    let first_parameter_name = first_parameter.clone().map(|p| p.name.clone()).unwrap_or_default();
    assert_eq!(first_parameter_name, "plop_id".to_string());

    if let Some(p) = first_parameter {
      let def = p.definition.clone().expect("missing parameter definition");
      match def {
        ParameterDefinition::Schema(sch) => match sch {
          ReferenceOr::Object(obj) => match obj {
            Schema::Bool(_) => panic!("expected schema object"),
            Schema::Object(obj) => {
              let str_obj = obj.string.expect("should be a string schema");
              assert_eq!(str_obj.pattern, Some(".+".to_string()));
            }
          },
          ReferenceOr::Reference { .. } => panic!("expected schema object"),
        },
        ParameterDefinition::Content(_) => panic!("expected schema"),
      }
    }
  }
}
