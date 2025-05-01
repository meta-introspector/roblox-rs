use crate::ast::luau::{
    LuauNode, Program, Function, Table, Buffer,
    TypeAnnotation, PrimitiveType, BufferOptimizationLevel
};
use std::fmt::Write;

/// Generate Luau code from a Luau AST Program
pub fn generate(program: &Program) -> Result<String, String> {
    let mut generator = LuauCodeGenerator::new();
    generator.generate(program)
}

use crate::analysis::{CodeAnalyzer, OptimizationHint, MemoryPattern};
use crate::memory::{MemoryManager, default_memory_settings};
use crate::types::{TypeOptimizer, TypeOptimizationHint, TypeLayout};
use crate::runtime::optimized::RuntimeOptimizer;

use crate::debug::breakpoint::BreakpointManager;

pub struct LuauCodeGenerator {
    runtime_optimizer: RuntimeOptimizer,
    type_optimizer: TypeOptimizer,
    memory_manager: MemoryManager,
    indent_level: usize,
    output: String,
    analyzer: CodeAnalyzer,
    optimization_level: OptimizationLevel,
    breakpoint_manager: BreakpointManager,
}

#[derive(Debug, Clone, Copy)]
pub enum OptimizationLevel {
    None,
    Basic,    // Basic optimizations like inlining small functions
    Advanced, // Advanced optimizations including parallelization
    Extreme   // Aggressive optimizations that may affect debugging
}

#[derive(Debug, Clone)]
pub struct GeneratedCode {
    pub code: String,
    pub optimization_info: OptimizationInfo,
}

#[derive(Debug, Clone)]
pub struct OptimizationInfo {
    pub inlined_functions: Vec<String>,
    pub parallelized_loops: Vec<String>,
    pub vectorized_ops: Vec<String>,
    pub memory_patterns: HashMap<String, MemoryPattern>,
}

impl LuauCodeGenerator {
    pub fn new() -> Self {
        let runtime_optimizer = RuntimeOptimizer::new();
        let type_optimizer = TypeOptimizer::new();
        let memory_manager = MemoryManager::new(default_memory_settings());
        Self {
            indent_level: 0,
            output: String::new(),
            analyzer: CodeAnalyzer::new(),
            optimization_level: OptimizationLevel::Basic,
            breakpoint_manager: BreakpointManager::new(),
            memory_manager,
            type_optimizer,
            runtime_optimizer,
        }
    }

    pub fn with_optimization_level(level: OptimizationLevel) -> Self {
        Self {
            indent_level: 0,
            output: String::new(),
            analyzer: CodeAnalyzer::new(),
            optimization_level: level,
        }
    }
    
    /// Helper to generate output to a temporary buffer
    fn with_buffer<F>(&self, buffer: &mut String, f: F) -> Result<(), String>
    where
        F: FnOnce(&mut LuauCodeGenerator) -> Result<(), String>,
    {
        let mut temp_gen = LuauCodeGenerator {
            indent_level: 0,
            output: String::new(),
        };
        f(&mut temp_gen)?;
        buffer.push_str(&temp_gen.output);
        Ok(())
    }

    pub fn generate(&mut self, program: &Program) -> Result<GeneratedCode, String> {
        // First analyze the program
        for node in &program.body {
            self.analyzer.analyze_node(node)?;
        }

        // Add Luau optimization directives based on optimization level
        match self.optimization_level {
            OptimizationLevel::None => {
                writeln!(&mut self.output, "--!optimize 0").map_err(|e| e.to_string())?;
            }
            OptimizationLevel::Basic => {
                writeln!(&mut self.output, "--!optimize 1").map_err(|e| e.to_string())?;
                writeln!(&mut self.output, "--!native").map_err(|e| e.to_string())?;
            }
            OptimizationLevel::Advanced | OptimizationLevel::Extreme => {
                writeln!(&mut self.output, "--!optimize 2").map_err(|e| e.to_string())?;
                writeln!(&mut self.output, "--!native").map_err(|e| e.to_string())?;
                writeln!(&mut self.output, "--!strict").map_err(|e| e.to_string())?;
            }
        }
        writeln!(&mut self.output).map_err(|e| e.to_string())?;

        // Add runtime helpers based on optimization hints
        self.add_runtime_helpers()?;

        // Generate code for each node
        for node in &program.body {
            self.generate_node(node)?;
            writeln!(&mut self.output).map_err(|e| e.to_string())?;
        }

        // Create optimization info
        let optimization_info = OptimizationInfo {
            inlined_functions: self.get_inlined_functions(),
            parallelized_loops: self.get_parallelized_loops(),
            vectorized_ops: self.get_vectorized_ops(),
            memory_patterns: self.analyzer.get_memory_patterns().clone(),
        };

        Ok(GeneratedCode {
            code: self.output.clone(),
            optimization_info,
        })
    }

    fn add_runtime_helpers(&mut self) -> Result<(), String> {
        // Add optimized runtime helpers
        writeln!(&mut self.output, "-- Optimized Runtime Helpers").map_err(|e| e.to_string())?;
        writeln!(&mut self.output, "{}", self.runtime_optimizer.generate_runtime_code()).map_err(|e| e.to_string())?;

        // Add memory management code
        writeln!(&mut self.output, "-- Memory Management Implementation").map_err(|e| e.to_string())?;
        writeln!(&mut self.output, "{}", self.memory_manager.generate_memory_code()).map_err(|e| e.to_string())?;

        // Add object pool if needed
        let hints = self.analyzer.get_optimization_hints();
        if hints.iter().any(|h| matches!(h, OptimizationHint::UseObjectPool(_))) {
            let pool = ObjectPool::new(default_pool_config());
            writeln!(&mut self.output, "-- Object Pool Implementation").map_err(|e| e.to_string())?;
            writeln!(&mut self.output, "{}", pool.generate_pool_code()).map_err(|e| e.to_string())?;
        }

        let hints = self.analyzer.get_optimization_hints();
        let mut added_helpers = HashSet::new();

        for hint in hints {
            match hint {
                OptimizationHint::ParallelizeLoop(_) => {
                    if !added_helpers.contains("parallel") {
                        writeln!(&mut self.output, "
                            local function parallel_for(array, fn)
                                local threads = {{}}
                                local size = #array
                                local chunk_size = math.ceil(size / 4) -- Use 4 threads
                                
                                for i = 1, size, chunk_size do
                                    local thread = coroutine.create(function()
                                        local end_idx = math.min(i + chunk_size - 1, size)
                                        for j = i, end_idx do
                                            fn(array[j], j)
                                        end
                                    end)
                                    table.insert(threads, thread)
                                end
                                
                                for _, thread in ipairs(threads) do
                                    coroutine.resume(thread)
                                end
                            end
                        ").map_err(|e| e.to_string())?;
                        added_helpers.insert("parallel");
                    }
                },
                OptimizationHint::VectorizeOperation(_) => {
                    if !added_helpers.contains("vector") {
                        writeln!(&mut self.output, "
                            local function vector_op(a, b, op)
                                local result = table.create(#a)
                                for i = 1, #a do
                                    result[i] = op(a[i], b[i])
                                end
                                return result
                            end
                        ").map_err(|e| e.to_string())?;
                        added_helpers.insert("vector");
                    }
                },
                OptimizationHint::UseObjectPool(_) => {
                    if !added_helpers.contains("pool") {
                        writeln!(&mut self.output, "
                            local ObjectPool = {{}}
                            
                            function ObjectPool.new(factory, initial_size)
                                local pool = {{
                                    objects = table.create(initial_size),
                                    factory = factory,
                                    size = 0
                                }}
                                
                                for i = 1, initial_size do
                                    pool.objects[i] = factory()
                                    pool.size = pool.size + 1
                                end
                                
                                return pool
                            end
                            
                            function ObjectPool:acquire()
                                if self.size > 0 then
                                    self.size = self.size - 1
                                    return self.objects[self.size + 1]
                                end
                                return self.factory()
                            end
                            
                            function ObjectPool:release(obj)
                                self.size = self.size + 1
                                self.objects[self.size] = obj
                            end
                        ").map_err(|e| e.to_string())?;
                        added_helpers.insert("pool");
                    }
                },
                _ => {}
            }
        }

        Ok(())
    }

    fn get_inlined_functions(&self) -> Vec<String> {
        self.analyzer.get_optimization_hints()
            .iter()
            .filter_map(|hint| match hint {
                OptimizationHint::InlineFunction(name) => Some(name.clone()),
                _ => None,
            })
            .collect()
    }

    fn get_parallelized_loops(&self) -> Vec<String> {
        self.analyzer.get_optimization_hints()
            .iter()
            .filter_map(|hint| match hint {
                OptimizationHint::ParallelizeLoop(name) => Some(name.clone()),
                _ => None,
            })
            .collect()
    }

    fn get_vectorized_ops(&self) -> Vec<String> {
        self.analyzer.get_optimization_hints()
            .iter()
            .filter_map(|hint| match hint {
                OptimizationHint::VectorizeOperation(name) => Some(name.clone()),
                _ => None,
            })
            .collect()
    }

    fn generate_node(&mut self, node: &LuauNode) -> Result<(), String> {
        // Check for type optimizations
        let type_name = self.get_node_type(node);
        if let Some(type_name) = type_name {
            let hints = self.type_optimizer.analyze_type(&type_name, &self.analyzer.type_info);
            for hint in hints {
                match hint {
                    TypeOptimizationHint::UseSpecialization(_, specialized) => {
                        // Use specialized type implementation
                        return self.generate_specialized_node(node, &specialized);
                    }
                    TypeOptimizationHint::OptimizeLayout(_, layout_opt) => {
                        // Use optimized memory layout
                        if let Some(layout) = self.type_optimizer.analyze_layout(&type_name, &self.analyzer.type_info) {
                            return self.generate_optimized_layout(node, &layout);
                        }
                    }
                    TypeOptimizationHint::VectorizeOperations(_) => {
                        // Use vectorized operations
                        if self.can_vectorize_node(node) {
                            return self.generate_vectorized_node(node);
                        }
                    }
                    TypeOptimizationHint::InlineType(_) => {
                        // Inline the type implementation
                        return self.generate_inlined_node(node);
                    }
                }
            }
        }

        // Track memory allocations in debug mode
        if matches!(self.optimization_level, OptimizationLevel::None | OptimizationLevel::Basic) {
            match node {
                LuauNode::Table(table) => {
                    writeln!(&mut self.output, "MemoryManager:trackAllocation({})", table.fields.len() * 8)
                        .map_err(|e| e.to_string())?;
                }
                _ => {}
            }
        }

        // Check if we should use object pooling for this node
        let use_pool = self.analyzer.get_optimization_hints().iter().any(|h| {
            matches!(h, OptimizationHint::UseObjectPool(name) if matches!(node, LuauNode::Table(_)))
        });

        if use_pool {
            self.generate_pooled_node(node)
        } else {
            self.generate_regular_node(node)
        }
    }

    fn generate_pooled_node(&mut self, node: &LuauNode) -> Result<(), String> {
        match node {
            LuauNode::Table(table) => {
                writeln!(&mut self.output, "local obj = ObjectPool:acquire('table', {})", table.fields.len())
                    .map_err(|e| e.to_string())?;
                
                // Initialize table fields
                for (key, value) in &table.fields {
                    write!(&mut self.output, "obj[").map_err(|e| e.to_string())?;
                    self.generate_node(key)?;
                    write!(&mut self.output, "] = ").map_err(|e| e.to_string())?;
                    self.generate_node(value)?;
                    writeln!(&mut self.output).map_err(|e| e.to_string())?;
                }
                
                Ok(())
            }
            _ => self.generate_regular_node(node)
        }
    }

    fn generate_regular_node(&mut self, node: &LuauNode) -> Result<(), String> {
        // Add debug point before node execution if in debug mode
        if matches!(self.optimization_level, OptimizationLevel::None | OptimizationLevel::Basic) {
            self.add_debug_point()?;
        }

        match node {
            LuauNode::Function(func) => {
                // Check if function should be inlined
                if self.should_inline_function(func) {
                    self.generate_inlined_function(func)
                } else {
                    self.generate_function(func)
                }
            },
            LuauNode::Return(expr) => self.generate_return(expr),
            LuauNode::Local { name, value } => self.generate_local(name, value),
            LuauNode::If { condition, then_branch, else_branch } => self.generate_if(condition, then_branch, else_branch),
            LuauNode::Table(table) => {
                // Check if table should use object pooling
                if self.should_use_object_pool(table) {
                    self.generate_pooled_table(table)
                } else {
                    self.generate_table(table)
                }
            },
            LuauNode::Buffer(buffer) => self.generate_buffer(buffer),
            LuauNode::Program(program) => self.generate(program).map(|_| ()),
            LuauNode::Block(nodes) => {
                // Check if block can be parallelized
                if self.can_parallelize_block(nodes) {
                    self.generate_parallel_block(nodes)
                } else {
                    for node in nodes {
                        self.generate_node(node)?
                    }
                    Ok(())
                }
            },
            LuauNode::Binary { left, op, right } => {
                // Check if operation can be vectorized
                if self.can_vectorize_operation(left, right) {
                    self.generate_vectorized_operation(left, right, op)
                } else {
                    self.generate_node(left)?;
                    write!(&mut self.output, " {} ", op).map_err(|e| e.to_string())?;
                    self.generate_node(right)?;
                    Ok(())
                }
            },
            LuauNode::Identifier(name) => {
                write!(&mut self.output, "{}", name).map_err(|e| e.to_string())
            },
            LuauNode::Number(num) => write!(&mut self.output, "{}", num).map_err(|e| e.to_string()),
            LuauNode::String(s) => write!(&mut self.output, "\"{}\"", s).map_err(|e| e.to_string()),
            LuauNode::Boolean(b) => write!(&mut self.output, "{}", b).map_err(|e| e.to_string()),
        }
    }

    fn should_inline_function(&self, func: &Function) -> bool {
        self.analyzer.get_optimization_hints().iter().any(|hint| {
            matches!(hint, OptimizationHint::InlineFunction(name) if name == &func.name)
        })
    }

    fn generate_inlined_function(&mut self, func: &Function) -> Result<(), String> {
        // For inlined functions, we directly generate the body
        if let LuauNode::Block(nodes) = func.body.as_ref() {
            for node in nodes {
                self.generate_node(node)?
            }
        }
        Ok(())
    }

    fn should_use_object_pool(&self, table: &Table) -> bool {
        self.analyzer.get_optimization_hints().iter().any(|hint| {
            matches!(hint, OptimizationHint::UseObjectPool(_))
        })
    }

    fn generate_pooled_table(&mut self, table: &Table) -> Result<(), String> {
        write!(&mut self.output, "ObjectPool:acquire()").map_err(|e| e.to_string())
    }

    fn can_parallelize_block(&self, nodes: &[LuauNode]) -> bool {
        self.analyzer.get_optimization_hints().iter().any(|hint| {
            matches!(hint, OptimizationHint::ParallelizeLoop(_))
        })
    }

    fn generate_parallel_block(&mut self, nodes: &[LuauNode]) -> Result<(), String> {
        writeln!(&mut self.output, "parallel_for(array, function(item, index)")?;
        self.indent_level += 1;
        for node in nodes {
            self.write_indent()?;
            self.generate_node(node)?
        }
        self.indent_level -= 1;
        writeln!(&mut self.output, "end)")?;
        Ok(())
    }

    fn can_vectorize_operation(&self, left: &LuauNode, right: &LuauNode) -> bool {
        self.analyzer.get_optimization_hints().iter().any(|hint| {
            matches!(hint, OptimizationHint::VectorizeOperation(_))
        })
    }

    fn generate_vectorized_operation(&mut self, left: &LuauNode, right: &LuauNode, op: &str) -> Result<(), String> {
        write!(&mut self.output, "vector_op(").map_err(|e| e.to_string())?;
        self.generate_node(left)?;
        write!(&mut self.output, ", ").map_err(|e| e.to_string())?;
        self.generate_node(right)?;
        write!(&mut self.output, ", function(a, b) return a {} b end)", op).map_err(|e| e.to_string())
    }

    fn generate_if(&mut self, condition: &LuauNode, then_branch: &LuauNode, else_branch: &Option<Box<LuauNode>>) -> Result<(), String> {
        write!(&mut self.output, "if ").map_err(|e| e.to_string())?;
        self.generate_node(condition)?;
        writeln!(&mut self.output, " then").map_err(|e| e.to_string())?;
        self.generate_node(then_branch)?;
        
        if let Some(else_branch) = else_branch {
            writeln!(&mut self.output, "else").map_err(|e| e.to_string())?;
            self.generate_node(else_branch)?;
        }
        
        writeln!(&mut self.output, "end").map_err(|e| e.to_string())
    }

    fn generate_return(&mut self, expr: &LuauNode) -> Result<(), String> {
        write!(&mut self.output, "return ").map_err(|e| e.to_string())?;
        self.generate_node(expr)?;
        writeln!(&mut self.output).map_err(|e| e.to_string())
    }

    fn generate_local(&mut self, name: &str, value: &LuauNode) -> Result<(), String> {
        write!(&mut self.output, "local {} = ", name).map_err(|e| e.to_string())?;
        self.generate_node(value)?;
        writeln!(&mut self.output).map_err(|e| e.to_string())
    }

    fn add_debug_point(&mut self) -> Result<(), String> {
        writeln!(&mut self.output, "
            if RobloxRS and RobloxRS.Debug then
                local locals = {{}}
                -- Capture local variables in scope
                local function capture_locals()
                    local variables = {{}}
                    local i = 1
                    while true do
                        local name, value = debug.getlocal(2, i)
                        if not name then break end
                        variables[name] = value
                        i = i + 1
                    end
                    return variables
                end
                locals = capture_locals()
                RobloxRS.Debug.check_breakpoint(debug.getinfo(1).currentline, locals)
            end
        ").map_err(|e| e.to_string())
    }

    fn generate_function(&mut self, func: &Function) -> Result<(), String> {
        // Add performance hints as comments
        writeln!(&mut self.output, "-- Performance optimized function").map_err(|e| e.to_string())?;
        if let Some(pattern) = self.analyzer.get_memory_patterns().get(&func.name) {
            writeln!(&mut self.output, "-- Memory usage: {} bytes peak", pattern.peak_memory).map_err(|e| e.to_string())?;
            writeln!(&mut self.output, "-- Average lifetime: {} seconds", pattern.average_lifetime).map_err(|e| e.to_string())?;
        }

        write!(&mut self.output, "function {}(", func.name).map_err(|e| e.to_string())?;
        
        // Generate parameters with type checks in debug mode
        for (i, param) in func.params.iter().enumerate() {
            if i > 0 {
                write!(&mut self.output, ", ").map_err(|e| e.to_string())?;
            }
            write!(&mut self.output, "{}", param.name).map_err(|e| e.to_string())?;
            
            // Add type annotation and runtime check if in debug mode
            if let Some(type_ann) = &param.type_annotation {
                write!(&mut self.output, ": {}", self.type_to_string(type_ann)).map_err(|e| e.to_string())?;
                
                if matches!(self.optimization_level, OptimizationLevel::None | OptimizationLevel::Basic) {
                    writeln!(&mut self.output, "
                        assert(type({}) == '{}', 'Expected {} to be {}, got ' .. type({}))
                    ", param.name, self.type_to_string(type_ann), param.name, self.type_to_string(type_ann), param.name)
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        
        writeln!(&mut self.output, ")").map_err(|e| e.to_string())?;
        
        // Add profiling if in debug mode
        if matches!(self.optimization_level, OptimizationLevel::None | OptimizationLevel::Basic) {
            writeln!(&mut self.output, "    local __start_time = os.clock()").map_err(|e| e.to_string())?;
        }

        // Generate optimized function body
        self.indent_level += 1;

        // Add local variable caching for frequently accessed values
        if let LuauNode::Block(nodes) = func.body.as_ref() {
            let mut cached_vars = HashSet::new();
            for hint in self.analyzer.get_optimization_hints() {
                if let OptimizationHint::CacheValue(var_name) = hint {
                    if !cached_vars.contains(var_name) {
                        writeln!(&mut self.output, "    local _{}_cache = {}", var_name, var_name)
                            .map_err(|e| e.to_string())?;
                        cached_vars.insert(var_name);
                    }
                }
            }

            // Generate the actual body
            self.generate_node(&func.body)?;
        }

        self.indent_level -= 1;
        
        // Add profiling output if in debug mode
        if matches!(self.optimization_level, OptimizationLevel::None | OptimizationLevel::Basic) {
            writeln!(&mut self.output, "    local __end_time = os.clock()").map_err(|e| e.to_string())?;
            writeln!(&mut self.output, "    print('{} execution time:', __end_time - __start_time)", func.name)
                .map_err(|e| e.to_string())?;
        }
        
        writeln!(&mut self.output, "end").map_err(|e| e.to_string())
    }

    fn get_node_type(&self, node: &LuauNode) -> Option<String> {
        match node {
            LuauNode::Table(table) => Some("table".to_string()),
            LuauNode::Array(elements) => Some("array".to_string()),
            LuauNode::Number(_) => Some("number".to_string()),
            LuauNode::String(_) => Some("string".to_string()),
            LuauNode::Boolean(_) => Some("boolean".to_string()),
            LuauNode::Function(func) => Some(format!("function_{}", func.name)),
            _ => None,
        }
    }

    fn generate_specialized_node(&mut self, node: &LuauNode, specialized_type: &str) -> Result<(), String> {
        // Generate code using specialized type implementation
        writeln!(&mut self.output, "-- Using specialized type: {}", specialized_type)
            .map_err(|e| e.to_string())?;
        self.generate_regular_node(node)
    }

    fn generate_optimized_layout(&mut self, node: &LuauNode, layout: &TypeLayout) -> Result<(), String> {
        // Generate code using optimized memory layout
        writeln!(&mut self.output, "-- Using optimized layout")
            .map_err(|e| e.to_string())?;
        
        match node {
            LuauNode::Table(table) => {
                // Generate table with optimized field layout
                let code = self.type_optimizer.generate_type_code(&self.get_node_type(node).unwrap_or_default(), layout);
                writeln!(&mut self.output, "{}", code)
                    .map_err(|e| e.to_string())
            }
            _ => self.generate_regular_node(node)
        }
    }

    fn can_vectorize_node(&self, node: &LuauNode) -> bool {
        match node {
            LuauNode::Array(elements) => {
                // Check if array elements are numeric
                elements.iter().all(|e| matches!(e, LuauNode::Number(_)))
            }
            LuauNode::Binary { left, right, .. } => {
                // Check if operation can be vectorized
                matches!(left.as_ref(), LuauNode::Array(_)) && 
                matches!(right.as_ref(), LuauNode::Array(_))
            }
            _ => false
        }
    }

    fn generate_vectorized_node(&mut self, node: &LuauNode) -> Result<(), String> {
        writeln!(&mut self.output, "-- Using vectorized operations with runtime helpers")
            .map_err(|e| e.to_string())?;
        
        match node {
            LuauNode::Array(elements) => {
                if elements.iter().all(|e| matches!(e, LuauNode::Number(_))) {
                    // Use SIMD helpers for numeric arrays
                    writeln!(&mut self.output, "local result = SimdHelpers.mapChunks({{", )
                        .map_err(|e| e.to_string())?;
                    
                    // Generate array elements
                    for (i, element) in elements.iter().enumerate() {
                        if i > 0 {
                            write!(&mut self.output, ", ").map_err(|e| e.to_string())?;
                        }
                        self.generate_node(element)?;
                    }
                    
                    writeln!(&mut self.output, "}}, 4, function(chunk)")
                        .map_err(|e| e.to_string())?;
                    writeln!(&mut self.output, "    return SimdHelpers.mul4(unpack(chunk))")
                        .map_err(|e| e.to_string())?;
                    writeln!(&mut self.output, "end)")
                        .map_err(|e| e.to_string())
                } else {
                    // Fall back to regular array processing
                    self.generate_regular_node(node)
                }
            }
            LuauNode::Binary { left, right, operator } => {
                if let (LuauNode::Array(_), LuauNode::Array(_)) = (left.as_ref(), right.as_ref()) {
                    // Use array optimizer for binary operations
                    writeln!(&mut self.output, "ArrayOptimizer.fastMap({{", )
                        .map_err(|e| e.to_string())?;
                    self.generate_node(left.as_ref())?;
                    writeln!(&mut self.output, "}}, function(x)")
                        .map_err(|e| e.to_string())?;
                    writeln!(&mut self.output, "    return x {} ", operator)
                        .map_err(|e| e.to_string())?;
                    self.generate_node(right.as_ref())?;
                    writeln!(&mut self.output, "end)")
                        .map_err(|e| e.to_string())
                } else {
                    self.generate_regular_node(node)
                }
            }
            _ => self.generate_regular_node(node)
        }
        writeln!(&mut self.output, "-- Using vectorized operations")
            .map_err(|e| e.to_string())?;
        
        match node {
            LuauNode::Array(elements) => {
                // Generate SIMD-style operations for arrays
                writeln!(&mut self.output, "local result = table.create({})\n", elements.len())
                    .map_err(|e| e.to_string())?;
                
                // Process elements in chunks of 4 for vectorization
                for chunk in elements.chunks(4) {
                    let chunk_code = chunk.iter()
                        .map(|e| self.generate_node(e))
                        .collect::<Result<Vec<_>, _>>()?;
                    writeln!(&mut self.output, "-- Vectorized chunk: {}", chunk_code.join(", "))
                        .map_err(|e| e.to_string())?;
                }
                
                Ok(())
            }
            _ => self.generate_regular_node(node)
        }
    }

    fn generate_inlined_node(&mut self, node: &LuauNode) -> Result<(), String> {
        writeln!(&mut self.output, "-- Using inlined implementation")
            .map_err(|e| e.to_string())?;
        self.generate_regular_node(node)
    }

    fn generate_table(&mut self, table: &Table) -> Result<(), String> {
        // Use pre-allocation for tables when enabled
        if self.memory_manager.settings.pre_allocate_tables {
            writeln!(&mut self.output, "local t = MemoryManager:preAllocateTable({})", table.fields.len())
                .map_err(|e| e.to_string())?;
        } else {
            writeln!(&mut self.output, "local t = {{}}").map_err(|e| e.to_string())?;
        }

        // Check for optimization hints
        if let Some(buffer_type) = &table.optimization_hints.native_buffer_type {
            match buffer_type.as_str() {
                "Vector3" => {
                    // Extract x, y, z values
                    let mut x_val = "0".to_string();
                    let mut y_val = "0".to_string();
                    let mut z_val = "0".to_string();
                    
                    for (key, value) in &table.fields {
                        if key == "x" {
                            let mut x_buf = String::new();
                            self.with_buffer(&mut x_buf, |generator| generator.generate_node(value))?;
                            x_val = x_buf.clone();
                        } else if key == "y" {
                            let mut y_buf = String::new();
                            self.with_buffer(&mut y_buf, |generator| generator.generate_node(value))?;
                            y_val = y_buf.clone();
                        } else if key == "z" {
                            let mut z_buf = String::new();
                            self.with_buffer(&mut z_buf, |generator| generator.generate_node(value))?;
                            z_val = z_buf.clone();
                        }
                    }
                    
                    // Generate Vector3.new(x, y, z)
                    write!(&mut self.output, "Vector3.new({}, {}, {})", x_val, y_val, z_val).map_err(|e| e.to_string())?;
                    return Ok(());
                },
                "Color3" => {
                    // Extract r, g, b values
                    let mut r_val = "0".to_string();
                    let mut g_val = "0".to_string();
                    let mut b_val = "0".to_string();
                    
                    for (key, value) in &table.fields {
                        if key == "r" {
                            let mut r_buf = String::new();
                            self.with_buffer(&mut r_buf, |generator| generator.generate_node(value))?;
                            r_val = r_buf.clone();
                        } else if key == "g" {
                            let mut g_buf = String::new();
                            self.with_buffer(&mut g_buf, |generator| generator.generate_node(value))?;
                            g_val = g_buf.clone();
                        } else if key == "b" {
                            let mut b_buf = String::new();
                            self.with_buffer(&mut b_buf, |generator| generator.generate_node(value))?;
                            b_val = b_buf.clone();
                        }
                    }
                    
                    // Generate Color3.new(r, g, b)
                    write!(&mut self.output, "Color3.new({}, {}, {})", r_val, g_val, b_val).map_err(|e| e.to_string())?;
                    return Ok(());
                },
                _ => {
                    return self.generate_standard_table(table);
                }
            }
        } else {
            return self.generate_standard_table(table);
        }
    }

    fn generate_standard_table(&mut self, table: &Table) -> Result<(), String> {
        // Check if we should use table.create for array-like tables
        if table.optimization_hints.array_like && table.optimization_hints.pre_allocate.is_some() {
            let size = table.optimization_hints.pre_allocate.unwrap_or(0);
            write!(&mut self.output, "table.create({})", size).map_err(|e| e.to_string())?;
            return Ok(());
        }
        
        write!(&mut self.output, "{{").map_err(|e| e.to_string())?;
        
        for (i, (key, value)) in table.fields.iter().enumerate() {
            // Add comma for all but the first element
            if i > 0 {
                write!(&mut self.output, ", ").map_err(|e| e.to_string())?;
            }
            
            // Check if key is a numeric index or a string key
            if key.parse::<usize>().is_ok() {
                // If it's a numeric key, just generate the value in array notation
                self.generate_node(value)?;
            } else {
                // Generate the key-value pair with string keys
                write!(&mut self.output, "{} = ", key).map_err(|e| e.to_string())?;
                self.generate_node(value)?;
            }
        }
        
        write!(&mut self.output, "}}").map_err(|e| e.to_string())
    }

    fn generate_buffer(&mut self, buffer: &Buffer) -> Result<(), String> {
        // Use buffer type with optimization hints
        match buffer.optimization_level {
            BufferOptimizationLevel::Speed => {
                write!(&mut self.output, "buffer.create({}, {{speed=true}})", 
                    buffer.initial_size).map_err(|e| e.to_string())?;
            }
            BufferOptimizationLevel::Size => {
                write!(&mut self.output, "buffer.create({}, {{compact=true}})", 
                    buffer.initial_size).map_err(|e| e.to_string())?;
            }
            BufferOptimizationLevel::Default => {
                write!(&mut self.output, "buffer.create({})", 
                    buffer.initial_size).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    fn type_to_string(&self, type_ann: &TypeAnnotation) -> String {
        match type_ann {
            TypeAnnotation::Number => "number".to_string(),
            TypeAnnotation::String => "string".to_string(),
            TypeAnnotation::Boolean => "boolean".to_string(),
            TypeAnnotation::Any => "any".to_string(),
            TypeAnnotation::Custom(name) => name.clone(),
            TypeAnnotation::Primitive(p) => format!("{:?}", p),
            TypeAnnotation::Table(_) => "table".to_string(),
            TypeAnnotation::Buffer(_) => "buffer".to_string(),
            TypeAnnotation::Function(_) => "function".to_string(),
        }
    }

    fn write_indent(&mut self) -> Result<(), String> {
        write!(&mut self.output, "{}", "    ".repeat(self.indent_level))
            .map_err(|e| e.to_string())
    }
}
