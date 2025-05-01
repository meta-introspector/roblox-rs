use std::sync::Arc;
use async_trait::async_trait;
use crate::instance::{Instance, InstanceClass, PropertyValue};
use crate::RobloxResult;

/// Base trait for all Roblox services
pub trait Service {
    /// Get the name of this service
    fn service_name() -> String;
}

/// The Workspace service
pub struct Workspace;

impl Service for Workspace {
    fn service_name() -> String {
        "Workspace".to_string()
    }
}

/// The Players service
pub struct Players;

impl Service for Players {
    fn service_name() -> String {
        "Players".to_string()
    }
}

/// The ReplicatedStorage service
pub struct ReplicatedStorage;

impl Service for ReplicatedStorage {
    fn service_name() -> String {
        "ReplicatedStorage".to_string()
    }
}

/// The ServerStorage service
pub struct ServerStorage;

impl Service for ServerStorage {
    fn service_name() -> String {
        "ServerStorage".to_string()
    }
}

/// The StarterGui service
pub struct StarterGui;

impl Service for StarterGui {
    fn service_name() -> String {
        "StarterGui".to_string()
    }
}

/// Trait for services that can run scripts
#[async_trait]
pub trait ScriptService {
    /// Run a script in this service
    async fn run_script(&self, source: &str) -> RobloxResult<()>;
}

/// Trait for services that can handle physics
pub trait PhysicsService {
    /// Set the gravity vector for the world
    fn set_gravity(&self, gravity: f64) -> RobloxResult<()>;
    
    /// Get the current gravity vector
    fn get_gravity(&self) -> RobloxResult<f64>;
    
    /// Cast a ray from origin in direction
    fn raycast(&self, origin: [f64; 3], direction: [f64; 3]) -> RobloxResult<Option<Arc<Instance>>>;
}

/// Implementation of physics methods for Workspace
impl PhysicsService for Arc<Instance> {
    fn set_gravity(&self, gravity: f64) -> RobloxResult<()> {
        self.set_property("Gravity", PropertyValue::Number(gravity))
    }
    
    fn get_gravity(&self) -> RobloxResult<f64> {
        match self.get_property("Gravity") {
            Some(PropertyValue::Number(g)) => Ok(g),
            _ => Ok(196.2) // Default Roblox gravity
        }
    }
    
    fn raycast(&self, _origin: [f64; 3], _direction: [f64; 3]) -> RobloxResult<Option<Arc<Instance>>> {
        // Simple implementation that just returns the first part it finds in the direction
        // In a real implementation, this would do actual collision detection
        let mut parts = self.get_children()
            .into_iter()
            .filter(|child| child.get_class() == InstanceClass::Part);
            
        // Just return the first part for now
        Ok(parts.next())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instance::InstanceClass;
    
    #[test]
    fn test_workspace_physics() {
        let workspace = Instance::new(InstanceClass::Workspace, "Workspace");
        
        // Test gravity
        workspace.set_gravity(100.0).unwrap();
        assert_eq!(workspace.get_gravity().unwrap(), 100.0);
        
        // Test raycast
        let part = Instance::new(InstanceClass::Part, "TestPart");
        part.set_parent(Some(workspace.clone())).unwrap();
        
        let hit = workspace.raycast([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]).unwrap();
        assert!(hit.is_some());
        assert_eq!(hit.unwrap().get_name(), "TestPart");
    }
} 