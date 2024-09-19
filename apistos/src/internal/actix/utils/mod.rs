use std::borrow::Cow;

use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use serde_json::json;

use apistos_models::paths::{Operation, ParameterDefinition, ParameterIn};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::{ApistosSchema, Schema};

use crate::internal::get_oas_version;

/// Regex that can be used to fetch templated path parameters.
#[expect(clippy::expect_used)]
static PATH_TEMPLATE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{(.*?)\}").expect("path template regex"));

pub(crate) trait OperationUpdater {
  fn update_path_parameter_name_from_path(&mut self, path: &str);
}

impl OperationUpdater for Operation {
  fn update_path_parameter_name_from_path(&mut self, path: &str) {
    let oas_version = get_oas_version();

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

          let parameter_definition = Schema::try_from(json!({
            "type": "string",
            "pattern": pattern
          }))
          .map_err(|err| {
            log::warn!("Error generating json schema: {err:?}");
            err
          })
          .map(|sch| ApistosSchema::new(sch, oas_version))
          .unwrap_or_default();
          param.definition = Some(ParameterDefinition::Schema(ReferenceOr::Object(parameter_definition)))
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
  #![expect(clippy::panic)]

  use apistos_models::paths::{Operation, Parameter, ParameterDefinition, ParameterIn};
  use apistos_models::reference_or::ReferenceOr;

  use crate::internal::actix::utils::OperationUpdater;

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
          ReferenceOr::Object(obj) => {
            let pattern = obj
              .inner()
              .as_object()
              .and_then(|obj| obj.get("pattern"))
              .and_then(|_type| _type.as_str());
            assert_eq!(pattern, Some(".+"));
          }
          ReferenceOr::Reference { .. } => panic!("expected schema object"),
        },
        ParameterDefinition::Content(_) => panic!("expected schema"),
      }
    }
  }
}
