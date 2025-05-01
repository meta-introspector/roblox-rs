//! Component module for the ECS framework
//!
//! Components are data containers that can be attached to entities.

use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;

/// Trait for component types
///
/// Components must be Send + Sync + 'static for safe concurrent access.
pub trait Component: Send + Sync + 'static {}

// Blanket implementation for any type that meets the requirements
impl<T: Send + Sync + 'static> Component for T {}

/// Registry for component types
///
/// This is used to track registered component types and their metadata.
pub struct ComponentRegistry {
    components: HashMap<TypeId, ComponentInfo>,
}

/// Metadata about a registered component type
#[derive(Debug, Clone)]
pub struct ComponentInfo {
    /// The name of the component type
    pub name: String,
    
    /// The size of the component in bytes
    pub size: usize,
    
    /// Whether the component is a singleton (only one instance can exist)
    pub is_singleton: bool,
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentRegistry {
    /// Create a new component registry
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
    
    /// Register a component type
    pub fn register<T: Component + Debug>(&mut self) {
        let type_id = TypeId::of::<T>();
        
        if !self.components.contains_key(&type_id) {
            let info = ComponentInfo {
                name: std::any::type_name::<T>().to_string(),
                size: std::mem::size_of::<T>(),
                is_singleton: false,
            };
            
            self.components.insert(type_id, info);
        }
    }
    
    /// Register a component type as a singleton
    pub fn register_singleton<T: Component + Debug>(&mut self) {
        let type_id = TypeId::of::<T>();
        
        if !self.components.contains_key(&type_id) {
            let info = ComponentInfo {
                name: std::any::type_name::<T>().to_string(),
                size: std::mem::size_of::<T>(),
                is_singleton: true,
            };
            
            self.components.insert(type_id, info);
        }
    }
    
    /// Check if a component type is registered
    pub fn is_registered<T: Component>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }
    
    /// Get information about a registered component type
    pub fn get_info<T: Component>(&self) -> Option<&ComponentInfo> {
        self.components.get(&TypeId::of::<T>())
    }
    
    /// Check if a component type is registered as a singleton
    pub fn is_singleton<T: Component>(&self) -> bool {
        self.get_info::<T>()
            .map(|info| info.is_singleton)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn component_registration() {
        #[derive(Debug)]
        struct Position { x: f32, y: f32 }
        
        #[derive(Debug)]
        struct Singleton;
        
        let mut registry = ComponentRegistry::new();
        
        registry.register::<Position>();
        registry.register_singleton::<Singleton>();
        
        assert!(registry.is_registered::<Position>());
        assert!(registry.is_registered::<Singleton>());
        
        assert!(!registry.is_singleton::<Position>());
        assert!(registry.is_singleton::<Singleton>());
        
        let pos_info = registry.get_info::<Position>().unwrap();
        assert_eq!(pos_info.size, std::mem::size_of::<Position>());
        assert!(pos_info.name.contains("Position"));
    }
} 