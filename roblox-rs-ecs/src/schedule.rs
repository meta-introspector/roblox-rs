//! Schedule module for the ECS framework
//!
//! The Schedule manages the execution of systems.

use std::any::TypeId;
use std::collections::{HashMap, HashSet};

use crate::system::{Commands, System};
use crate::world::World;

/// A stage in the schedule
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stage {
    /// Pre-update stage (before the main update)
    PreUpdate,
    /// Main update stage
    Update,
    /// Post-update stage (after the main update)
    PostUpdate,
    /// Last stage (after all other stages)
    Last,
}

impl Stage {
    /// Get the stage's order (for sorting)
    pub fn order(&self) -> usize {
        match self {
            Stage::PreUpdate => 0,
            Stage::Update => 1,
            Stage::PostUpdate => 2,
            Stage::Last => 3,
        }
    }
}

/// A schedule that manages the execution of systems
pub struct Schedule {
    /// Systems grouped by stage
    stages: HashMap<Stage, Vec<Box<dyn System>>>,
    /// Command buffers for each system
    command_buffers: Vec<Commands<'static>>,
}

impl Schedule {
    /// Create a new schedule
    pub fn new() -> Self {
        let mut stages = HashMap::new();
        stages.insert(Stage::PreUpdate, Vec::new());
        stages.insert(Stage::Update, Vec::new());
        stages.insert(Stage::PostUpdate, Vec::new());
        stages.insert(Stage::Last, Vec::new());
        
        Self {
            stages,
            command_buffers: Vec::new(),
        }
    }
    
    /// Add a system to the schedule (in the Update stage)
    pub fn add_system(&mut self, system: impl System + 'static) -> &mut Self {
        self.add_system_to_stage(Stage::Update, system)
    }
    
    /// Add a system to a specific stage
    pub fn add_system_to_stage(
        &mut self,
        stage: Stage,
        system: impl System + 'static,
    ) -> &mut Self {
        self.stages
            .entry(stage)
            .or_insert_with(Vec::new)
            .push(Box::new(system));
        self
    }
    
    /// Run all systems in the schedule
    pub fn run(&mut self, world: &mut World) {
        // Run systems in order of stages
        let stages = [Stage::PreUpdate, Stage::Update, Stage::PostUpdate, Stage::Last];
        
        for stage in stages.iter() {
            if let Some(systems) = self.stages.get_mut(stage) {
                for system in systems.iter_mut() {
                    system.run(world);
                }
            }
        }
        
        // Apply command buffers
        for command_buffer in self.command_buffers.iter_mut() {
            // SAFETY: This is safe because we're ensuring the command buffer is only applied
            // after all systems have run, and we're not holding any references to the world
            // while applying commands.
            let commands = unsafe { std::mem::transmute::<_, &mut Commands>(command_buffer) };
            commands.apply(world);
        }
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::{Res, ResMut, Resource};
    use crate::system::IntoSystem;
    
    #[derive(Default)]
    struct Counter(i32);
    
    impl Resource for Counter {}
    
    #[derive(Default)]
    struct PreUpdateCounter(i32);
    
    impl Resource for PreUpdateCounter {}
    
    #[derive(Default)]
    struct PostUpdateCounter(i32);
    
    impl Resource for PostUpdateCounter {}
    
    #[test]
    fn schedule_stages() {
        let mut world = World::new();
        world.insert_resource(Counter::default());
        world.insert_resource(PreUpdateCounter::default());
        world.insert_resource(PostUpdateCounter::default());
        
        fn pre_update_system(mut counter: ResMut<PreUpdateCounter>) {
            counter.0 += 1;
        }
        
        fn update_system(mut counter: ResMut<Counter>) {
            counter.0 += 1;
        }
        
        fn post_update_system(mut counter: ResMut<PostUpdateCounter>) {
            counter.0 += 1;
        }
        
        let mut schedule = Schedule::new();
        schedule
            .add_system_to_stage(Stage::PreUpdate, pre_update_system.into_system())
            .add_system(update_system.into_system())
            .add_system_to_stage(Stage::PostUpdate, post_update_system.into_system());
            
        schedule.run(&mut world);
        
        assert_eq!(world.get_resource::<PreUpdateCounter>().unwrap().0, 1);
        assert_eq!(world.get_resource::<Counter>().unwrap().0, 1);
        assert_eq!(world.get_resource::<PostUpdateCounter>().unwrap().0, 1);
    }
} 