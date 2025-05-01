// Roblox-RS Transformer
// Transforms Rust AST (syn) into our internal Luau AST representation

use crate::transpiler::ast::*;
use syn;
use syn::visit::{self, Visit};
use proc_macro2::TokenStream;
use quote::ToTokens;

/// Transform a Rust AST (syn) into our Luau AST
pub fn transform_rust_to_luau(rust_ast: &syn::File) -> Result<Program, String> {
    let mut transformer = RustToLuauTransformer::new();
    transformer.visit_file(rust_ast);
    Ok(transformer.program)
}

/// Main transformer that visits the Rust AST and builds our Luau AST
pub struct RustToLuauTransformer {
    pub program: Program,
    current_module: Option<String>,
    current_class: Option<String>,
}

impl RustToLuauTransformer {
    pub fn new() -> Self {
        Self {
            program: Program { body: Vec::new() },
            current_module: None,
            current_class: None,
        }
    }
    
    /// Add a node to the current program
    fn add_node(&mut self, node: LuauNode) {
        self.program.body.push(node);
    }
    
    /// Convert a Rust function to our Luau function
    fn convert_function(&mut self, func: &syn::ItemFn) -> Function {
        let name = func.sig.ident.to_string();
        
        // Convert parameters
        let mut params = Vec::new();
        for input in &func.sig.inputs {
            if let syn::FnArg::Typed(pat_type) = input {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    let param_name = pat_ident.ident.to_string();
                    
                    // Extract type if available
                    let param_type = match &*pat_type.ty {
                        syn::Type::Path(type_path) => {
                            let type_name = type_path.path.segments.iter()
                                .map(|seg| seg.ident.to_string())
                                .collect::<Vec<_>>()
                                .join("::");
                            
                            Some(Type::Simple(type_name))
                        }
                        _ => None
                    };
                    
                    params.push(Parameter {
                        name: param_name,
                        param_type,
                        default_value: None,
                    });
                }
            }
        }
        
        // Convert return type
        let return_type = match &func.sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => {
                match &**ty {
                    syn::Type::Path(type_path) => {
                        let type_name = type_path.path.segments.iter()
                            .map(|seg| seg.ident.to_string())
                            .collect::<Vec<_>>()
                            .join("::");
                        
                        Some(Type::Simple(type_name))
                    }
                    _ => None
                }
            }
        };
        
        // Convert function body
        let mut body_statements = Vec::new();
        
        // Iterate through statements in the function body
        for stmt in &func.block.stmts {
            match stmt {
                syn::Stmt::Local(local) => {
                    // Local variable declaration
                    if let syn::Pat::Ident(pat_ident) = &local.pat {
                        let var_name = pat_ident.ident.to_string();
                        
                        // Extract type if available
                        let var_type = if let Some((_, ty)) = &local.ty {
                            match ty {
                                syn::Type::Path(type_path) => {
                                    let type_name = type_path.path.segments.iter()
                                        .map(|seg| seg.ident.to_string())
                                        .collect::<Vec<_>>()
                                        .join("::");
                                    
                                    Some(Type::Simple(type_name))
                                }
                                _ => None
                            }
                        } else {
                            None
                        };
                        
                        // Extract initializer if available
                        let initializer = if let Some((_, expr)) = &local.init {
                            Some(self.convert_expression(expr))
                        } else {
                            None
                        };
                        
                        body_statements.push(LuauNode::Variable(Variable {
                            name: var_name,
                            var_type,
                            initializer,
                            is_local: true,
                            is_const: false,
                        }));
                    }
                }
                
                syn::Stmt::Expr(expr, _) => {
                    // Expression statement
                    body_statements.push(LuauNode::Statement(Statement::ExpressionStatement(
                        self.convert_expression(expr)
                    )));
                }
                
                syn::Stmt::Semi(expr, _) => {
                    // Expression statement with semicolon
                    body_statements.push(LuauNode::Statement(Statement::ExpressionStatement(
                        self.convert_expression(expr)
                    )));
                }
                
                syn::Stmt::Item(item) => {
                    // Nested item (function, struct, enum, etc.)
                    // For simplicity, we'll just add a comment for now
                    body_statements.push(LuauNode::Comment(
                        format!("Nested item: {}", item.to_token_stream().to_string())
                    ));
                }
            }
        }
        
        Function {
            name,
            params,
            body: Block { statements: body_statements },
            return_type,
            is_method: false, // We'll set this when processing structs/impls
            is_local: true,   // Default to local functions
        }
    }
    
    /// Convert a Rust expression to our Luau expression
    fn convert_expression(&self, expr: &syn::Expr) -> Expression {
        match expr {
            syn::Expr::Path(expr_path) => {
                let ident = expr_path.path.segments.iter()
                    .map(|seg| seg.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::");
                
                Expression::Identifier(ident)
            }
            
            syn::Expr::Lit(expr_lit) => {
                match &expr_lit.lit {
                    syn::Lit::Str(lit_str) => {
                        Expression::Literal(Literal::String(lit_str.value()))
                    }
                    syn::Lit::Int(lit_int) => {
                        Expression::Literal(Literal::Number(lit_int.base10_parse::<f64>().unwrap_or(0.0)))
                    }
                    syn::Lit::Float(lit_float) => {
                        Expression::Literal(Literal::Number(lit_float.base10_parse::<f64>().unwrap_or(0.0)))
                    }
                    syn::Lit::Bool(lit_bool) => {
                        Expression::Literal(Literal::Boolean(lit_bool.value))
                    }
                    _ => Expression::Literal(Literal::Nil),
                }
            }
            
            syn::Expr::Binary(expr_bin) => {
                let left = Box::new(self.convert_expression(&expr_bin.left));
                let right = Box::new(self.convert_expression(&expr_bin.right));
                
                let operator = match &expr_bin.op {
                    syn::BinOp::Add(_) => BinaryOperator::Add,
                    syn::BinOp::Sub(_) => BinaryOperator::Subtract,
                    syn::BinOp::Mul(_) => BinaryOperator::Multiply,
                    syn::BinOp::Div(_) => BinaryOperator::Divide,
                    syn::BinOp::Rem(_) => BinaryOperator::Modulo,
                    syn::BinOp::BitXor(_) => BinaryOperator::Exponent, // ^ is exponent in Lua
                    syn::BinOp::Eq(_) => BinaryOperator::Equal,
                    syn::BinOp::Ne(_) => BinaryOperator::NotEqual,
                    syn::BinOp::Lt(_) => BinaryOperator::LessThan,
                    syn::BinOp::Le(_) => BinaryOperator::LessThanOrEqual,
                    syn::BinOp::Gt(_) => BinaryOperator::GreaterThan,
                    syn::BinOp::Ge(_) => BinaryOperator::GreaterThanOrEqual,
                    syn::BinOp::And(_) => BinaryOperator::And,
                    syn::BinOp::Or(_) => BinaryOperator::Or,
                    _ => BinaryOperator::Add, // Default for unsupported operators
                };
                
                Expression::BinaryExpression { left, operator, right }
            }
            
            syn::Expr::Unary(expr_unary) => {
                let argument = Box::new(self.convert_expression(&expr_unary.expr));
                
                let operator = match &expr_unary.op {
                    syn::UnOp::Neg(_) => UnaryOperator::Minus,
                    syn::UnOp::Not(_) => UnaryOperator::Not,
                    _ => UnaryOperator::Not, // Default for unsupported operators
                };
                
                Expression::UnaryExpression { operator, argument }
            }
            
            syn::Expr::Call(expr_call) => {
                let callee = Box::new(self.convert_expression(&expr_call.func));
                
                let mut arguments = Vec::new();
                for arg in &expr_call.args {
                    arguments.push(self.convert_expression(arg));
                }
                
                Expression::FunctionCall { callee, arguments }
            }
            
            syn::Expr::Field(expr_field) => {
                let object = Box::new(self.convert_expression(&expr_field.base));
                
                let property = if let syn::Member::Named(ident) = &expr_field.member {
                    Box::new(Expression::Identifier(ident.to_string()))
                } else {
                    // For unnamed members (tuple indices), convert to string identifier
                    Box::new(Expression::Identifier("unknown".to_string()))
                };
                
                Expression::MemberExpression {
                    object,
                    property,
                    computed: false,
                }
            }
            
            syn::Expr::Index(expr_index) => {
                let object = Box::new(self.convert_expression(&expr_index.expr));
                let property = Box::new(self.convert_expression(&expr_index.index));
                
                Expression::MemberExpression {
                    object,
                    property,
                    computed: true,
                }
            }
            
            syn::Expr::Struct(expr_struct) => {
                let mut properties = Vec::new();
                
                for field in &expr_struct.fields {
                    let key = Expression::Identifier(field.member.to_token_stream().to_string());
                    let value = self.convert_expression(&field.expr);
                    
                    properties.push((key, value));
                }
                
                Expression::TableExpression { properties }
            }
            
            syn::Expr::Array(expr_array) => {
                let mut elements = Vec::new();
                
                for element in &expr_array.elems {
                    elements.push(self.convert_expression(element));
                }
                
                Expression::ArrayExpression { elements }
            }
            
            syn::Expr::Block(expr_block) => {
                // For simplicity, just return a nil literal for block expressions
                // In a full implementation, we would recursively process the block
                Expression::Literal(Literal::Nil)
            }
            
            syn::Expr::If(expr_if) => {
                // For simplicity, just return a nil literal for if expressions
                // In a full implementation, we would convert this to a function call pattern
                Expression::Literal(Literal::Nil)
            }
            
            syn::Expr::Match(expr_match) => {
                // For simplicity, just return a nil literal for match expressions
                // In a full implementation, we would convert this to a series of if/elseif statements
                Expression::Literal(Literal::Nil)
            }
            
            _ => {
                // Default for unsupported expressions
                Expression::Literal(Literal::Nil)
            }
        }
    }
}

impl<'ast> Visit<'ast> for RustToLuauTransformer {
    fn visit_file(&mut self, file: &'ast syn::File) {
        // Process module attributes (if any)
        for attr in &file.attrs {
            if attr.path().is_ident("module") {
                // Extract module name
                if let Ok(lit) = attr.parse_args::<syn::LitStr>() {
                    self.current_module = Some(lit.value());
                }
            }
        }
        
        // Process all items in the file
        for item in &file.items {
            match item {
                syn::Item::Fn(func) => {
                    let luau_func = self.convert_function(func);
                    self.add_node(LuauNode::Function(luau_func));
                }
                
                syn::Item::Struct(item_struct) => {
                    let struct_name = item_struct.ident.to_string();
                    
                    // Store current class context
                    self.current_class = Some(struct_name.clone());
                    
                    // Create a class for this struct
                    let mut class = Class {
                        name: struct_name,
                        methods: Vec::new(),
                        properties: Vec::new(),
                        parent: None,
                    };
                    
                    // Process struct fields as class properties
                    if let syn::Fields::Named(fields_named) = &item_struct.fields {
                        for field in &fields_named.named {
                            if let Some(ident) = &field.ident {
                                let field_name = ident.to_string();
                                
                                // Extract type if available
                                let field_type = match &field.ty {
                                    syn::Type::Path(type_path) => {
                                        let type_name = type_path.path.segments.iter()
                                            .map(|seg| seg.ident.to_string())
                                            .collect::<Vec<_>>()
                                            .join("::");
                                        
                                        Some(Type::Simple(type_name))
                                    }
                                    _ => None
                                };
                                
                                class.properties.push(Variable {
                                    name: field_name,
                                    var_type: field_type,
                                    initializer: None,
                                    is_local: false,
                                    is_const: false,
                                });
                            }
                        }
                    }
                    
                    // Clear class context
                    self.current_class = None;
                    
                    // Add the class to the program
                    self.add_node(LuauNode::Class(class));
                }
                
                syn::Item::Impl(item_impl) => {
                    // Extract the type we're implementing for
                    if let syn::Type::Path(type_path) = &*item_impl.self_ty {
                        let impl_name = type_path.path.segments.iter()
                            .map(|seg| seg.ident.to_string())
                            .collect::<Vec<_>>()
                            .join("::");
                        
                        // Store class context
                        self.current_class = Some(impl_name.clone());
                        
                        // Process all impl items (methods)
                        for item in &item_impl.items {
                            if let syn::ImplItem::Fn(impl_fn) = item {
                                let method_name = impl_fn.sig.ident.to_string();
                                
                                // Check if it's an instance method (has self parameter)
                                let has_self = if let Some(first_arg) = impl_fn.sig.inputs.first() {
                                    matches!(first_arg, syn::FnArg::Receiver(_))
                                } else {
                                    false
                                };
                                
                                // Add a comment in the program that we found a method
                                self.add_node(LuauNode::Comment(
                                    format!("Method {} for class {}", method_name, impl_name)
                                ));
                                
                                // In a full implementation, we would add this to the class methods
                                // For now, just add as a standalone function
                                
                                // Convert parameters
                                let mut params = Vec::new();
                                for input in &impl_fn.sig.inputs {
                                    match input {
                                        syn::FnArg::Receiver(_) => {
                                            // Add 'self' parameter
                                            params.push(Parameter {
                                                name: "self".to_string(),
                                                param_type: None,
                                                default_value: None,
                                            });
                                        }
                                        syn::FnArg::Typed(pat_type) => {
                                            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                                                let param_name = pat_ident.ident.to_string();
                                                
                                                // Extract type if available
                                                let param_type = match &*pat_type.ty {
                                                    syn::Type::Path(type_path) => {
                                                        let type_name = type_path.path.segments.iter()
                                                            .map(|seg| seg.ident.to_string())
                                                            .collect::<Vec<_>>()
                                                            .join("::");
                                                        
                                                        Some(Type::Simple(type_name))
                                                    }
                                                    _ => None
                                                };
                                                
                                                params.push(Parameter {
                                                    name: param_name,
                                                    param_type,
                                                    default_value: None,
                                                });
                                            }
                                        }
                                    }
                                }
                                
                                // Create function node for this method
                                let func = Function {
                                    name: format!("{}.{}", impl_name, method_name),
                                    params,
                                    body: Block { statements: Vec::new() },
                                    return_type: None,
                                    is_method: has_self,
                                    is_local: false,
                                };
                                
                                self.add_node(LuauNode::Function(func));
                            }
                        }
                        
                        // Clear class context
                        self.current_class = None;
                    }
                }
                
                syn::Item::Enum(item_enum) => {
                    let enum_name = item_enum.ident.to_string();
                    
                    // Add a comment for this enum
                    self.add_node(LuauNode::Comment(
                        format!("Enum: {}", enum_name)
                    ));
                    
                    // In a full implementation, we would convert the enum to a table of constants
                    // For now, just add a comment for each variant
                    
                    for variant in &item_enum.variants {
                        let variant_name = variant.ident.to_string();
                        
                        self.add_node(LuauNode::Comment(
                            format!("Enum variant: {}.{}", enum_name, variant_name)
                        ));
                    }
                }
                
                syn::Item::Mod(item_mod) => {
                    let mod_name = item_mod.ident.to_string();
                    
                    // Handle inline modules
                    if let Some(content) = &item_mod.content {
                        // Store previous module context
                        let prev_module = self.current_module.clone();
                        
                        // Set current module context
                        self.current_module = Some(mod_name.clone());
                        
                        // Add a comment for this module
                        self.add_node(LuauNode::Comment(
                            format!("Module: {}", mod_name)
                        ));
                        
                        // Process all items in the module
                        for item in &content.1 {
                            visit::visit_item(self, item);
                        }
                        
                        // Restore previous module context
                        self.current_module = prev_module;
                    } else {
                        // External module (not inline)
                        self.add_node(LuauNode::Comment(
                            format!("External module: {}", mod_name)
                        ));
                    }
                }
                
                // For all other items, just add a comment
                _ => {
                    self.add_node(LuauNode::Comment(
                        format!("Unsupported item: {}", item.to_token_stream().to_string())
                    ));
                }
            }
        }
        
        // Call the default implementation to visit any nested items
        visit::visit_file(self, file);
    }
}
