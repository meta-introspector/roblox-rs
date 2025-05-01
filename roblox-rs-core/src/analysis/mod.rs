use std::collections::{HashMap, HashSet};
use crate::ast::luau::{LuauNode, Function, Table};
use crate::types::TypeMapper;

/// Advanced code analyzer for Rust-to-Luau compilation
mod memory;
use memory::{LifetimeTracker, AccessType};

pub struct CodeAnalyzer {
    // Track variable dependencies
    variable_deps: HashMap<String, HashSet<String>>,
    // Track function calls
    call_graph: HashMap<String, HashSet<String>>,
    // Track memory usage patterns
    memory_patterns: HashMap<String, MemoryPattern>,
    // Track type information
    type_info: TypeMapper,
    // Track optimization opportunities
    optimization_hints: Vec<OptimizationHint>,
    lifetime_tracker: LifetimeTracker,
}

#[derive(Debug, Clone)]
pub struct MemoryPattern {
    pub allocation_sites: Vec<AllocationSite>,
    pub peak_memory: usize,
    pub average_lifetime: f64,
    pub reuse_opportunities: Vec<ReuseOpportunity>,
}

#[derive(Debug, Clone)]
pub struct AllocationSite {
    pub location: String,
    pub size: usize,
    pub frequency: usize,
    pub type_name: String,
}

#[derive(Debug, Clone)]
pub struct ReuseOpportunity {
    pub variable_name: String,
    pub potential_savings: usize,
    pub confidence: f64,
}

#[derive(Debug, Clone)]
pub enum OptimizationHint {
    InlineFunction(String),
    CacheValue(String),
    UseObjectPool(String),
    ParallelizeLoop(String),
    VectorizeOperation(String),
}

impl CodeAnalyzer {
    pub fn new() -> Self {
        Self {
            variable_deps: HashMap::new(),
            call_graph: HashMap::new(),
            memory_patterns: HashMap::new(),
            type_info: TypeMapper::new(),
            optimization_hints: Vec::new(),
            lifetime_tracker: LifetimeTracker::new(),
        }
    }

    /// Analyze a Luau AST node and gather optimization information
    pub fn analyze_node(&mut self, node: &LuauNode) -> Result<(), String> {
        match node {
            LuauNode::Function(func) => self.analyze_function(func)?,
            LuauNode::Block(nodes) => {
                for node in nodes {
                    self.analyze_node(node)?;
                }
            }
            LuauNode::Binary { left, right, .. } => {
                self.analyze_node(left)?;
                self.analyze_node(right)?;
                self.check_vectorization_opportunity(left, right);
            }
            LuauNode::Call { func, args } => {
                self.analyze_node(func)?;
                for arg in args {
                    self.analyze_node(arg)?;
                }
                self.update_call_graph(func, args);
            }
            LuauNode::Table(table) => self.analyze_table(table)?,
            _ => {}
        }
        Ok(())
    }

    /// Analyze a function for optimization opportunities
    fn analyze_function(&mut self, func: &Function) -> Result<(), String> {
        // Enter new scope for function
        self.lifetime_tracker.enter_scope(func.span.start.line);
        // Check if function should be inlined
        if self.should_inline(func) {
            self.optimization_hints.push(OptimizationHint::InlineFunction(func.name.clone()));
        }

        // Analyze function body
        if let LuauNode::Block(nodes) = func.body.as_ref() {
            // Track variable dependencies
            self.analyze_variable_dependencies(nodes);

            // Look for loop parallelization opportunities
            self.find_parallel_loops(nodes);

            // Analyze memory usage
            self.analyze_memory_usage(nodes, &func.name);
        }

        // Exit function scope and get freed variables
        let freed_vars = self.lifetime_tracker.exit_scope(func.span.end.line);
        
        // Add optimization hints for freed variables
        for var in freed_vars {
            if let Some(pattern) = self.lifetime_tracker.get_usage_pattern(&var) {
                if pattern.frequency > 5 {
                    self.optimization_hints.push(OptimizationHint::CacheValue(var));
                }
            }
        }

        Ok(())
    }

    /// Determine if a function should be inlined
    fn should_inline(&self, func: &Function) -> bool {
        // Check function size
        if let LuauNode::Block(nodes) = func.body.as_ref() {
            // Inline if function is small and called frequently
            if nodes.len() <= 3 && self.get_call_frequency(&func.name) > 5 {
                return true;
            }
        }
        false
    }

    /// Get how often a function is called
    fn get_call_frequency(&self, func_name: &str) -> usize {
        self.call_graph
            .get(func_name)
            .map(|callers| callers.len())
            .unwrap_or(0)
    }

    /// Analyze variable dependencies in a block
    fn analyze_variable_dependencies(&mut self, nodes: &[LuauNode]) {
        let mut current_line = 0; // In real impl, get from node span
        for node in nodes {
            match node {
                LuauNode::Binary { left, right, .. } => {
                    if let LuauNode::Identifier(var_name) = left.as_ref() {
                        let deps = self.variable_deps.entry(var_name.clone()).or_default();
                        // Track variable write
                        self.lifetime_tracker.track_usage(var_name, current_line, AccessType::Write);
                        if let LuauNode::Identifier(dep_name) = right.as_ref() {
                            deps.insert(dep_name.clone());
                            // Track variable read
                            self.lifetime_tracker.track_usage(dep_name, current_line, AccessType::Read);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Look for loops that can be parallelized
    fn find_parallel_loops(&mut self, nodes: &[LuauNode]) {
        for (i, node) in nodes.iter().enumerate() {
            if let LuauNode::Block(block_nodes) = node {
                // Check if this block represents a loop
                if self.is_parallelizable_loop(block_nodes) {
                    self.optimization_hints.push(OptimizationHint::ParallelizeLoop(
                        format!("loop_{}", i)
                    ));
                }
            }
        }
    }

    /// Check if a loop can be parallelized
    fn is_parallelizable_loop(&self, nodes: &[LuauNode]) -> bool {
        // Check for loop pattern
        if nodes.len() < 2 {
            return false;
        }

        // Check for independent iterations (no cross-iteration dependencies)
        let mut writes = HashSet::new();
        let mut reads = HashSet::new();

        for node in nodes {
            self.collect_access_patterns(node, &mut writes, &mut reads);
        }

        // Loop is parallelizable if there are no overlapping reads and writes
        writes.is_disjoint(&reads)
    }

    /// Collect variable access patterns
    fn collect_access_patterns(&self, node: &LuauNode, writes: &mut HashSet<String>, reads: &mut HashSet<String>) {
        match node {
            LuauNode::Binary { left, right, .. } => {
                if let LuauNode::Identifier(var_name) = left.as_ref() {
                    writes.insert(var_name.clone());
                }
                if let LuauNode::Identifier(var_name) = right.as_ref() {
                    reads.insert(var_name.clone());
                }
            }
            LuauNode::Call { args, .. } => {
                for arg in args {
                    if let LuauNode::Identifier(var_name) = arg {
                        reads.insert(var_name.clone());
                    }
                }
            }
            _ => {}
        }
    }

    /// Check for vectorization opportunities
    fn check_vectorization_opportunity(&mut self, left: &Box<LuauNode>, right: &Box<LuauNode>) {
        // Check if operation can be vectorized (e.g., array operations)
        if let (LuauNode::Table(left_table), LuauNode::Table(right_table)) = (left.as_ref(), right.as_ref()) {
            if self.is_numeric_array(left_table) && self.is_numeric_array(right_table) {
                self.optimization_hints.push(OptimizationHint::VectorizeOperation(
                    "array_operation".to_string()
                ));
            }
        }
    }

    /// Check if a table represents a numeric array
    fn is_numeric_array(&self, table: &Table) -> bool {
        table.fields.iter().all(|(_, value)| {
            matches!(value, LuauNode::Number(_))
        })
    }

    /// Analyze table for optimization opportunities
    fn analyze_table(&mut self, table: &Table) -> Result<(), String> {
        // Check if table should use object pooling
        if self.should_use_object_pool(table) {
            self.optimization_hints.push(OptimizationHint::UseObjectPool(
                "table_pool".to_string()
            ));
        }

        // Analyze memory usage patterns
        let pattern = self.analyze_table_memory_pattern(table);
        self.memory_patterns.insert("table".to_string(), pattern);

        Ok(())
    }

    /// Determine if a table should use object pooling
    fn should_use_object_pool(&self, table: &Table) -> bool {
        // Check table size and allocation frequency
        table.fields.len() > 10 && table.optimization_hints.pre_allocate.unwrap_or(0) > 0
    }

    /// Analyze memory usage patterns for a table
    fn analyze_table_memory_pattern(&self, table: &Table) -> MemoryPattern {
        MemoryPattern {
            allocation_sites: vec![AllocationSite {
                location: "table_creation".to_string(),
                size: table.fields.len() * std::mem::size_of::<LuauNode>(),
                frequency: 1,
                type_name: "table".to_string(),
            }],
            peak_memory: table.fields.len() * std::mem::size_of::<LuauNode>(),
            average_lifetime: 0.0, // Would be calculated from runtime data
            reuse_opportunities: vec![],
        }
    }

    /// Update the call graph with a new function call
    fn update_call_graph(&mut self, func: &Box<LuauNode>, args: &[LuauNode]) {
        if let LuauNode::Identifier(func_name) = func.as_ref() {
            let callers = self.call_graph.entry(func_name.clone()).or_default();
            // Add current function as a caller
            callers.insert("current_function".to_string()); // In real impl, would track current function
        }
    }

    /// Get optimization hints
    pub fn get_optimization_hints(&self) -> &[OptimizationHint] {
        &self.optimization_hints
    }

    /// Get memory usage patterns
    pub fn get_memory_patterns(&self) -> &HashMap<String, MemoryPattern> {
        &self.memory_patterns
    }

    /// Analyze memory usage in a block of code
    fn analyze_memory_usage(&mut self, nodes: &[LuauNode], context: &str) {
        let mut pattern = MemoryPattern {
            allocation_sites: Vec::new(),
            peak_memory: 0,
            average_lifetime: 0.0,
            reuse_opportunities: Vec::new(),
        };

        for node in nodes {
            if let LuauNode::Table(table) = node {
                let site = AllocationSite {
                    location: context.to_string(),
                    size: table.fields.len() * std::mem::size_of::<LuauNode>(),
                    frequency: 1,
                    type_name: "table".to_string(),
                };
                pattern.allocation_sites.push(site);
                pattern.peak_memory += site.size;
            }
        }

        // Look for reuse opportunities
        for (var_name, deps) in &self.variable_deps {
            if deps.is_empty() && pattern.peak_memory > 1000 {
                pattern.reuse_opportunities.push(ReuseOpportunity {
                    variable_name: var_name.clone(),
                    potential_savings: pattern.peak_memory,
                    confidence: 0.8,
                });
            }
        }

        self.memory_patterns.insert(context.to_string(), pattern);
    }
}
