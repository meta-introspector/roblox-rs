//! System module for the ECS framework
//!
//! Systems are functions that operate on entities, components, and resources.

use std::any::TypeId;
use std::marker::PhantomData;

use crate::entity::Entity;
use crate::world::World;
use crate::resource::{Res, ResMut, Resource};

/// A system that can be run on the world
///
/// Systems are the primary way to implement game logic in the ECS framework.
pub trait System: Send + Sync + 'static {
    /// Run the system on the given world
    fn run(&mut self, world: &mut World);
    
    /// Get the name of the system
    fn name(&self) -> &str;
    
    /// Get the system's dependencies (other systems that must run before this one)
    fn dependencies(&self) -> &[TypeId];
}

/// A trait for types that can be converted into a system
pub trait IntoSystem {
    /// The resulting system type
    type System: System;
    
    /// Convert this value into a system
    fn into_system(self) -> Self::System;
}

/// A command buffer for deferred operations on the world
///
/// Commands allow systems to queue up operations on the world
/// that will be executed after all systems have run.
pub struct Commands {
    operations: Vec<Box<dyn FnOnce(&mut World) + Send + Sync>>,
}

impl Commands {
    /// Create a new command buffer
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }
    
    /// Queue an operation to be executed later
    pub fn add<F>(&mut self, f: F)
    where
        F: FnOnce(&mut World) + Send + Sync + 'static,
    {
        self.operations.push(Box::new(f));
    }
    
    /// Spawn a new entity with the given components
    pub fn spawn<T>(&mut self, components: T)
    where
        T: hecs::DynamicBundle + Send + Sync + 'static,
    {
        self.add(move |world| {
            world.spawn(components);
        });
    }
    
    /// Despawn an entity
    pub fn despawn(&mut self, entity: Entity) {
        self.add(move |world| {
            let hecs_entity = match hecs::Entity::from_bits(entity.id() as u64) {
                Some(e) => e,
                None => return,
            };
            
            if world.hecs_world().contains(hecs_entity) {
                let _ = world.hecs_world_mut().despawn(hecs_entity);
            }
        });
    }
    
    /// Insert a resource into the world
    pub fn insert_resource<R: Resource>(&mut self, resource: R) {
        self.add(move |world| {
            world.insert_resource(resource);
        });
    }
    
    /// Remove a resource from the world
    pub fn remove_resource<R: Resource>(&mut self) {
        self.add(move |world| {
            let _ = world.remove_resource::<R>();
        });
    }
    
    /// Execute all queued operations on the world
    pub fn apply(self, world: &mut World) {
        for op in self.operations {
            op(world);
        }
    }
}

/// A function system that operates directly on the world
pub struct FunctionSystem<F> {
    func: F,
    name: String,
    dependencies: Vec<TypeId>,
}

impl<F> FunctionSystem<F> {
    /// Create a new function system
    pub fn new(func: F) -> Self {
        Self {
            func,
            name: std::any::type_name::<F>().to_string(),
            dependencies: Vec::new(),
        }
    }
    
    /// Add a dependency on another system
    pub fn with_dependency<S: 'static>(mut self) -> Self {
        self.dependencies.push(TypeId::of::<S>());
        self
    }
}

impl<F> System for FunctionSystem<F>
where
    F: FnMut(&mut World) + Send + Sync + 'static,
{
    fn run(&mut self, world: &mut World) {
        (self.func)(world);
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn dependencies(&self) -> &[TypeId] {
        &self.dependencies
    }
}

// Implementation of IntoSystem for functions that take a World parameter
impl<F> IntoSystem for F
where
    F: FnMut(&mut World) + Send + Sync + 'static,
{
    type System = FunctionSystem<F>;
    
    fn into_system(self) -> Self::System {
        FunctionSystem::new(self)
    }
}

/// A marker trait for types that can be used as system parameters
pub trait SystemParam {
    /// Extract the parameter from the world
    fn extract<'a>(world: &'a mut World) -> Self;
}

// Implement SystemParam for Commands
impl SystemParam for Commands {
    fn extract<'a>(_world: &'a mut World) -> Self {
        Commands::new()
    }
}

// Implement SystemParam for World references
impl<'a> SystemParam for &'a mut World {
    fn extract<'w>(world: &'w mut World) -> Self {
        // Safety: This is safe as long as the system only runs once
        // with the extracted parameters and doesn't store them
        unsafe { std::mem::transmute(world) }
    }
}

// Implement SystemParam for Res
impl<'a, T: Resource> SystemParam for Res<'a, T> {
    fn extract<'w>(world: &'w mut World) -> Self {
        let res = world.get_resource::<T>()
            .expect(&format!("Resource not found: {}", std::any::type_name::<T>()));
        
        // Safety: This is safe as long as the system only runs once 
        // with the extracted parameters and doesn't store them
        Res::new(unsafe { std::mem::transmute(res) })
    }
}

// Implement SystemParam for ResMut
impl<'a, T: Resource> SystemParam for ResMut<'a, T> {
    fn extract<'w>(world: &'w mut World) -> Self {
        let res = world.get_resource_mut::<T>()
            .expect(&format!("Resource not found: {}", std::any::type_name::<T>()));
        
        // Safety: This is safe as long as the system only runs once
        // with the extracted parameters and doesn't store them
        ResMut::new(unsafe { std::mem::transmute(res) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn function_system() {
        let mut world = World::new();
        
        struct TestResource(i32);
        impl Resource for TestResource {}
        
        world.insert_resource(TestResource(0));
        
        fn test_system(world: &mut World) {
            let mut res = world.get_resource_mut::<TestResource>().unwrap();
            res.0 += 1;
        }
        
        let mut system = test_system.into_system();
        
        system.run(&mut world);
        
        assert_eq!(world.get_resource::<TestResource>().unwrap().0, 1);
        
        system.run(&mut world);
        
        assert_eq!(world.get_resource::<TestResource>().unwrap().0, 2);
    }
    
    #[test]
    fn commands_system() {
        let mut world = World::new();
        
        let mut commands = Commands::new();
        commands.insert_resource(42u32);
        
        // Resource isn't added yet because commands are deferred
        assert!(world.get_resource::<u32>().is_none());
        
        // Apply the commands
        commands.apply(&mut world);
        
        // Now the resource should be present
        assert_eq!(*world.get_resource::<u32>().unwrap(), 42);
    }
} 
} 