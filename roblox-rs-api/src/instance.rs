use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::RobloxResult;
use crate::{RobloxError, events::*};

/// Represents a Roblox instance class name
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstanceClass {
    DataModel,
    Part,
    Model,
    Folder,
    Script,
    LocalScript,
    ModuleScript,
    Workspace,
    Players,
    ReplicatedStorage,
    ServerStorage,
    StarterGui,
    // Add more as needed
}

/// Represents a property value in a Roblox instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Bool(bool),
    Vector3 { x: f64, y: f64, z: f64 },
    CFrame { position: [f64; 3], orientation: [f64; 9] },
    Color3 { r: f64, g: f64, b: f64 },
    #[serde(skip)]
    Instance(Arc<Instance>),
    // Add more as needed
}

type EventHandler = Box<dyn Fn(&PropertyValue) + Send + Sync + 'static>;

/// Represents a Roblox instance
#[derive(Clone)]
pub struct Instance {
    /// The class name of this instance
    class: InstanceClass,
    /// The name of this instance
    name: Arc<RwLock<String>>,
    /// The parent of this instance
    parent: Arc<RwLock<Option<Arc<Instance>>>>,
    /// The children of this instance
    children: Arc<RwLock<Vec<Arc<Instance>>>>,
    /// The properties of this instance
    properties: Arc<RwLock<HashMap<String, PropertyValue>>>,
    /// Event handlers for this instance
    #[allow(clippy::type_complexity)]
    events: Arc<RwLock<HashMap<String, Vec<EventHandler>>>>,
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instance")
            .field("class", &self.class)
            .field("name", &self.name)
            .field("parent", &self.parent)
            .field("children", &self.children)
            .field("properties", &self.properties)
            .field("events", &format!("<{} handlers>", 
                self.events.read().unwrap().values().map(|v| v.len()).sum::<usize>()))
            .finish()
    }
}

impl Instance {
    /// Create a new instance with the given class and name
    pub fn new(class: InstanceClass, name: impl Into<String>) -> Arc<Self> {
        Arc::new(Self {
            class,
            name: Arc::new(RwLock::new(name.into())),
            parent: Arc::new(RwLock::new(None)),
            children: Arc::new(RwLock::new(Vec::new())),
            properties: Arc::new(RwLock::new(HashMap::new())),
            events: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Get the class of this instance
    pub fn get_class(&self) -> InstanceClass {
        self.class.clone()
    }

    /// Get the name of this instance
    pub fn get_name(&self) -> String {
        self.name.read().unwrap().clone()
    }

    /// Set the name of this instance
    pub fn set_name(&self, name: impl Into<String>) {
        *self.name.write().unwrap() = name.into();
    }

    /// Get the parent of this instance
    pub fn get_parent(&self) -> Option<Arc<Instance>> {
        self.parent.read().unwrap().clone()
    }

    /// Set the parent of this instance
    pub fn set_parent(&self, parent: Option<Arc<Instance>>) -> RobloxResult<()> {
        // Remove from old parent
        if let Some(old_parent) = self.get_parent() {
            let mut children = old_parent.children.write().unwrap();
            children.retain(|child| !Arc::ptr_eq(child, &Arc::new(self.clone())));
        }

        // Add to new parent
        if let Some(new_parent) = parent.clone() {
            new_parent.children.write().unwrap().push(Arc::new(self.clone()));
        }

        *self.parent.write().unwrap() = parent;
        Ok(())
    }

    /// Get a child by name
    pub fn get_child(&self, name: &str) -> Option<Arc<Instance>> {
        self.children.read().unwrap()
            .iter()
            .find(|child| child.get_name() == name)
            .cloned()
    }

    /// Get all children
    pub fn get_children(&self) -> Vec<Arc<Instance>> {
        self.children.read().unwrap().clone()
    }

    /// Set a property value
    pub fn set_property(&self, name: &str, value: PropertyValue) -> RobloxResult<()> {
        let mut properties = self.properties.write().unwrap();
        properties.insert(name.to_string(), value.clone());
        
        // Trigger property changed event
        if let Some(handlers) = self.events.read().unwrap().get("PropertyChanged") {
            for handler in handlers {
                handler(&value);
            }
        }
        
        Ok(())
    }

    /// Get a property value
    pub fn get_property(&self, name: &str) -> Option<PropertyValue> {
        self.properties.read().unwrap().get(name).cloned()
    }

    /// Connect a function to an event
    pub fn connect<F>(&self, event_name: &str, callback: F)
    where
        F: Fn(&PropertyValue) + Send + Sync + 'static,
    {
        let mut events = self.events.write().unwrap();
        events.entry(event_name.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_creation() {
        let instance = Instance::new(InstanceClass::Part, "TestPart");
        assert_eq!(instance.get_name(), "TestPart");
        assert_eq!(instance.get_class(), InstanceClass::Part);
    }

    #[test]
    fn test_parent_child_relationship() {
        let parent = Instance::new(InstanceClass::Model, "Parent");
        let child = Instance::new(InstanceClass::Part, "Child");
        
        child.set_parent(Some(parent.clone())).unwrap();
        assert!(Arc::ptr_eq(&child.get_parent().unwrap(), &parent));
        assert_eq!(parent.get_child("Child").unwrap().get_name(), "Child");
    }

    #[test]
    fn test_properties() {
        let instance = Instance::new(InstanceClass::Part, "TestPart");
        instance.set_property("Size", PropertyValue::Vector3 { x: 1.0, y: 2.0, z: 3.0 }).unwrap();
        
        if let PropertyValue::Vector3 { x, y, z } = instance.get_property("Size").unwrap() {
            assert_eq!(x, 1.0);
            assert_eq!(y, 2.0);
            assert_eq!(z, 3.0);
        } else {
            panic!("Wrong property type");
        }
    }
} 