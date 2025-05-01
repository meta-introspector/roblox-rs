//! App module for the ECS framework
//!
//! The App is the main entry point for the ECS framework.

use std::any::TypeId;
use crate::plugin::{Plugin, PluginGroup};
use crate::resource::Resource;
use crate::schedule::Schedule;
use crate::system::{IntoSystem, System};
use crate::world::World;

/// The main entry point for the ECS framework
pub struct App {
    /// The world containing entities, components, and resources
    world: World,
    /// The schedule of systems to run
    schedule: Schedule,
}

impl App {
    /// Create a new app
    pub fn new() -> Self {
        Self {
            world: World::new(),
            schedule: Schedule::new(),
        }
    }
    
    /// Get a reference to the world
    pub fn world(&self) -> &World {
        &self.world
    }
    
    /// Get a mutable reference to the world
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
    
    /// Add a plugin to the app
    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        plugin.build(self);
        self
    }
    
    /// Add a plugin group to the app
    pub fn add_plugins<G: PluginGroup>(&mut self, mut group: G) -> &mut Self {
        group.configure();
        group.build(self);
        self
    }
    
    /// Add a system to the app
    pub fn add_system<S, P>(&mut self, system: S) -> &mut Self
    where
        S: IntoSystem<P>,
    {
        self.schedule.add_system(system.into_system());
        self
    }
    
    /// Insert a resource into the world
    pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.world.insert_resource(resource);
        self
    }
    
    /// Initialize a resource with its default value
    pub fn init_resource<R: Resource + Default>(&mut self) -> &mut Self {
        if !self.world.has_resource::<R>() {
            self.world.insert_resource(R::default());
        }
        self
    }
    
    /// Run the app once
    pub fn update(&mut self) {
        self.schedule.run(&mut self.world);
    }
    
    /// Run the app
    pub fn run(&mut self) {
        // In a real Roblox environment, we would connect this to a heartbeat signal
        // For this simulation, we'll just run the update loop once
        self.update();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// A builder for creating an app
pub struct AppBuilder {
    app: App,
}

impl AppBuilder {
    /// Create a new app builder
    pub fn new() -> Self {
        Self {
            app: App::new(),
        }
    }
    
    /// Add a plugin to the app
    pub fn add_plugin<P: Plugin>(mut self, plugin: P) -> Self {
        self.app.add_plugin(plugin);
        self
    }
    
    /// Add a plugin group to the app
    pub fn add_plugins<G: PluginGroup>(mut self, group: G) -> Self {
        self.app.add_plugins(group);
        self
    }
    
    /// Add a system to the app
    pub fn add_system<S, P>(mut self, system: S) -> Self
    where
        S: IntoSystem<P>,
    {
        self.app.add_system(system);
        self
    }
    
    /// Insert a resource into the world
    pub fn insert_resource<R: Resource>(mut self, resource: R) -> Self {
        self.app.insert_resource(resource);
        self
    }
    
    /// Initialize a resource with its default value
    pub fn init_resource<R: Resource + Default>(mut self) -> Self {
        self.app.init_resource::<R>();
        self
    }
    
    /// Build the app
    pub fn build(self) -> App {
        self.app
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn app_builder() {
        struct TestResource(i32);
        
        fn test_system(_res: crate::resource::Res<TestResource>) {
            // Do nothing
        }
        
        let app = AppBuilder::new()
            .insert_resource(TestResource(42))
            .add_system(test_system)
            .build();
            
        assert!(app.world().has_resource::<TestResource>());
        assert_eq!(app.world().get_resource::<TestResource>().unwrap().0, 42);
    }
} 