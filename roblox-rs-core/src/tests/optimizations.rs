use std::time::Instant;
use crate::runtime::optimized::{RuntimeOptimizer, TypeOptimizer};
use crate::codegen::LuauCodeGenerator;
use crate::ast::luau::{LuauNode, Table, Array, Number};

/// Test suite for runtime optimizations
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_operations() {
        let optimizer = RuntimeOptimizer::new();
        let code = optimizer.generate_runtime_code();
        
        // Verify SIMD helper functions are present
        assert!(code.contains("SimdHelpers"));
        assert!(code.contains("add4"));
        assert!(code.contains("mul4"));
        assert!(code.contains("mapChunks"));
    }

    #[test]
    fn test_array_optimizations() {
        let mut generator = LuauCodeGenerator::new();
        
        // Create a test array
        let array = LuauNode::Array(Array {
            elements: vec![
                LuauNode::Number(Number::Integer(1)),
                LuauNode::Number(Number::Integer(2)),
                LuauNode::Number(Number::Integer(3)),
                LuauNode::Number(Number::Integer(4)),
            ],
        });

        // Generate code with optimizations
        generator.generate_node(&array).expect("Failed to generate code");
        let code = generator.get_output();

        // Verify vectorized operations are used
        assert!(code.contains("SimdHelpers.mapChunks"));
        assert!(code.contains("chunk"));
    }

    #[test]
    fn test_string_optimizations() {
        let optimizer = RuntimeOptimizer::new();
        let args = vec![
            String::from("\"Hello, World!\""),
            String::from("\", \""),
        ];

        // Test string operations
        let concat_code = optimizer.get_type_optimization("string", "concat", &args)
            .expect("Failed to get string optimization");
        assert!(concat_code.contains("StringOptimizer.fastConcat"));

        let split_code = optimizer.get_type_optimization("string", "split", &args)
            .expect("Failed to get string optimization");
        assert!(split_code.contains("StringOptimizer.fastSplit"));
    }

    #[test]
    fn test_number_optimizations() {
        let optimizer = RuntimeOptimizer::new();
        let args = vec![
            String::from("2"),
            String::from("3"),
        ];

        // Test numeric operations
        let pow_code = optimizer.get_type_optimization("number", "pow", &args)
            .expect("Failed to get number optimization");
        assert!(pow_code.contains("NumberOptimizer.fastPow"));
    }

    #[test]
    fn test_optimization_performance() {
        let mut generator = LuauCodeGenerator::new();
        
        // Create a large array for testing
        let elements: Vec<LuauNode> = (0..1000)
            .map(|i| LuauNode::Number(Number::Integer(i)))
            .collect();
        
        let large_array = LuauNode::Array(Array { elements });

        // Measure time with optimizations
        let start = Instant::now();
        generator.generate_node(&large_array).expect("Failed to generate optimized code");
        let optimized_time = start.elapsed();

        // Reset generator and disable optimizations
        generator = LuauCodeGenerator::new();
        generator.set_optimization_level(OptimizationLevel::None);

        // Measure time without optimizations
        let start = Instant::now();
        generator.generate_node(&large_array).expect("Failed to generate unoptimized code");
        let unoptimized_time = start.elapsed();

        // Verify optimizations improve performance
        assert!(optimized_time < unoptimized_time);
    }

    #[test]
    fn test_memory_usage() {
        let mut generator = LuauCodeGenerator::new();
        
        // Create a table with many fields
        let mut fields = Vec::new();
        for i in 0..100 {
            fields.push((
                format!("field_{}", i),
                LuauNode::Number(Number::Integer(i)),
            ));
        }
        
        let table = LuauNode::Table(Table { fields });

        // Generate code and verify memory optimizations
        generator.generate_node(&table).expect("Failed to generate code");
        let code = generator.get_output();

        // Check for memory optimization features
        assert!(code.contains("table.create")); // Pre-allocation
        assert!(code.contains("MemoryManager:trackAllocation")); // Memory tracking
    }

    #[test]
    fn test_type_specialization() {
        let mut generator = LuauCodeGenerator::new();
        
        // Create a small numeric type that should be specialized
        let small_array = LuauNode::Array(Array {
            elements: vec![
                LuauNode::Number(Number::Integer(1)),
                LuauNode::Number(Number::Integer(2)),
            ],
        });

        // Generate code and check for specialization
        generator.generate_node(&small_array).expect("Failed to generate code");
        let code = generator.get_output();
        
        assert!(code.contains("specialized")); // Type specialization
        assert!(code.contains("inline")); // Inlining
    }

    #[test]
    fn test_vectorization_conditions() {
        let optimizer = RuntimeOptimizer::new();
        
        // Test numeric array
        let numeric_elements: Vec<LuauNode> = vec![
            LuauNode::Number(Number::Integer(1)),
            LuauNode::Number(Number::Integer(2)),
        ];
        let numeric_array = LuauNode::Array(Array { elements: numeric_elements });
        
        // Test mixed array
        let mixed_elements: Vec<LuauNode> = vec![
            LuauNode::Number(Number::Integer(1)),
            LuauNode::String(String::from("test")),
        ];
        let mixed_array = LuauNode::Array(Array { elements: mixed_elements });

        let mut generator = LuauCodeGenerator::new();
        
        // Numeric array should use vectorization
        generator.generate_node(&numeric_array).expect("Failed to generate code");
        let numeric_code = generator.get_output();
        assert!(numeric_code.contains("SimdHelpers"));

        // Mixed array should not use vectorization
        generator = LuauCodeGenerator::new();
        generator.generate_node(&mixed_array).expect("Failed to generate code");
        let mixed_code = generator.get_output();
        assert!(!mixed_code.contains("SimdHelpers"));
    }
}

/// Benchmark suite for optimization performance
#[cfg(test)]
mod benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_array_operations(b: &mut Bencher) {
        let mut generator = LuauCodeGenerator::new();
        let elements: Vec<LuauNode> = (0..100)
            .map(|i| LuauNode::Number(Number::Integer(i)))
            .collect();
        let array = LuauNode::Array(Array { elements });

        b.iter(|| {
            generator.generate_node(&array).expect("Failed to generate code");
        });
    }

    #[bench]
    fn bench_string_operations(b: &mut Bencher) {
        let optimizer = RuntimeOptimizer::new();
        let args = vec![
            String::from("\"Hello, World!\""),
            String::from("\", \""),
        ];

        b.iter(|| {
            optimizer.get_type_optimization("string", "concat", &args)
                .expect("Failed to get string optimization");
        });
    }

    #[bench]
    fn bench_number_operations(b: &mut Bencher) {
        let optimizer = RuntimeOptimizer::new();
        let args = vec![
            String::from("2"),
            String::from("3"),
        ];

        b.iter(|| {
            optimizer.get_type_optimization("number", "pow", &args)
                .expect("Failed to get number optimization");
        });
    }
}
