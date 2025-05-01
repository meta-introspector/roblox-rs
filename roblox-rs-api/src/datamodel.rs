use std::sync::Arc;
use crate::instance::{Instance, InstanceClass};
use crate::{RobloxError, RobloxResult};
use crate::services::*;

/// Represents the root of the Roblox instance hierarchy
#[derive(Debug)]
pub struct DataModel {
    root: Arc<Instance>,
}

impl DataModel {
    /// Create a new DataModel with default services
    pub fn new() -> Self {
        let root = Instance::new(InstanceClass::DataModel, "game");
        
        // Create default services
        let workspace = Instance::new(InstanceClass::Workspace, "Workspace");
        workspace.set_parent(Some(root.clone())).unwrap();
        
        let players = Instance::new(InstanceClass::Players, "Players");
        players.set_parent(Some(root.clone())).unwrap();
        
        let replicated_storage = Instance::new(InstanceClass::ReplicatedStorage, "ReplicatedStorage");
        replicated_storage.set_parent(Some(root.clone())).unwrap();
        
        let server_storage = Instance::new(InstanceClass::ServerStorage, "ServerStorage");
        server_storage.set_parent(Some(root.clone())).unwrap();
        
        let starter_gui = Instance::new(InstanceClass::StarterGui, "StarterGui");
        starter_gui.set_parent(Some(root.clone())).unwrap();
        
        Self { root }
    }
    
    /// Get the root instance
    pub fn get_root(&self) -> Arc<Instance> {
        self.root.clone()
    }
    
    /// Get a service by type
    pub fn get_service<T: Service>(&self) -> RobloxResult<Arc<Instance>> {
        let service_name = T::service_name();
        self.root.get_child(&service_name)
            .ok_or_else(|| RobloxError::InstanceNotFound(service_name))
    }
    
    /// Find the first instance that matches the given name
    pub fn find_first_child(&self, name: &str) -> Option<Arc<Instance>> {
        self.root.get_child(name)
    }
    
    /// Find all instances that match the given class name
    pub fn find_instances_of_class(&self, class: InstanceClass) -> Vec<Arc<Instance>> {
        let mut result = Vec::new();
        self.collect_instances_of_class(&self.root, class, &mut result);
        result
    }
    
    // Helper function to recursively collect instances of a given class
    fn collect_instances_of_class(
        &self,
        instance: &Arc<Instance>,
        class: InstanceClass,
        result: &mut Vec<Arc<Instance>>
    ) {
        if instance.get_class() == class {
            result.push(instance.clone());
        }
        
        for child in instance.get_children() {
            self.collect_instances_of_class(&child, class.clone(), result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_datamodel_creation() {
        let datamodel = DataModel::new();
        assert_eq!(datamodel.get_root().get_name(), "game");
        assert!(datamodel.get_service::<Workspace>().is_ok());
        assert!(datamodel.get_service::<Players>().is_ok());
    }
    
    #[test]
    fn test_find_instances() {
        let datamodel = DataModel::new();
        let workspace = datamodel.get_service::<Workspace>().unwrap();
        
        let part = Instance::new(InstanceClass::Part, "TestPart");
        part.set_parent(Some(workspace)).unwrap();
        
        let found = datamodel.find_instances_of_class(InstanceClass::Part);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].get_name(), "TestPart");
    }
} 