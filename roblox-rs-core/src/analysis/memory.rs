use std::collections::{HashMap, HashSet};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct LifetimeTracker {
    // Track when variables are created
    creation_points: HashMap<String, CreationPoint>,
    // Track when variables are last used
    last_usage: HashMap<String, Usage>,
    // Track variable scopes
    scope_stack: Vec<Scope>,
    // Track current function depth
    current_depth: usize,
}

#[derive(Debug, Clone)]
pub struct CreationPoint {
    pub line: usize,
    pub scope_depth: usize,
    pub allocation_size: usize,
    pub type_info: String,
}

#[derive(Debug, Clone)]
pub struct Usage {
    pub line: usize,
    pub access_type: AccessType,
    pub frequency: usize,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub depth: usize,
    pub variables: HashSet<String>,
    pub start_line: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum AccessType {
    Read,
    Write,
    Both,
}

impl LifetimeTracker {
    pub fn new() -> Self {
        Self {
            creation_points: HashMap::new(),
            last_usage: HashMap::new(),
            scope_stack: vec![Scope {
                depth: 0,
                variables: HashSet::new(),
                start_line: 0,
            }],
            current_depth: 0,
        }
    }

    pub fn enter_scope(&mut self, line: usize) {
        self.current_depth += 1;
        self.scope_stack.push(Scope {
            depth: self.current_depth,
            variables: HashSet::new(),
            start_line: line,
        });
    }

    pub fn exit_scope(&mut self, line: usize) -> Vec<String> {
        if let Some(scope) = self.scope_stack.pop() {
            self.current_depth -= 1;
            
            // Find variables that can be freed
            let mut freed_vars = Vec::new();
            for var in scope.variables {
                if let Some(usage) = self.last_usage.get(&var) {
                    if usage.line <= line {
                        freed_vars.push(var.clone());
                    }
                }
            }
            freed_vars
        } else {
            Vec::new()
        }
    }

    pub fn track_creation(&mut self, var_name: String, line: usize, size: usize, type_info: String) {
        let creation = CreationPoint {
            line,
            scope_depth: self.current_depth,
            allocation_size: size,
            type_info,
        };
        self.creation_points.insert(var_name.clone(), creation);
        
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.variables.insert(var_name);
        }
    }

    pub fn track_usage(&mut self, var_name: &str, line: usize, access: AccessType) {
        let usage = self.last_usage.entry(var_name.to_string()).or_insert(Usage {
            line,
            access_type: access,
            frequency: 0,
        });
        
        usage.line = line;
        usage.frequency += 1;
        usage.access_type = match (usage.access_type, access) {
            (_, AccessType::Both) => AccessType::Both,
            (AccessType::Both, _) => AccessType::Both,
            (AccessType::Read, AccessType::Write) => AccessType::Both,
            (AccessType::Write, AccessType::Read) => AccessType::Both,
            _ => access,
        };
    }

    pub fn get_lifetime(&self, var_name: &str) -> Option<Duration> {
        if let (Some(creation), Some(last_use)) = (
            self.creation_points.get(var_name),
            self.last_usage.get(var_name)
        ) {
            // Convert lines to approximate duration
            // Assuming each line takes about 1ms to execute
            Some(Duration::from_millis((last_use.line - creation.line) as u64))
        } else {
            None
        }
    }

    pub fn get_allocation_info(&self, var_name: &str) -> Option<&CreationPoint> {
        self.creation_points.get(var_name)
    }

    pub fn get_usage_pattern(&self, var_name: &str) -> Option<&Usage> {
        self.last_usage.get(var_name)
    }

    pub fn get_current_scope_vars(&self) -> HashSet<String> {
        self.scope_stack
            .last()
            .map(|scope| scope.variables.clone())
            .unwrap_or_default()
    }

    pub fn analyze_memory_patterns(&self) -> Vec<MemoryPattern> {
        let mut patterns = Vec::new();
        let mut var_groups: HashMap<String, Vec<(String, &CreationPoint)>> = HashMap::new();

        // Group variables by type
        for (var, creation) in &self.creation_points {
            var_groups
                .entry(creation.type_info.clone())
                .or_default()
                .push((var.clone(), creation));
        }

        // Analyze patterns for each type
        for (type_name, vars) in var_groups {
            let total_size: usize = vars.iter().map(|(_, c)| c.allocation_size).sum();
            let avg_lifetime = vars
                .iter()
                .filter_map(|(var, _)| self.get_lifetime(var))
                .map(|d| d.as_millis() as f64)
                .sum::<f64>() / vars.len() as f64;

            patterns.push(MemoryPattern {
                type_name,
                allocation_count: vars.len(),
                total_size,
                average_lifetime: Duration::from_millis(avg_lifetime as u64),
                reuse_potential: self.calculate_reuse_potential(&vars),
            });
        }

        patterns
    }

    fn calculate_reuse_potential(&self, vars: &[(String, &CreationPoint)]) -> f64 {
        let mut overlapping = 0;
        let total = vars.len();

        for (i, (var1, _)) in vars.iter().enumerate() {
            for (var2, _) in vars.iter().skip(i + 1) {
                if self.lifetimes_overlap(var1, var2) {
                    overlapping += 1;
                }
            }
        }

        1.0 - (overlapping as f64 / total as f64)
    }

    fn lifetimes_overlap(&self, var1: &str, var2: &str) -> bool {
        let (c1, u1) = match (self.creation_points.get(var1), self.last_usage.get(var1)) {
            (Some(c), Some(u)) => (c.line, u.line),
            _ => return false,
        };

        let (c2, u2) = match (self.creation_points.get(var2), self.last_usage.get(var2)) {
            (Some(c), Some(u)) => (c.line, u.line),
            _ => return false,
        };

        // Check if lifetimes overlap
        !(u1 < c2 || u2 < c1)
    }
}

#[derive(Debug)]
pub struct MemoryPattern {
    pub type_name: String,
    pub allocation_count: usize,
    pub total_size: usize,
    pub average_lifetime: Duration,
    pub reuse_potential: f64,
}
