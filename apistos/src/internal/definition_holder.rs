use crate::internal::actix::resource::Resource;
use crate::internal::actix::route::RouteWrapper;
use crate::internal::actix::scope::Scope;
use crate::internal::actix::service_config::ServiceConfig;
use actix_web::http::StatusCode;
use apistos_models::OpenApiVersion;
use apistos_models::components::Components;
use apistos_models::paths::{Operation, OperationType, PathItem, Responses};
use apistos_models::reference_or::ReferenceOr;
use indexmap::IndexMap;
use std::collections::BTreeMap;
use std::mem;

use super::actix::METHODS;
use super::actix::redirect::Redirect;

pub trait DefinitionHolder {
  fn operations(&mut self, _oas_version: OpenApiVersion) -> IndexMap<String, IndexMap<OperationType, Operation>>;
  fn components(&mut self, _oas_version: OpenApiVersion) -> Vec<Components>;
  fn update_path_items(&mut self, oas_version: OpenApiVersion, path_op_map: &mut IndexMap<String, PathItem>) {
    let ops = self.operations(oas_version);
    if !ops.is_empty() {
      for (path, ops) in ops {
        path_op_map.entry(path).or_default().operations.extend(ops);
      }
    }
  }
}

impl DefinitionHolder for RouteWrapper {
  fn operations(&mut self, _oas_version: OpenApiVersion) -> IndexMap<String, IndexMap<OperationType, Operation>> {
    let operations_map = mem::take(&mut self.def.item.operations);
    IndexMap::from_iter(vec![(self.def.path.clone(), operations_map)])
  }

  fn components(&mut self, _oas_version: OpenApiVersion) -> Vec<Components> {
    mem::take(&mut self.component)
  }
}

impl<T> DefinitionHolder for Resource<T> {
  fn operations(&mut self, _oas_version: OpenApiVersion) -> IndexMap<String, IndexMap<OperationType, Operation>> {
    let operations_map = mem::take(&mut self.item_definition).unwrap_or_default().operations;
    IndexMap::from_iter(vec![(self.path.clone(), operations_map)])
  }

  fn components(&mut self, _oas_version: OpenApiVersion) -> Vec<Components> {
    mem::take(&mut self.components)
  }
}

impl<T> DefinitionHolder for Scope<T> {
  fn operations(&mut self, _oas_version: OpenApiVersion) -> IndexMap<String, IndexMap<OperationType, Operation>> {
    let item_map = mem::take(&mut self.item_map);
    item_map
      .into_iter()
      .map(|(path, operations)| (path, operations.operations))
      .collect()
  }

  fn components(&mut self, _oas_version: OpenApiVersion) -> Vec<Components> {
    mem::take(&mut self.components)
  }
}

impl DefinitionHolder for ServiceConfig<'_> {
  fn operations(&mut self, _oas_version: OpenApiVersion) -> IndexMap<String, IndexMap<OperationType, Operation>> {
    let item_map = mem::take(&mut self.item_map);
    item_map
      .into_iter()
      .map(|(path, operations)| (path, operations.operations))
      .collect()
  }

  fn components(&mut self, _oas_version: OpenApiVersion) -> Vec<Components> {
    mem::take(&mut self.components)
  }
}

impl DefinitionHolder for Redirect {
  fn operations(&mut self, oas_version: OpenApiVersion) -> IndexMap<OperationType, Operation> {
    let mut index_map: IndexMap<OperationType, Operation> = IndexMap::new();
    let methods = match self.code {
      StatusCode::TEMPORARY_REDIRECT | StatusCode::PERMANENT_REDIRECT => METHODS,
      StatusCode::SEE_OTHER => &[OperationType::Get],
      _ => &[],
    };

    if !methods.is_empty() {
      let response = self.get_open_api_response(oas_version);
      for method in methods {
        index_map.insert(
          *method,
          Operation {
            responses: Responses {
              responses: BTreeMap::from_iter(vec![(
                self.code.as_u16().to_string(),
                ReferenceOr::Object(response.clone()),
              )]),
              ..Default::default()
            },
            ..Default::default()
          },
        );
      }
    }
    index_map
  }

  fn components(&mut self, _oas_version: OpenApiVersion) -> Vec<Components> {
    vec![]
  }
}
