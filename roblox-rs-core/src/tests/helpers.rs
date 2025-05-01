use crate::ast::{LuauNode, Table};
use std::string::String;

// Define these types directly instead of importing them from ast::luau
type Array = Vec<LuauNode>;

enum Number {
    Integer(i64),
    Float(f64),
}

type LuauString = String;
use crate::codegen::{LuauCodeGenerator, OptimizationLevel};
use crate::runtime::optimized::RuntimeOptimizer;

/// Helper functions for testing optimizations
pub struct TestHelper {
    pub generator: LuauCodeGenerator,
    pub optimizer: RuntimeOptimizer,
}

impl TestHelper {
    pub fn new() -> Self {
        Self {
            generator: LuauCodeGenerator::new(),
            optimizer: RuntimeOptimizer::new(),
        }
    }

    /// Create a test array of numbers
    pub fn create_numeric_array(&self, size: usize) -> LuauNode {
        let elements: Vec<LuauNode> = (0..size)
            .map(|i| LuauNode::Number(Number::Integer(i as i64)))
            .collect();
        LuauNode::Array(Array { elements })
    }

    /// Create a test array of strings
    pub fn create_string_array(&self, size: usize) -> LuauNode {
        let elements: Vec<LuauNode> = (0..size)
            .map(|i| LuauNode::String(format!("test_{}", i)))
            .collect();
        LuauNode::Array(Array { elements })
    }

    /// Create a test table with fields
    pub fn create_test_table(&self, field_count: usize) -> LuauNode {
        let mut fields = Vec::new();
        for i in 0..field_count {
            fields.push((
                format!("field_{}", i),
                LuauNode::Number(Number::Integer(i as i64)),
            ));
        }
        LuauNode::Table(Table { fields })
    }

    /// Generate and validate optimized code
    pub fn generate_and_validate(&mut self, node: &LuauNode, expected_patterns: &[&str]) -> bool {
        self.generator.generate_node(node).expect("Failed to generate code");
        let code = self.generator.get_output();
        
        expected_patterns.iter().all(|pattern| code.contains(pattern))
    }

    /// Measure code generation time
    pub fn measure_generation_time(&mut self, node: &LuauNode) -> std::time::Duration {
        let start = std::time::Instant::now();
        self.generator.generate_node(node).expect("Failed to generate code");
        start.elapsed()
    }

    /// Compare optimized vs unoptimized generation
    pub fn compare_optimization_impact(&mut self, node: &LuauNode) -> (std::time::Duration, std::time::Duration) {
        // Measure optimized time
        let optimized_time = self.measure_generation_time(node);

        // Reset generator and disable optimizations
        self.generator = LuauCodeGenerator::new();
        self.generator.set_optimization_level(OptimizationLevel::None);

        // Measure unoptimized time
        let unoptimized_time = self.measure_generation_time(node);

        (optimized_time, unoptimized_time)
    }

    /// Validate memory usage patterns
    pub fn validate_memory_patterns(&mut self, node: &LuauNode) -> bool {
        self.generator.generate_node(node).expect("Failed to generate code");
        let code = self.generator.get_output();

        // Check for memory optimization patterns
        let patterns = [
            "table.create",
            "MemoryManager:trackAllocation",
            "MemoryManager:trackFree",
        ];

        patterns.iter().all(|pattern| code.contains(pattern))
    }

    /// Validate type specialization
    pub fn validate_type_specialization(&mut self, node: &LuauNode) -> bool {
        self.generator.generate_node(node).expect("Failed to generate code");
        let code = self.generator.get_output();

        // Check for specialization patterns
        let patterns = [
            "specialized",
            "inline",
            "optimized",
        ];

        patterns.iter().any(|pattern| code.contains(pattern))
    }

    /// Validate vectorization
    pub fn validate_vectorization(&mut self, node: &LuauNode) -> bool {
        self.generator.generate_node(node).expect("Failed to generate code");
        let code = self.generator.get_output();

        // Check for vectorization patterns
        let patterns = [
            "SimdHelpers",
            "mapChunks",
            "chunk",
        ];

        patterns.iter().all(|pattern| code.contains(pattern))
    }

    /// Create test cases for different optimization scenarios
    pub fn create_test_cases(&self) -> Vec<(LuauNode, Vec<&'static str>)> {
        vec![
            // Numeric array test case
            (
                self.create_numeric_array(10),
                vec!["SimdHelpers", "mapChunks"],
            ),
            // String array test case
            (
                self.create_string_array(10),
                vec!["StringOptimizer", "fastConcat"],
            ),
            // Table test case
            (
                self.create_test_table(10),
                vec!["table.create", "MemoryManager"],
            ),
        ]
    }

    /// Run a complete optimization test suite
    pub fn run_test_suite(&mut self) -> Vec<TestResult> {
        let test_cases = self.create_test_cases();
        let mut results = Vec::new();

        for (node, expected_patterns) in test_cases {
            let success = self.generate_and_validate(&node, &expected_patterns);
            let (opt_time, unopt_time) = self.compare_optimization_impact(&node);
            
            results.push(TestResult {
                success,
                optimized_time: opt_time,
                unoptimized_time: unopt_time,
                memory_optimized: self.validate_memory_patterns(&node),
                vectorized: self.validate_vectorization(&node),
                specialized: self.validate_type_specialization(&node),
            });
        }

        results
    }
}

/// Test result structure
#[derive(Debug)]
pub struct TestResult {
    pub success: bool,
    pub optimized_time: std::time::Duration,
    pub unoptimized_time: std::time::Duration,
    pub memory_optimized: bool,
    pub vectorized: bool,
    pub specialized: bool,
}
