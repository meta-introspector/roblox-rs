//! Luau code generation and utilities.
//!
//! This module provides types and functions for generating and manipulating Luau code.

use std::collections::HashMap;

/// A Luau AST node.
#[derive(Debug, Clone, PartialEq)]
pub struct LuauAst {
    /// Top-level statements in the AST
    pub statements: Vec<LuauStmt>,
}

impl LuauAst {
    /// Create a new empty AST
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
    
    /// Add a statement to the AST
    pub fn add_stmt(&mut self, stmt: LuauStmt) {
        self.statements.push(stmt);
    }
}

/// A Luau statement.
#[derive(Debug, Clone, PartialEq)]
pub enum LuauStmt {
    /// A local variable declaration: `local x = y`
    LocalAssign(Vec<String>, Vec<LuauExpr>),
    /// A global variable assignment: `x = y`
    GlobalAssign(Vec<String>, Vec<LuauExpr>),
    /// A function declaration: `function name(params) body end`
    FunctionDecl(LuauFunction),
    /// A return statement: `return expr`
    Return(Option<LuauExpr>),
    /// An if statement: `if cond then body else else_body end`
    If {
        condition: LuauExpr,
        then_block: Vec<LuauStmt>,
        else_block: Option<Vec<LuauStmt>>,
    },
    /// A while loop: `while cond do body end`
    While {
        condition: LuauExpr,
        body: Vec<LuauStmt>,
    },
    /// A for loop: `for var = start, end, step do body end`
    For {
        var: String,
        start: LuauExpr,
        end: LuauExpr,
        step: Option<LuauExpr>,
        body: Vec<LuauStmt>,
    },
    /// A for-in loop: `for k, v in pairs(table) do body end`
    ForIn {
        vars: Vec<String>,
        iterators: Vec<LuauExpr>,
        body: Vec<LuauStmt>,
    },
    /// A comment: `-- comment`
    Comment(String),
    /// A raw statement as a string (for complex cases)
    Raw(String),
}

/// A Luau expression.
#[derive(Debug, Clone, PartialEq)]
pub enum LuauExpr {
    /// A literal string: `"hello"`
    String(String),
    /// A literal number: `42` or `3.14`
    Number(f64),
    /// A boolean: `true` or `false`
    Boolean(bool),
    /// `nil`
    Nil,
    /// A variable reference: `foo`
    Variable(String),
    /// A table: `{ foo = bar, [1] = baz }`
    Table(LuauTable),
    /// A function call: `foo(arg1, arg2)`
    Call {
        func: Box<LuauExpr>,
        args: Vec<LuauExpr>,
    },
    /// A binary operation: `a + b`
    BinaryOp {
        left: Box<LuauExpr>,
        op: String,
        right: Box<LuauExpr>,
    },
    /// A unary operation: `-a` or `not a`
    UnaryOp {
        op: String,
        expr: Box<LuauExpr>,
    },
    /// A table index: `tbl[expr]` or `tbl.key`
    Index {
        table: Box<LuauExpr>,
        key: Box<LuauExpr>,
    },
    /// A function definition: `function(params) body end`
    Function(LuauFunction),
    /// A parenthesized expression: `(expr)`
    Paren(Box<LuauExpr>),
    /// A raw expression as a string (for complex cases)
    Raw(String),
}

/// A Luau function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct LuauFunction {
    /// Function name, if any
    pub name: Option<String>,
    /// Function parameters
    pub params: Vec<String>,
    /// Function body
    pub body: Vec<LuauStmt>,
    /// Whether the function is variadic
    pub is_variadic: bool,
}

impl LuauFunction {
    /// Create a new named function
    pub fn new(name: String) -> Self {
        Self {
            name: Some(name),
            params: Vec::new(),
            body: Vec::new(),
            is_variadic: false,
        }
    }
    
    /// Create a new anonymous function
    pub fn anonymous() -> Self {
        Self {
            name: None,
            params: Vec::new(),
            body: Vec::new(),
            is_variadic: false,
        }
    }
    
    /// Add a parameter to the function
    pub fn add_param(&mut self, param: String) {
        self.params.push(param);
    }
    
    /// Set the function body
    pub fn set_body(&mut self, body: Vec<LuauStmt>) {
        self.body = body;
    }
    
    /// Make the function variadic
    pub fn make_variadic(&mut self) {
        self.is_variadic = true;
    }
}

/// A Luau table.
#[derive(Debug, Clone, PartialEq)]
pub struct LuauTable {
    /// Fields with string keys
    pub fields: HashMap<String, LuauExpr>,
    /// Fields with expression keys
    pub array_items: Vec<LuauExpr>,
}

impl LuauTable {
    /// Create a new empty table
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            array_items: Vec::new(),
        }
    }
    
    /// Add a field with a string key
    pub fn add_field(&mut self, key: String, value: LuauExpr) {
        self.fields.insert(key, value);
    }
    
    /// Add an array item
    pub fn add_array_item(&mut self, value: LuauExpr) {
        self.array_items.push(value);
    }
}

/// Generates Luau code from an AST.
pub mod generator {
    use super::*;
    
    /// Generate Luau code from an AST.
    pub fn generate_code(ast: &LuauAst) -> Result<String, String> {
        let mut code = String::new();
        
        // Add preamble
        code.push_str("-- Generated by roblox-rs\n\n");
        
        // Generate code for each statement
        for stmt in &ast.statements {
            code.push_str(&generate_stmt(stmt, 0)?);
            code.push('\n');
        }
        
        Ok(code)
    }
    
    /// Generate code for a statement.
    fn generate_stmt(stmt: &LuauStmt, indent_level: usize) -> Result<String, String> {
        let indent = "    ".repeat(indent_level);
        
        match stmt {
            LuauStmt::LocalAssign(names, exprs) => {
                let mut code = format!("{}local ", indent);
                code.push_str(&names.join(", "));
                
                if !exprs.is_empty() {
                    code.push_str(" = ");
                    let expr_code: Result<Vec<String>, _> = 
                        exprs.iter().map(|e| generate_expr(e)).collect();
                    code.push_str(&expr_code?.join(", "));
                }
                
                Ok(code)
            }
            LuauStmt::GlobalAssign(names, exprs) => {
                let mut code = format!("{}", indent);
                code.push_str(&names.join(", "));
                
                if !exprs.is_empty() {
                    code.push_str(" = ");
                    let expr_code: Result<Vec<String>, _> = 
                        exprs.iter().map(|e| generate_expr(e)).collect();
                    code.push_str(&expr_code?.join(", "));
                }
                
                Ok(code)
            }
            LuauStmt::FunctionDecl(func) => generate_function(func, indent_level),
            LuauStmt::Return(expr) => {
                let mut code = format!("{}return", indent);
                
                if let Some(expr) = expr {
                    code.push_str(" ");
                    code.push_str(&generate_expr(expr)?);
                }
                
                Ok(code)
            }
            LuauStmt::If { condition, then_block, else_block } => {
                let mut code = format!("{}if {} then\n", indent, generate_expr(condition)?);
                
                for stmt in then_block {
                    code.push_str(&generate_stmt(stmt, indent_level + 1)?);
                    code.push('\n');
                }
                
                if let Some(else_block) = else_block {
                    code.push_str(&format!("{}else\n", indent));
                    
                    for stmt in else_block {
                        code.push_str(&generate_stmt(stmt, indent_level + 1)?);
                        code.push('\n');
                    }
                }
                
                code.push_str(&format!("{}end", indent));
                Ok(code)
            }
            LuauStmt::While { condition, body } => {
                let mut code = format!("{}while {} do\n", indent, generate_expr(condition)?);
                
                for stmt in body {
                    code.push_str(&generate_stmt(stmt, indent_level + 1)?);
                    code.push('\n');
                }
                
                code.push_str(&format!("{}end", indent));
                Ok(code)
            }
            LuauStmt::For { var, start, end, step, body } => {
                let mut code = format!(
                    "{}for {} = {}, {}", 
                    indent, 
                    var, 
                    generate_expr(start)?, 
                    generate_expr(end)?
                );
                
                if let Some(step) = step {
                    code.push_str(&format!(", {}", generate_expr(step)?));
                }
                
                code.push_str(" do\n");
                
                for stmt in body {
                    code.push_str(&generate_stmt(stmt, indent_level + 1)?);
                    code.push('\n');
                }
                
                code.push_str(&format!("{}end", indent));
                Ok(code)
            }
            LuauStmt::ForIn { vars, iterators, body } => {
                let mut code = format!("{}for {} in ", indent, vars.join(", "));
                
                let iter_code: Result<Vec<String>, _> = 
                    iterators.iter().map(|e| generate_expr(e)).collect();
                code.push_str(&iter_code?.join(", "));
                
                code.push_str(" do\n");
                
                for stmt in body {
                    code.push_str(&generate_stmt(stmt, indent_level + 1)?);
                    code.push('\n');
                }
                
                code.push_str(&format!("{}end", indent));
                Ok(code)
            }
            LuauStmt::Comment(comment) => {
                Ok(format!("{}-- {}", indent, comment))
            }
            LuauStmt::Raw(raw) => {
                Ok(format!("{}{}", indent, raw))
            }
        }
    }
    
    /// Generate code for a function.
    fn generate_function(func: &LuauFunction, indent_level: usize) -> Result<String, String> {
        let indent = "    ".repeat(indent_level);
        let mut code = String::new();
        
        if let Some(name) = &func.name {
            code.push_str(&format!("{}function {}(", indent, name));
        } else {
            code.push_str(&format!("{}function(", indent));
        }
        
        // Add parameters
        code.push_str(&func.params.join(", "));
        
        // Add variadic marker
        if func.is_variadic {
            if !func.params.is_empty() {
                code.push_str(", ...");
            } else {
                code.push_str("...");
            }
        }
        
        code.push_str(")\n");
        
        // Add body
        for stmt in &func.body {
            code.push_str(&generate_stmt(stmt, indent_level + 1)?);
            code.push('\n');
        }
        
        code.push_str(&format!("{}end", indent));
        Ok(code)
    }
    
    /// Generate code for an expression.
    fn generate_expr(expr: &LuauExpr) -> Result<String, String> {
        match expr {
            LuauExpr::String(s) => {
                // Escape quotes properly
                let escaped = s.replace('\"', "\\\"");
                Ok(format!("\"{}\"", escaped))
            }
            LuauExpr::Number(n) => {
                Ok(n.to_string())
            }
            LuauExpr::Boolean(b) => {
                Ok(if *b { "true".to_string() } else { "false".to_string() })
            }
            LuauExpr::Nil => {
                Ok("nil".to_string())
            }
            LuauExpr::Variable(name) => {
                Ok(name.clone())
            }
            LuauExpr::Table(table) => {
                let mut items = Vec::new();
                
                // Array part
                for item in &table.array_items {
                    items.push(generate_expr(item)?);
                }
                
                // Named fields
                for (key, value) in &table.fields {
                    let value_code = generate_expr(value)?;
                    
                    // See if the key is a valid identifier
                    if key.chars().all(|c| c.is_alphanumeric() || c == '_') 
                        && !key.chars().next().unwrap_or('_').is_numeric() {
                        items.push(format!("{} = {}", key, value_code));
                    } else {
                        items.push(format!("[\"{}\"] = {}", key.replace('\"', "\\\""), value_code));
                    }
                }
                
                Ok(format!("{{{}}}", items.join(", ")))
            }
            LuauExpr::Call { func, args } => {
                let func_code = generate_expr(func)?;
                
                let arg_code: Result<Vec<String>, _> = 
                    args.iter().map(|e| generate_expr(e)).collect();
                
                Ok(format!("{}({})", func_code, arg_code?.join(", ")))
            }
            LuauExpr::BinaryOp { left, op, right } => {
                let left_code = generate_expr(left)?;
                let right_code = generate_expr(right)?;
                
                Ok(format!("{} {} {}", left_code, op, right_code))
            }
            LuauExpr::UnaryOp { op, expr } => {
                let expr_code = generate_expr(expr)?;
                
                Ok(format!("{}{}", op, expr_code))
            }
            LuauExpr::Index { table, key } => {
                let table_code = generate_expr(table)?;
                
                // Handle different types of keys
                match &**key {
                    LuauExpr::String(s) => {
                        // If the key is a valid identifier, use dot notation
                        if s.chars().all(|c| c.is_alphanumeric() || c == '_') 
                            && !s.chars().next().unwrap_or('_').is_numeric() {
                            Ok(format!("{}.{}", table_code, s))
                        } else {
                            Ok(format!("{}[{}]", table_code, generate_expr(key)?))
                        }
                    }
                    _ => {
                        Ok(format!("{}[{}]", table_code, generate_expr(key)?))
                    }
                }
            }
            LuauExpr::Function(func) => {
                generate_function(func, 0)
            }
            LuauExpr::Paren(expr) => {
                let expr_code = generate_expr(expr)?;
                
                Ok(format!("({})", expr_code))
            }
            LuauExpr::Raw(raw) => {
                Ok(raw.clone())
            }
        }
    }
}

/// Optimizes Luau code.
pub mod optimizer {
    use super::*;
    use crate::compiler::CompileOptions;
    
    /// Optimize Luau code.
    pub fn optimize_code(code: &str, options: &CompileOptions) -> Result<String, String> {
        // In a real implementation, we would apply optimizations here
        // For now, just add optimization directives
        
        let mut optimized = String::new();
        
        match options.optimization_level {
            crate::compiler::OptimizationLevel::Minimal => {
                // No optimization directives
            }
            crate::compiler::OptimizationLevel::Default => {
                optimized.push_str("--!optimize 1\n");
            }
            crate::compiler::OptimizationLevel::Aggressive => {
                optimized.push_str("--!optimize 2\n--!native\n");
            }
        }
        
        optimized.push_str(code);
        Ok(optimized)
    }
} 