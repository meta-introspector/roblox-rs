//! Resource module for the ECS framework
//!
//! Resources are global data that can be accessed by systems, but are not
//! associated with any particular entity.

use std::ops::{Deref, DerefMut};
use std::fmt::Debug;

/// Trait for resource types
///
/// Resources must be Send + Sync + 'static for safe concurrent access.
/// This is a marker trait that must be manually implemented for types to be used as resources.
pub trait Resource: Send + Sync + 'static {}

// We'll implement this trait for specific types in their respective modules
// No blanket implementation to avoid conflicts

/// A read-only reference to a resource
///
/// This is typically used as a system parameter.
pub struct Res<'a, T: Resource> {
    value: &'a T,
}

impl<'a, T: Resource> Res<'a, T> {
    /// Create a new resource reference
    pub(crate) fn new(value: &'a T) -> Self {
        Self { value }
    }
}

impl<'a, T: Resource> Deref for Res<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T: Resource + Debug> Debug for Res<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Res")
            .field("value", &self.value)
            .finish()
    }
}

/// A mutable reference to a resource
///
/// This is typically used as a system parameter.
pub struct ResMut<'a, T: Resource> {
    value: &'a mut T,
}

impl<'a, T: Resource> ResMut<'a, T> {
    /// Create a new mutable resource reference
    pub(crate) fn new(value: &'a mut T) -> Self {
        Self { value }
    }
}

impl<'a, T: Resource> Deref for ResMut<'a, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T: Resource> DerefMut for ResMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

impl<'a, T: Resource + Debug> Debug for ResMut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResMut")
            .field("value", &self.value)
            .finish()
    }
}

/// Helper macro to implement Resource for a type
#[macro_export]
macro_rules! impl_resource {
    ($type:ty) => {
        impl $crate::resource::Resource for $type {}
    };
}

// Internal convenience implementations for common types
impl Resource for String {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::World;
    
    // Implement Resource for Counter for test
    struct Counter(i32);
    impl Resource for Counter {}
    
    #[test]
    fn resource_access() {
        let mut world = World::new();
        
        world.insert_resource(Counter(10));
        
        let counter = world.get_resource::<Counter>().unwrap();
        assert_eq!(counter.0, 10);
        
        let counter_mut = world.get_resource_mut::<Counter>().unwrap();
        counter_mut.0 += 1;
        
        let counter = world.get_resource::<Counter>().unwrap();
        assert_eq!(counter.0, 11);
    }
} 