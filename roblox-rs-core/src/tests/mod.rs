// Test modules
pub mod helpers;
pub mod optimizations;
pub mod integration_test;
pub mod basic_test;
pub mod dependencies_test;

pub use helpers::TestHelper;

#[cfg(test)]
pub mod pooling_test;
pub mod parallel_test;
pub mod type_declaration_test;
pub mod rojo_test;
pub mod rojo_standalone_test;
pub mod rojo_integration_test;
pub mod luau_execution_test;
