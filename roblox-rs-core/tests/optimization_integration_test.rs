use roblox_rs_core::tests::TestHelper;
use roblox_rs_core::ast::luau::{LuauNode, Table, Array, Number, String as LuauString};
use roblox_rs_core::codegen::LuauCodeGenerator;
use roblox_rs_core::runtime::optimized::RuntimeOptimizer;

#[test]
fn test_full_optimization_pipeline() {
    let mut helper = TestHelper::new();
    
    // Create a complex test case that exercises all optimizations
    let mut test_cases = helper.create_test_cases();
    let results = helper.run_test_suite();

    // Verify all test cases passed
    for result in results {
        assert!(result.success, "Test case failed");
        
        // Verify optimizations improved performance
        assert!(result.optimized_time < result.unoptimized_time, 
            "Optimization did not improve performance");
        
        // Verify memory optimizations were applied
        assert!(result.memory_optimized, 
            "Memory optimizations were not applied");
        
        // Check that appropriate optimizations were applied
        if result.vectorized {
            assert!(result.optimized_time.as_micros() < result.unoptimized_time.as_micros() * 2,
                "Vectorization did not provide sufficient speedup");
        }
        
        if result.specialized {
            assert!(result.optimized_time.as_micros() < result.unoptimized_time.as_micros() * 3,
                "Type specialization did not provide sufficient speedup");
        }
    }
}

#[test]
fn test_array_optimizations() {
    let mut helper = TestHelper::new();
    
    // Test numeric array optimizations
    let numeric_array = helper.create_numeric_array(100);
    assert!(helper.validate_vectorization(&numeric_array),
        "Numeric array was not vectorized");
        
    // Test string array optimizations
    let string_array = helper.create_string_array(100);
    assert!(helper.validate_memory_patterns(&string_array),
        "String array memory was not optimized");
}

#[test]
fn test_table_optimizations() {
    let mut helper = TestHelper::new();
    
    // Test table with various field types
    let table = helper.create_test_table(50);
    assert!(helper.validate_memory_patterns(&table),
        "Table memory was not optimized");
    assert!(helper.validate_type_specialization(&table),
        "Table was not specialized");
}

#[test]
fn test_optimization_stability() {
    let mut helper = TestHelper::new();
    
    // Run multiple iterations to ensure consistent optimization
    for _ in 0..10 {
        let results = helper.run_test_suite();
        
        for result in results {
            assert!(result.success, "Optimization stability test failed");
            assert!(result.optimized_time < result.unoptimized_time,
                "Optimization performance degraded");
        }
    }
}

#[test]
fn test_memory_usage() {
    let mut helper = TestHelper::new();
    
    // Create a large data structure
    let large_table = helper.create_test_table(1000);
    
    // Verify memory optimizations
    assert!(helper.validate_memory_patterns(&large_table),
        "Memory optimizations not applied to large table");
        
    // Generate code and verify memory tracking
    helper.generator.generate_node(&large_table).expect("Failed to generate code");
    let code = helper.generator.get_output();
    
    assert!(code.contains("MemoryManager:trackAllocation"),
        "Memory tracking not implemented");
    assert!(code.contains("table.create"),
        "Table pre-allocation not implemented");
}

#[test]
fn test_type_system_integration() {
    let mut helper = TestHelper::new();
    
    // Test various node types
    let nodes = vec![
        helper.create_numeric_array(10),
        helper.create_string_array(10),
        helper.create_test_table(10),
    ];
    
    for node in nodes {
        // Verify type-specific optimizations
        assert!(helper.validate_type_specialization(&node),
            "Type specialization not applied");
            
        // Generate code and verify type system integration
        helper.generator.generate_node(&node).expect("Failed to generate code");
        let code = helper.generator.get_output();
        
        assert!(code.contains("optimized") || code.contains("specialized"),
            "Type-specific optimizations not applied");
    }
}

#[test]
fn test_runtime_helper_generation() {
    let optimizer = RuntimeOptimizer::new();
    let code = optimizer.generate_runtime_code();
    
    // Verify all runtime helpers are present
    let required_helpers = [
        "SimdHelpers",
        "ArrayOptimizer",
        "NumberOptimizer",
        "StringOptimizer",
        "fastMap",
        "fastFilter",
        "fastReduce",
        "fastPow",
        "fastConcat",
    ];
    
    for helper in required_helpers.iter() {
        assert!(code.contains(helper),
            "Missing runtime helper: {}", helper);
    }
}

#[test]
fn test_optimization_combinations() {
    let mut helper = TestHelper::new();
    
    // Create a complex structure with multiple optimization opportunities
    let mut table_fields = Vec::new();
    
    // Add numeric array field
    table_fields.push((
        "numbers".to_string(),
        helper.create_numeric_array(50),
    ));
    
    // Add string array field
    table_fields.push((
        "strings".to_string(),
        helper.create_string_array(50),
    ));
    
    // Add nested table field
    table_fields.push((
        "nested".to_string(),
        helper.create_test_table(50),
    ));
    
    let complex_table = LuauNode::Table(Table { fields: table_fields });
    
    // Generate code and verify combined optimizations
    helper.generator.generate_node(&complex_table).expect("Failed to generate code");
    let code = helper.generator.get_output();
    
    // Verify all optimization types are applied
    assert!(code.contains("SimdHelpers"), "Missing SIMD optimizations");
    assert!(code.contains("ArrayOptimizer"), "Missing array optimizations");
    assert!(code.contains("StringOptimizer"), "Missing string optimizations");
    assert!(code.contains("MemoryManager"), "Missing memory optimizations");
    assert!(code.contains("specialized"), "Missing type specialization");
}
