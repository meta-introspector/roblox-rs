//! AST visitor module.
//!
//! This module provides a visitor pattern implementation for traversing the AST.

use syn::{visit::{self, Visit}, File, ItemFn, ItemStruct, ItemEnum, ItemImpl, ImplItem};

/// A visitor that collects information about the AST.
pub struct AstCollector {
    /// Functions found in the AST
    pub functions: Vec<String>,
    /// Structs found in the AST
    pub structs: Vec<String>,
    /// Enums found in the AST
    pub enums: Vec<String>,
    /// Import paths found in the AST
    pub imports: Vec<String>,
}

impl AstCollector {
    /// Create a new collector
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            imports: Vec::new(),
        }
    }
    
    /// Analyze a Rust file and collect information
    pub fn analyze(&mut self, file: &File) {
        self.visit_file(file);
    }
}

impl<'ast> Visit<'ast> for AstCollector {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        // Collect function names
        self.functions.push(node.sig.ident.to_string());
        
        // Continue visiting the function body
        visit::visit_item_fn(self, node);
    }
    
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        // Collect struct names
        self.structs.push(node.ident.to_string());
        
        // Continue visiting the struct fields
        visit::visit_item_struct(self, node);
    }
    
    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        // Collect enum names
        self.enums.push(node.ident.to_string());
        
        // Continue visiting the enum variants
        visit::visit_item_enum(self, node);
    }
    
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        // Collect import paths (simplified)
        let path = format!("{:?}", node.tree);
        self.imports.push(path);
        
        // Continue visiting
        visit::visit_item_use(self, node);
    }
}

/// A dependency analyzer for finding external dependencies.
pub struct DependencyAnalyzer {
    /// External crates used in the code
    pub external_crates: Vec<String>,
    /// Standard library modules used
    pub std_modules: Vec<String>,
}

impl DependencyAnalyzer {
    /// Create a new dependency analyzer
    pub fn new() -> Self {
        Self {
            external_crates: Vec::new(),
            std_modules: Vec::new(),
        }
    }
    
    /// Analyze a Rust file and collect dependency information
    pub fn analyze(&mut self, file: &File) {
        self.visit_file(file);
    }
}

impl<'ast> Visit<'ast> for DependencyAnalyzer {
    fn visit_use_path(&mut self, path: &'ast syn::UsePath) {
        // Check for std modules
        let name = path.ident.to_string();
        
        if name == "std" {
            // Access the tree to look for the second segment
            if let syn::UseTree::Path(subtree) = &*path.tree {
                let module = subtree.ident.to_string();
                if !self.std_modules.contains(&module) {
                    self.std_modules.push(module);
                }
            }
        } else if !name.starts_with("self") && !name.starts_with("crate") {
            // External crate
            if !self.external_crates.contains(&name) {
                self.external_crates.push(name);
            }
        }
        
        // Continue visiting
        visit::visit_use_path(self, path);
    }
}

/// A visitor that looks for potential compatibility issues when compiling to Luau.
pub struct CompatibilityChecker {
    /// List of compatibility issues found
    pub issues: Vec<String>,
}

impl CompatibilityChecker {
    /// Create a new compatibility checker
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
        }
    }
    
    /// Check a Rust file for compatibility issues
    pub fn check(&mut self, file: &File) {
        self.visit_file(file);
    }
}

impl<'ast> Visit<'ast> for CompatibilityChecker {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        // Check for async functions
        if node.sig.asyncness.is_some() {
            self.issues.push(format!(
                "Async function '{}' may need special handling for Luau",
                node.sig.ident
            ));
        }
        
        // Check for unsafe blocks
        if node.sig.unsafety.is_some() {
            self.issues.push(format!(
                "Unsafe function '{}' will need manual safety checks in Luau",
                node.sig.ident
            ));
        }
        
        // Continue visiting
        visit::visit_item_fn(self, node);
    }
    
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        // Check for methods that might not be compatible
        let method_name = node.method.to_string();
        
        // Examples of methods that might need special handling
        if ["thread_local", "spawn", "catch_unwind"].contains(&method_name.as_str()) {
            self.issues.push(format!(
                "Method call '{}' might not be directly translatable to Luau",
                method_name
            ));
        }
        
        // Continue visiting
        visit::visit_expr_method_call(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;
    
    #[test]
    fn test_ast_collector() {
        let code = parse_quote! {
            fn hello() {
                println!("Hello");
            }
            
            struct Point {
                x: f32,
                y: f32,
            }
            
            enum Direction {
                North,
                South,
                East,
                West,
            }
        };
        
        let mut collector = AstCollector::new();
        collector.analyze(&code);
        
        assert_eq!(collector.functions.len(), 1);
        assert_eq!(collector.functions[0], "hello");
        
        assert_eq!(collector.structs.len(), 1);
        assert_eq!(collector.structs[0], "Point");
        
        assert_eq!(collector.enums.len(), 1);
        assert_eq!(collector.enums[0], "Direction");
    }
} 