use crate::internal::actix::redirect::Redirect;
use crate::internal::actix::resource::Resource;
use crate::internal::actix::route::RouteWrapper;
use crate::internal::actix::scope::Scope;
use crate::internal::actix::service_config::ServiceConfig;
use actix_web::http::StatusCode;
use apistos_models::components::Components;
use apistos_models::paths::{Header, MediaType, Operation, OperationType, ParameterDefinition, PathItem, Response};
use apistos_models::reference_or::ReferenceOr;
use apistos_models::{Schema, SchemaObject};
use indexmap::IndexMap;
use serde_json::Value;
use std::collections::BTreeMap;
use std::mem;

pub trait DefinitionHolder {
  fn path(&self) -> &str;
  fn operations(&mut self) -> IndexMap<OperationType, Operation>;
  fn components(&mut self) -> Vec<Components>;
  fn update_path_items(&mut self, path_op_map: &mut IndexMap<String, PathItem>) {
    let ops = self.operations();
    if !ops.is_empty() {
      let op_map = path_op_map.entry(self.path().into()).or_default();
      op_map.operations.extend(ops);
    }
  }
}

impl DefinitionHolder for RouteWrapper {
  fn path(&self) -> &str {
    &self.def.path
  }

  fn operations(&mut self) -> IndexMap<OperationType, Operation> {
    mem::take(&mut self.def.item.operations)
  }

  fn components(&mut self) -> Vec<Components> {
    mem::take(&mut self.component)
  }
}

impl<T> DefinitionHolder for Resource<T> {
  fn path(&self) -> &str {
    &self.path
  }

  fn operations(&mut self) -> IndexMap<OperationType, Operation> {
    mem::take(&mut self.item_definition).unwrap_or_default().operations
  }

  fn components(&mut self) -> Vec<Components> {
    mem::take(&mut self.components)
  }
}

#[allow(clippy::unimplemented)]
impl<T> DefinitionHolder for Scope<T> {
  fn path(&self) -> &str {
    unimplemented!("Scope has multiple paths");
  }

  fn operations(&mut self) -> IndexMap<OperationType, Operation> {
    unimplemented!("Scope has multiple operation maps");
  }

  fn components(&mut self) -> Vec<Components> {
    mem::take(&mut self.components)
  }

  fn update_path_items(&mut self, path_op_map: &mut IndexMap<String, PathItem>) {
    for (path, item) in mem::take(&mut self.item_map) {
      let op_map = path_op_map.entry(path).or_default();
      op_map.operations.extend(item.operations.into_iter());
    }
  }
}

#[allow(clippy::unimplemented)]
impl DefinitionHolder for ServiceConfig<'_> {
  fn path(&self) -> &str {
    unimplemented!("ServiceConfig has multiple paths.");
  }

  fn operations(&mut self) -> IndexMap<OperationType, Operation> {
    unimplemented!("ServiceConfig has multiple operation maps.")
  }

  fn components(&mut self) -> Vec<Components> {
    mem::take(&mut self.components)
  }

  fn update_path_items(&mut self, path_op_map: &mut IndexMap<String, PathItem>) {
    for (path, item) in mem::take(&mut self.item_map) {
      let op_map = path_op_map.entry(path).or_default();
      op_map.operations.extend(item.operations.into_iter());
    }
  }
}

impl DefinitionHolder for Redirect {
  fn path(&self) -> &str {
    &self.path
  }

  fn operations(&mut self) -> IndexMap<OperationType, Operation> {
    let mut index_map: IndexMap<OperationType, Operation> = IndexMap::new();
    match self.code {
      StatusCode::TEMPORARY_REDIRECT | StatusCode::PERMANENT_REDIRECT => {
        let _unused = index_map.insert(OperationType::Get, Operation { ..Default::default() });
        let _unused = index_map.insert(OperationType::Delete, Operation { ..Default::default() });
        let _unused = index_map.insert(OperationType::Head, Operation { ..Default::default() });
        let _unused = index_map.insert(OperationType::Options, Operation { ..Default::default() });
        let _unused = index_map.insert(OperationType::Patch, Operation { ..Default::default() });
        let _unused = index_map.insert(OperationType::Post, Operation { ..Default::default() });
        let _unused = index_map.insert(OperationType::Put, Operation { ..Default::default() });
        let _unused = index_map.insert(OperationType::Trace, Operation { ..Default::default() });
      }
      StatusCode::SEE_OTHER => {
        let _unused = index_map.insert(OperationType::Get, Operation { ..Default::default() });
      }
      _ => {}
    }
    index_map
  }

  // https://github.com/OAI/OpenAPI-Specification/issues/2512
  fn components(&mut self) -> Vec<Components> {
    let location_header = Header {
      definition: Some(ParameterDefinition::Content(BTreeMap::from_iter(vec![(
        "text/plain".to_string(),
        MediaType {
          schema: Some(ReferenceOr::Object(Schema::Object(SchemaObject {
            enum_values: Some(vec![Value::String(self.redirect.clone())]),
            ..Default::default()
          }))),
          ..Default::default()
        },
      )]))),
      ..Default::default()
    };

    let response = Response {
      description: "Redirect".to_owned(),
      headers: BTreeMap::from_iter(vec![("Location".to_string(), ReferenceOr::Object(location_header))]),
      ..Default::default()
    };

    vec![Components {
      responses: BTreeMap::from_iter(vec![(self.code.as_u16().to_string(), ReferenceOr::Object(response))]),
      ..Default::default()
    }]
  }
}
