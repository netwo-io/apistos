use crate::internal::actix::resource::Resource;
use crate::internal::actix::route::RouteWrapper;
use crate::internal::actix::scope::Scope;
use crate::internal::actix::service_config::ServiceConfig;
use apistos_models::components::Components;
use apistos_models::paths::{Operation, OperationType, PathItem};
use indexmap::IndexMap;
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

impl DefinitionHolder for Resource {
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
impl<'a> DefinitionHolder for ServiceConfig<'a> {
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
