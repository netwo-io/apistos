use apistos_models::paths::{Operation, Parameter, ParameterDefinition, ParameterIn};
use apistos_models::reference_or::ReferenceOr;
use log::warn;
use once_cell::sync::Lazy;
use regex::Regex;
use schemars::schema::{Schema, SchemaObject, StringValidation};
use std::collections::HashSet;

/// Regex that can be used to fetch templated path parameters.
#[allow(clippy::expect_used)]
static PATH_TEMPLATE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{(.*?)\}").expect("path template regex"));

struct PathTemplateParameter {
  name: String,
  pattern: Option<String>,
}

pub(crate) trait OperationUpdater {
  fn update_path_parameter_name_from_path(&mut self, path: &str);
}

impl OperationUpdater for Operation {
  fn update_path_parameter_name_from_path(&mut self, path: &str) {
    let mut path_template_parameters = path_template_parameters(path);
    let path_parameters = self
      .parameters
      .iter()
      .filter_map(|p| match p {
        ReferenceOr::Reference { .. } => None,
        ReferenceOr::Object(p) => Some(p),
      })
      .filter(|p| p._in == ParameterIn::Path)
      .collect::<Vec<_>>();
    let should_match_by_name = !path_parameters.is_empty() && path_parameters.iter().all(|p| !p.name.is_empty());

    if should_match_by_name && path_template_parameters.len() == path_parameters.len() {
      check_path_parameter_name_mismatch(path, &path_template_parameters, &path_parameters);
    }

    for param in self
      .parameters
      .iter_mut()
      .filter_map(|p| p.get_object_mut())
      .filter(|p| p._in == ParameterIn::Path)
    {
      if should_match_by_name {
        if let Some(path_template_parameter) = path_template_parameters.iter().find(|p| p.name == param.name) {
          update_parameter_pattern(param, path_template_parameter);
        }
      } else if let Some(path_template_parameter) = path_template_parameters.pop() {
        param.name = path_template_parameter.name.clone();
        update_parameter_pattern(param, &path_template_parameter);
      } else {
        break;
      }
    }
  }
}

fn check_path_parameter_name_mismatch(
  path: &str,
  path_template_parameters: &[PathTemplateParameter],
  path_parameters: &[&Parameter],
) {
  let expected_names = path_template_parameters
    .iter()
    .map(|p| p.name.as_str())
    .collect::<HashSet<_>>();
  let generated_names = path_parameters.iter().map(|p| p.name.as_str()).collect::<HashSet<_>>();
  let mut missing_names = expected_names.difference(&generated_names).copied().collect::<Vec<_>>();
  let mut unexpected_names = generated_names.difference(&expected_names).copied().collect::<Vec<_>>();
  missing_names.sort_unstable();
  unexpected_names.sort_unstable();

  if !missing_names.is_empty() || !unexpected_names.is_empty() {
    warn!(
      "Path parameter names for '{path}' do not match generated parameter names. Missing from generated parameters: {missing_names:?}. Not present in path: {unexpected_names:?}.",
    );
  }
}

fn path_template_parameters(path: &str) -> Vec<PathTemplateParameter> {
  let mut parameters = PATH_TEMPLATE_REGEX
    .captures_iter(path)
    .filter_map(|captures| captures.get(1).map(|m| m.as_str()))
    .map(|value| {
      if let Some((name, pattern)) = value.split_once(':') {
        PathTemplateParameter {
          name: name.to_string(),
          pattern: Some(pattern.to_string()),
        }
      } else {
        PathTemplateParameter {
          name: value.to_string(),
          pattern: None,
        }
      }
    })
    .collect::<Vec<_>>();
  parameters.reverse();
  parameters
}

fn update_parameter_pattern(param: &mut Parameter, path_template_parameter: &PathTemplateParameter) {
  if let Some(pattern) = &path_template_parameter.pattern {
    param.definition = Some(ParameterDefinition::Schema(Box::new(ReferenceOr::Object(
      Schema::Object(SchemaObject {
        string: Some(Box::new(StringValidation {
          pattern: Some(pattern.clone()),
          ..Default::default()
        })),
        ..Default::default()
      }),
    ))))
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
  fn named_path_parameter_mismatch_preserves_generated_names() {
    let mut operation = Operation {
      parameters: vec![
        ReferenceOr::Object(Parameter {
          name: "user_id".to_string(),
          _in: ParameterIn::Path,
          ..Default::default()
        }),
        ReferenceOr::Object(Parameter {
          name: "article_id".to_string(),
          _in: ParameterIn::Path,
          ..Default::default()
        }),
      ],
      ..Default::default()
    };

    operation.update_path_parameter_name_from_path("/users/{user_id}/posts/{post_id}");

    let parameter_names = operation
      .parameters
      .iter()
      .filter_map(|p| p.clone().get_object())
      .map(|p| p.name)
      .collect::<Vec<_>>();

    assert_eq!(parameter_names, vec!["user_id".to_string(), "article_id".to_string()]);
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
        ParameterDefinition::Schema(sch) => match *sch {
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
