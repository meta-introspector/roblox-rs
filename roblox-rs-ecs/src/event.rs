//! Event module for the ECS framework
//!
//! Events allow for communication between systems.

use std::marker::PhantomData;
use std::sync::Arc;
use parking_lot::RwLock;

use crate::resource::Resource;
use crate::system::{SystemParam, SystemParamFetch};
use crate::world::World;

/// A trait for event types
pub trait Event: Send + Sync + 'static {}

// Implement for any type that meets the requirements
impl<T: Send + Sync + 'static> Event for T {}

/// An event queue for a specific event type
pub struct Events<T: Event> {
    /// Events that have been sent
    events: Vec<T>,
    /// Events from the previous frame
    events_previous: Vec<T>,
    /// Whether the events have been swapped this frame
    swapped: bool,
}

impl<T: Event> Default for Events<T> {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            events_previous: Vec::new(),
            swapped: false,
        }
    }
}

impl<T: Event> Events<T> {
    /// Create a new event queue
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Send an event
    pub fn send(&mut self, event: T) {
        self.events.push(event);
    }
    
    /// Iterate over all events
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.events.iter()
    }
    
    /// Clear all events
    pub fn clear(&mut self) {
        self.events.clear();
        self.events_previous.clear();
    }
    
    /// Get the number of events
    pub fn len(&self) -> usize {
        self.events.len()
    }
    
    /// Check if there are no events
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
    
    /// Update the event queue (called once per frame)
    pub fn update(&mut self) {
        // Swap the event buffers
        std::mem::swap(&mut self.events, &mut self.events_previous);
        self.events.clear();
        self.swapped = true;
    }
}

// Make Events a Resource
impl<T: Event> Resource for Events<T> {}

/// A reader for events
///
/// This allows a system to read events without consuming them.
pub struct EventReader<T: Event> {
    /// The last event index that was read
    last_index: usize,
    /// Marker for the event type
    _marker: PhantomData<T>,
}

impl<T: Event> Default for EventReader<T> {
    fn default() -> Self {
        Self {
            last_index: 0,
            _marker: PhantomData,
        }
    }
}

impl<T: Event> EventReader<T> {
    /// Create a new event reader
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Read events from the event queue
    pub fn read<'a>(&mut self, events: &'a Events<T>) -> impl Iterator<Item = &'a T> {
        // Get the events that haven't been read yet
        let from_index = self.last_index;
        self.last_index = events.events.len();
        
        // Return an iterator over the events
        events.events[from_index..].iter()
    }
    
    /// Check if there are any unread events
    pub fn has_new_events(&self, events: &Events<T>) -> bool {
        self.last_index < events.events.len()
    }
}

/// A writer for events
///
/// This is used to send events.
pub struct EventWriter<T: Event> {
    /// Marker for the event type
    _marker: PhantomData<T>,
}

impl<T: Event> Default for EventWriter<T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T: Event> EventWriter<T> {
    /// Create a new event writer
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Send an event to the event queue
    pub fn send(&self, world: &mut World, event: T) {
        // Get or create the event queue
        if !world.has_resource::<Events<T>>() {
            world.insert_resource(Events::<T>::default());
        }
        
        // Send the event
        let events = world.get_resource_mut::<Events<T>>().unwrap();
        events.send(event);
    }
}

/// Update all event queues in the world
///
/// This should be called once per frame, typically at the end of a frame.
pub fn update_event_queues(world: &mut World) {
    // This is a simplified implementation; in a real system we would
    // want to iterate through all registered event types
    
    // For now, we'll update a few common event types as examples
    if let Some(mut events) = world.get_resource_mut::<Events<()>>() {
        events.update();
    }
    
    // Add other event types as needed
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn event_basic() {
        // Create an event queue
        let mut events = Events::<i32>::new();
        
        // Send some events
        events.send(1);
        events.send(2);
        events.send(3);
        
        // Check that the events were received
        let mut received = Vec::new();
        for event in events.iter() {
            received.push(*event);
        }
        
        assert_eq!(received, vec![1, 2, 3]);
    }
    
    #[test]
    fn event_reader() {
        // Create an event queue
        let mut events = Events::<i32>::new();
        let mut reader = EventReader::<i32>::new();
        
        // Send some events
        events.send(1);
        events.send(2);
        
        // Read the events
        let mut received = Vec::new();
        for event in reader.read(&events) {
            received.push(*event);
        }
        
        assert_eq!(received, vec![1, 2]);
        
        // Send more events
        events.send(3);
        
        // Reader should only see the new event
        let mut received = Vec::new();
        for event in reader.read(&events) {
            received.push(*event);
        }
        
        assert_eq!(received, vec![3]);
    }
    
    #[test]
    fn event_writer() {
        let mut world = World::new();
        let writer = EventWriter::<i32>::new();
        
        // Send an event
        writer.send(&mut world, 42);
        
        // Check that the event was received
        let events = world.get_resource::<Events<i32>>().unwrap();
        assert_eq!(events.iter().next(), Some(&42));
    }
    
    #[test]
    fn event_update() {
        let mut events = Events::<i32>::new();
        let mut reader = EventReader::<i32>::new();
        
        // Send events in frame 1
        events.send(1);
        events.send(2);
        
        // Reader reads all events
        let mut received = Vec::new();
        for event in reader.read(&events) {
            received.push(*event);
        }
        assert_eq!(received, vec![1, 2]);
        
        // Update the event queue (end of frame 1)
        events.update();
        
        // Send events in frame 2
        events.send(3);
        events.send(4);
        
        // Reader should only see frame 2 events
        let mut received = Vec::new();
        for event in reader.read(&events) {
            received.push(*event);
        }
        assert_eq!(received, vec![3, 4]);
    }
} 