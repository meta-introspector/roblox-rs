//! World module for the ECS framework
//!
//! The World is the primary container for entities, components, and resources.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use hecs::{EntityBuilder as HecsEntityBuilder, World as HecsWorld};

use crate::entity::{Entity, EntityBuilder, EntityRef};
use crate::resource::Resource;
use crate::Error;
use crate::Result;

/// The World is the primary container for entities, components, and resources.
pub struct World {
    /// The underlying HECS world for entities and components
    hecs: HecsWorld,
    
    /// Resources stored in the world
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    
    /// Shared resources that can be accessed across worlds (like Roblox services)
    shared_resources: Arc<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    /// Create a new, empty world
    pub fn new() -> Self {
        Self {
            hecs: HecsWorld::new(),
            resources: HashMap::new(),
            shared_resources: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create a new entity with the given components
    pub fn spawn(&mut self, components: impl hecs::DynamicBundle) -> Entity {
        let id = self.hecs.spawn(components);
        Entity::new(id.id() as u64)
    }
    
    /// Create a new entity builder
    pub fn spawn_builder(&mut self) -> EntityBuilder {
        EntityBuilder::new(HecsEntityBuilder::new())
    }
    
    /// Get a reference to an entity
    pub fn entity(&self, entity: Entity) -> Result<EntityRef> {
        let hecs_entity = match hecs::Entity::from_bits(entity.id() as u64) {
            Some(e) => e,
            None => return Err(Error::EntityNotFound(entity)),
        };
            
        if !self.hecs.contains(hecs_entity) {
            return Err(Error::EntityNotFound(entity));
        }
        
        Ok(EntityRef::new(entity, hecs_entity, &self.hecs))
    }
    
    /// Get a mutable reference to an entity
    pub fn entity_mut(&mut self, entity: Entity) -> Result<EntityRef> {
        let hecs_entity = match hecs::Entity::from_bits(entity.id() as u64) {
            Some(e) => e,
            None => return Err(Error::EntityNotFound(entity)),
        };
            
        if !self.hecs.contains(hecs_entity) {
            return Err(Error::EntityNotFound(entity));
        }
        
        Ok(EntityRef::new(entity, hecs_entity, &self.hecs))
    }
    
    /// Insert a resource into the world
    pub fn insert_resource<R: Resource>(&mut self, resource: R) {
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
    }
    
    /// Get a resource from the world
    pub fn get_resource<R: Resource>(&self) -> Option<&R> {
        self.resources
            .get(&TypeId::of::<R>())
            .and_then(|res| res.downcast_ref::<R>())
    }
    
    /// Get a mutable resource from the world
    pub fn get_resource_mut<R: Resource>(&mut self) -> Option<&mut R> {
        self.resources
            .get_mut(&TypeId::of::<R>())
            .and_then(|res| res.downcast_mut::<R>())
    }
    
    /// Check if a resource exists in the world
    pub fn has_resource<R: Resource>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<R>())
    }
    
    /// Remove a resource from the world
    pub fn remove_resource<R: Resource>(&mut self) -> Option<R> {
        self.resources
            .remove(&TypeId::of::<R>())
            .and_then(|res| res.downcast::<R>().ok())
            .map(|r| *r)
    }
    
    /// Get a list of all entities in the world
    pub fn entities(&self) -> Vec<Entity> {
        let mut result = Vec::new();
        
        // To iterate all entities, we can use a query with no components requirement
        // and just convert the hecs::Entity to our Entity type
        for (id, _) in self.hecs.query::<()>().iter() {
            result.push(Entity::new(id.id() as u64));
        }
        
        result
    }
    
    /// Get the underlying HECS world
    pub(crate) fn hecs_world(&self) -> &HecsWorld {
        &self.hecs
    }
    
    /// Get a mutable reference to the underlying HECS world
    pub(crate) fn hecs_world_mut(&mut self) -> &mut HecsWorld {
        &mut self.hecs
    }
    
    /// Insert a shared resource that can be accessed across worlds
    pub fn insert_shared_resource<R: Resource + 'static>(&self, resource: R) {
        let mut shared = self.shared_resources.write();
        shared.insert(TypeId::of::<R>(), Arc::new(resource));
    }
    
    /// Get a shared resource
    pub fn get_shared_resource<R: Resource + 'static>(&self) -> Option<Arc<R>> {
        let shared = self.shared_resources.read();
        shared
            .get(&TypeId::of::<R>())
            .and_then(|res| res.clone().downcast::<R>().ok())
    }
} 