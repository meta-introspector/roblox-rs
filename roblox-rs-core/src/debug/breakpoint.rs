use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub line: usize,
    pub condition: Option<String>,
    pub hit_count: usize,
    pub watch_expressions: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug)]
pub struct BreakpointManager {
    breakpoints: HashMap<usize, Breakpoint>,
    watch_variables: HashMap<String, WatchInfo>,
    current_line: usize,
    current_locals: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct WatchInfo {
    pub expression: String,
    pub last_value: Option<String>,
    pub condition: Option<String>,
}

impl BreakpointManager {
    pub fn new() -> Self {
        Self {
            breakpoints: HashMap::new(),
            watch_variables: HashMap::new(),
            current_line: 0,
            current_locals: HashMap::new(),
        }
    }

    pub fn add_breakpoint(&mut self, line: usize, condition: Option<String>) {
        let breakpoint = Breakpoint {
            line,
            condition,
            hit_count: 0,
            watch_expressions: Vec::new(),
            enabled: true,
        };
        self.breakpoints.insert(line, breakpoint);
    }

    pub fn add_watch(&mut self, variable: String, condition: Option<String>) {
        let watch = WatchInfo {
            expression: variable.clone(),
            last_value: None,
            condition,
        };
        self.watch_variables.insert(variable, watch);
    }

    pub fn update_line(&mut self, line: usize, locals: HashMap<String, String>) -> bool {
        self.current_line = line;
        self.current_locals = locals;

        // Check if we should break
        if let Some(breakpoint) = self.breakpoints.get_mut(&line) {
            if !breakpoint.enabled {
                return false;
            }

            breakpoint.hit_count += 1;

            // Check condition if present
            if let Some(condition) = &breakpoint.condition {
                if !self.evaluate_condition(condition) {
                    return false;
                }
            }

            // Update watch expressions
            for expr in &breakpoint.watch_expressions {
                if let Some(watch) = self.watch_variables.get_mut(expr) {
                    watch.last_value = self.evaluate_expression(expr);
                }
            }

            true
        } else {
            false
        }
    }

    pub fn evaluate_condition(&self, condition: &str) -> bool {
        // In real implementation, evaluate the condition using current_locals
        // For now, just return true
        true
    }

    pub fn evaluate_expression(&self, expression: &str) -> Option<String> {
        self.current_locals.get(expression).cloned()
    }

    pub fn get_watch_values(&self) -> HashMap<String, Option<String>> {
        self.watch_variables
            .iter()
            .map(|(var, info)| (var.clone(), info.last_value.clone()))
            .collect()
    }

    pub fn toggle_breakpoint(&mut self, line: usize) -> bool {
        if let Some(breakpoint) = self.breakpoints.get_mut(&line) {
            breakpoint.enabled = !breakpoint.enabled;
            breakpoint.enabled
        } else {
            false
        }
    }

    pub fn add_watch_to_breakpoint(&mut self, line: usize, expression: String) -> bool {
        if let Some(breakpoint) = self.breakpoints.get_mut(&line) {
            breakpoint.watch_expressions.push(expression);
            true
        } else {
            false
        }
    }

    pub fn get_breakpoint_info(&self, line: usize) -> Option<&Breakpoint> {
        self.breakpoints.get(&line)
    }

    pub fn clear_breakpoint(&mut self, line: usize) -> bool {
        self.breakpoints.remove(&line).is_some()
    }

    pub fn clear_all_breakpoints(&mut self) {
        self.breakpoints.clear();
    }

    pub fn get_current_context(&self) -> DebugContext {
        DebugContext {
            line: self.current_line,
            locals: self.current_locals.clone(),
            watches: self.get_watch_values(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DebugContext {
    pub line: usize,
    pub locals: HashMap<String, String>,
    pub watches: HashMap<String, Option<String>>,
}
