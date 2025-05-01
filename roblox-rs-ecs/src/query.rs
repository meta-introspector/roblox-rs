//! Query module for the ECS framework
//!
//! Queries allow systems to efficiently access collections of entities that have
//! specific component types.

use std::marker::PhantomData;
use hecs::{Query as HecsQuery, QueryBorrow};
use crate::world::World;
use crate::entity::Entity;

/// A query for entities that have a specific set of components
///
/// Queries are used to efficiently find and access entities based on their components.
/// For example, a `Query<(&Position, &mut Velocity)>` will find all entities that have
/// both a `Position` and a `Velocity` component, and provide read access to `Position`
/// and write access to `Velocity`.
pub struct Query<'w, Q: HecsQuery> {
    world: &'w World,
    _marker: PhantomData<Q>,
}

impl<'w, Q: HecsQuery> Query<'w, Q> {
    /// Create a new query
    pub(crate) fn new(world: &'w World) -> Self {
        Self {
            world,
            _marker: PhantomData,
        }
    }
    
    /// Get an iterator over entities and components that match the query
    pub fn iter(&self) -> QueryIter<Q> {
        let borrow = self.world.hecs_world().query::<Q>();
        QueryIter {
            iter: borrow.iter(),
        }
    }
    
    /// Check if there are any entities that match the query
    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }
    
    /// Count the number of entities that match the query
    pub fn count(&self) -> usize {
        self.iter().count()
    }
}

/// An iterator over entities and components that match a query
pub struct QueryIter<Q: HecsQuery> {
    iter: hecs::QueryIter<Q>,
}

impl<Q: HecsQuery> Iterator for QueryIter<Q> {
    type Item = (Entity, Q::Fetch);
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(hecs_entity, components)| {
            (Entity::new(hecs_entity.id() as u64), components)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn query_entities() {
        let mut world = World::new();
        
        #[derive(Debug, PartialEq)]
        struct Position { x: f32, y: f32 }
        
        #[derive(Debug, PartialEq)]
        struct Velocity { x: f32, y: f32 }
        
        let e1 = world.spawn((Position { x: 1.0, y: 2.0 }, Velocity { x: 3.0, y: 4.0 }));
        let e2 = world.spawn((Position { x: 5.0, y: 6.0 },));
        let e3 = world.spawn((Velocity { x: 7.0, y: 8.0 },));
        
        let query = Query::<&Position>::new(&world);
        let positions: Vec<_> = query.iter().map(|(_, pos)| pos).collect();
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&&Position { x: 1.0, y: 2.0 }));
        assert!(positions.contains(&&Position { x: 5.0, y: 6.0 }));
        
        let query = Query::<&Velocity>::new(&world);
        let velocities: Vec<_> = query.iter().map(|(_, vel)| vel).collect();
        assert_eq!(velocities.len(), 2);
        assert!(velocities.contains(&&Velocity { x: 3.0, y: 4.0 }));
        assert!(velocities.contains(&&Velocity { x: 7.0, y: 8.0 }));
        
        let query = Query::<(&Position, &Velocity)>::new(&world);
        let combined: Vec<_> = query.iter().collect();
        assert_eq!(combined.len(), 1);
        
        let (_, (pos, vel)) = combined[0];
        assert_eq!(pos, &Position { x: 1.0, y: 2.0 });
        assert_eq!(vel, &Velocity { x: 3.0, y: 4.0 });
    }
} 