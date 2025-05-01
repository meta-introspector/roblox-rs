//! Entity module for the ECS framework
//!
//! Entities are the fundamental unit of the ECS architecture, serving as unique
//! identifiers for collections of components.

use std::fmt;
use hecs::{EntityBuilder as HecsEntityBuilder, World as HecsWorld};
use crate::component::Component;
use crate::Error;
use crate::Result;

/// A unique identifier for an entity in the world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    id: u64,
}

impl Entity {
    /// Create a new entity with the given ID
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
    
    /// Get the entity's ID
    pub fn id(&self) -> u64 {
        self.id
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity({:#x})", self.id)
    }
}

/// A builder for creating entities with components
pub struct EntityBuilder {
    // Using Option to allow taking ownership in build
    builder: Option<HecsEntityBuilder>,
}

impl EntityBuilder {
    /// Create a new entity builder
    pub(crate) fn new(builder: HecsEntityBuilder) -> Self {
        Self { 
            builder: Some(builder) 
        }
    }
    
    /// Add a component to the entity
    pub fn with<T: Component>(mut self, component: T) -> Self {
        if let Some(builder) = &mut self.builder {
            builder.add(component);
        }
        self
    }
    
    /// Build the entity and spawn it in the world
    pub fn build(mut self, world: &mut crate::world::World) -> Entity {
        // Take ownership of the builder
        let builder = self.builder.take().expect("EntityBuilder already built");
        let bundle = builder.build();
        let hecs_entity = world.hecs_world_mut().spawn(bundle);
        Entity::new(hecs_entity.id() as u64)
    }
}

/// A reference to an entity in the world, used for querying and manipulating components
pub struct EntityRef<'a> {
    entity: Entity,
    hecs_entity: hecs::Entity,
    world: &'a HecsWorld,
}

impl<'a> EntityRef<'a> {
    /// Create a new entity reference
    pub(crate) fn new(entity: Entity, hecs_entity: hecs::Entity, world: &'a HecsWorld) -> Self {
        Self {
            entity,
            hecs_entity,
            world,
        }
    }
    
    /// Get the entity's ID
    pub fn id(&self) -> Entity {
        self.entity
    }
    
    /// Check if the entity has a component of the given type
    pub fn has<T: Component>(&self) -> bool {
        self.world.contains::<T>(self.hecs_entity)
    }
    
    /// Get a reference to a component on the entity
    pub fn get<T: Component>(&self) -> Result<&'a T> {
        // Note: We need to use a different approach since hecs doesn't directly
        // support borrowing components with arbitrary lifetimes
        match self.world.get_unchecked::<T>(self.hecs_entity) {
            Ok(component_ref) => {
                // This is safe because we're tying the component's lifetime to the EntityRef,
                // which itself is tied to the world reference
                let component_ptr = component_ref.as_ptr();
                Ok(unsafe { &*component_ptr })
            }
            Err(_) => Err(Error::ComponentNotFound(self.entity)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::World;
    
    #[test]
    fn entity_builder() {
        let mut world = World::new();
        
        #[derive(Debug, PartialEq)]
        struct Position { x: f32, y: f32 }
        
        #[derive(Debug, PartialEq)]
        struct Velocity { x: f32, y: f32 }
        
        let entity = world.spawn_builder()
            .with(Position { x: 1.0, y: 2.0 })
            .with(Velocity { x: 3.0, y: 4.0 })
            .build(&mut world);
            
        assert!(world.entity(entity).is_ok());
        
        let entity_ref = world.entity(entity).unwrap();
        assert!(entity_ref.has::<Position>());
        assert!(entity_ref.has::<Velocity>());
        
        assert_eq!(
            entity_ref.get::<Position>().unwrap(),
            &Position { x: 1.0, y: 2.0 }
        );
        
        assert_eq!(
            entity_ref.get::<Velocity>().unwrap(),
            &Velocity { x: 3.0, y: 4.0 }
        );
    }
} 