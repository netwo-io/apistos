use std::collections::BTreeMap;
use std::mem;
use utoipa::openapi::{Components, PathItem, PathItemType};
use utoipa::openapi::path::Operation;
use crate::internal::actix::resource::Resource;
use crate::internal::actix::route::RouteWrapper;
use crate::internal::actix::scope::Scope;
use crate::internal::actix::service_config::ServiceConfig;

pub trait DefinitionHolder {
  fn path(&self) -> &str;
  fn operations(&mut self) -> BTreeMap<PathItemType, Operation>;
  fn components(&mut self) -> BTreeMap<String, Components>;
  fn update_path_items(&mut self, path_op_map: &mut BTreeMap<String, PathItem>) -> () {
    let ops = self.operations();
    if !ops.is_empty() {
      let op_map = path_op_map.entry(self.path().into()).or_insert_with(Default::default);
      op_map.operations.extend(ops.into_iter());
    }
  }
}

impl DefinitionHolder for RouteWrapper {
  fn path(&self) -> &str {
    &self.def.path
  }

  fn operations(&mut self) -> BTreeMap<PathItemType, Operation> {
    mem::take(&mut self.def.item.operations)
  }

  fn components(&mut self) -> BTreeMap<String, Components> {
    mem::take(&mut self.component)
  }
}

impl DefinitionHolder for Resource {
  fn path(&self) -> &str {
    &self.path
  }

  fn operations(&mut self) -> BTreeMap<PathItemType, Operation> {
    mem::take(&mut self.item_definition).unwrap_or_default().operations
  }

  fn components(&mut self) -> BTreeMap<String, Components> {
    mem::take(&mut self.components)
  }
}

impl DefinitionHolder for Scope {
  fn path(&self) -> &str {
    unimplemented!("Scope has multiple paths");
  }

  fn operations(&mut self) -> BTreeMap<PathItemType, Operation> {
    unimplemented!("Scope has multiple operation maps");
  }

  fn components(&mut self) -> BTreeMap<String, Components> {
    mem::take(&mut self.components)
  }

  fn update_path_items(&mut self, path_op_map: &mut BTreeMap<String, PathItem>) -> () {
    for (path, item) in mem::take(&mut self.item_map) {
      let op_map = path_op_map.entry(path).or_insert_with(Default::default);
      op_map.operations.extend(item.operations.into_iter());
    }
  }
}

// weird, should not happen ?
impl<'a> DefinitionHolder for ServiceConfig<'a> {
  fn path(&self) -> &str {
    unimplemented!("ServiceConfig has multiple paths.");
  }

  fn operations(&mut self) -> BTreeMap<PathItemType, Operation> {
    unimplemented!("ServiceConfig has multiple operation maps.")
  }

  fn components(&mut self) -> BTreeMap<String, Components> {
    mem::take(&mut self.components)
  }

  fn update_path_items(&mut self, path_op_map: &mut BTreeMap<String, PathItem>) -> () {
    for (path, item) in mem::take(&mut self.item_map) {
      let op_map = path_op_map.entry(path).or_insert_with(Default::default);
      op_map.operations.extend(item.operations.into_iter());
    }
  }
}
