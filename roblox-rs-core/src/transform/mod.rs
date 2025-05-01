use syn::{Item, ItemFn, Expr, ItemStruct, ItemImpl, ImplItem, Pat, Type, Stmt, Block, TypeInfer, BinOp, Lit};
use quote::ToTokens;
use crate::ast::{LuauNode, Program, Buffer, Table, Function, Parameter};
use std::string::String;

// Define these types directly instead of importing them from ast::luau
type Number = i32;
type Array = Vec<LuauNode>;
type LuauString = String;
use crate::types::TypeMapper;

pub struct RustToLuauTransformer {
    // Track state during transformation
    current_scope_vars: Vec<String>,
    type_mapper: TypeMapper,
}

impl RustToLuauTransformer {
    pub fn new() -> Self {
        Self {
            current_scope_vars: Vec::new(),
            type_mapper: TypeMapper::new(),
        }
    }

    pub fn transform(&mut self, item: Item) -> Result<LuauNode, String> {
        self.transform_item(item)
    }

    pub fn transform_program(&mut self, input: &str) -> Result<Program, String> {
        // Parse Rust code using syn
        let syntax_tree = syn::parse_file(input)
            .map_err(|e| format!("Failed to parse Rust code: {}", e))?;

        let mut program = Program {
            body: Vec::new(),
            type_annotations: Default::default(),
        };

        // Transform each item in the Rust AST
        for item in syntax_tree.items {
            match self.transform_item(item) {
                Ok(node) => program.body.push(node),
                Err(e) => return Err(format!("Transform error: {}", e)),
            }
        }

        Ok(program)
    }

    fn transform_item(&mut self, item: Item) -> Result<LuauNode, String> {
        match item {
            Item::Fn(func) => self.transform_function(func),
            Item::Struct(struct_item) => self.transform_struct(struct_item),
            Item::Impl(impl_item) => self.transform_impl(impl_item),
            Item::Const(const_item) => self.transform_const(const_item),
            Item::Static(static_item) => self.transform_static(static_item),
            // Handle other item types
            _ => Err(format!("Unsupported item type: {}", item.to_token_stream())),
        }
    }

    fn transform_function(&mut self, func: ItemFn) -> Result<LuauNode, String> {
        // Transform parameters
        let params = func.sig.inputs.iter().map(|param| {
            if let syn::FnArg::Typed(pat_type) = param {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    Ok(Parameter {
                        name: pat_ident.ident.to_string(),
                        type_annotation: Some(self.type_mapper.map_type(&pat_type.ty)),
                    })
                } else {
                    Err("Unsupported parameter pattern".to_string())
                }
            } else {
                Err("Unsupported parameter type".to_string())
            }
        }).collect::<Result<Vec<_>, _>>()?;

        // Transform return type
        let return_type = if let syn::ReturnType::Type(_, ty) = &func.sig.output {
            Some(self.type_mapper.map_type(ty))
        } else {
            None
        };

        // Transform function body
        let mut body_nodes = Vec::new();
        
        // Transform all statements except the last one
        for stmt in func.block.stmts.iter().take(func.block.stmts.len().saturating_sub(1)) {
            body_nodes.push(self.transform_stmt(stmt)?);                
        }
        
        // Handle the last statement as a return value if it's an expression
        if let Some(last_stmt) = func.block.stmts.last() {
            if let syn::Stmt::Expr(expr, _) = last_stmt {
                body_nodes.push(LuauNode::Return(Box::new(self.transform_expr(expr)?)));
            } else {
                body_nodes.push(self.transform_stmt(last_stmt)?);                    
            }
        }
        
        Ok(LuauNode::Function(Box::new(Function {
            name: func.sig.ident.to_string(),
            params,
            return_type,
            body: Box::new(LuauNode::Block(body_nodes)),
        })))
    }

    fn transform_stmt(&mut self, stmt: &syn::Stmt) -> Result<LuauNode, String> {
        match stmt {
            syn::Stmt::Local(local) => {
                // Transform local variable declaration
                let pattern = &local.pat;
                let init = local.init.as_ref().map(|init| &init.expr);
                
                // For now, only support simple variable declarations
                if let syn::Pat::Ident(ident) = pattern {
                    let var_name = ident.ident.to_string();
                    if let Some(expr) = init {
                        let value = self.transform_expr(expr)?;
                        Ok(LuauNode::Local {
                            name: var_name,
                            value: Box::new(value),
                            type_annotation: None,
                        })
                    } else {
                        Ok(LuauNode::Local {
                            name: var_name,
                            value: Box::new(LuauNode::Nil),
                            type_annotation: None,
                        })
                    }
                } else {
                    Err(format!("Unsupported pattern in local statement: {}", pattern.to_token_stream()))
                }
            },
            syn::Stmt::Expr(expr, _) => {
                // Transform expression statement
                self.transform_expr(expr)
            },
            _ => Err(format!("Unsupported statement type: {}", stmt.to_token_stream())),
        }
    }

    fn transform_block(&mut self, block: &Block) -> Result<Vec<LuauNode>, String> {
        let mut nodes = Vec::new();
        
        for stmt in &block.stmts {
            nodes.push(self.transform_stmt(stmt)?);
        }
        
        Ok(nodes)
    }

    fn transform_expr(&mut self, expr: &Expr) -> Result<LuauNode, String> {
        match expr {
            Expr::Lit(lit) => {
                match &lit.lit {
                    Lit::Int(i) => Ok(LuauNode::Number(Number::Integer(i.base10_parse().map_err(|e| e.to_string())?))),
                    Lit::Float(f) => Ok(LuauNode::Number(Number::Float(f.base10_parse().map_err(|e| e.to_string())?))),
                    Lit::Bool(b) => Ok(LuauNode::Boolean(b.value)),
                    Lit::Str(s) => Ok(LuauNode::String(s.value())),
                    _ => Err(format!("Unsupported literal type: {}", lit.lit.to_token_stream())),
                }
            },
            Expr::Array(array) => {
                let mut elements = Vec::new();
                for elem in &array.elems {
                    elements.push(self.transform_expr(elem)?);
                }
                Ok(LuauNode::Array(Array { elements }))
            },
            Expr::Binary(binary) => {
                let left = Box::new(self.transform_expr(&binary.left)?);
                let right = Box::new(self.transform_expr(&binary.right)?);
                Ok(LuauNode::Binary {
                    left,
                    operator: format!("{}", binary.op.to_token_stream()),
                    right,
                })
            },
            Expr::If(expr_if) => {
                let condition = Box::new(self.transform_expr(&expr_if.cond)?);
                let then_branch = self.transform_block(&expr_if.then_branch)?;
                
                let else_branch = if let Some((_, else_block)) = &expr_if.else_branch {
                    match &**else_block {
                        Expr::Block(block) => Some(self.transform_block(&block.block)?),
                        _ => return Err(format!("Unsupported else branch: {}", else_block.to_token_stream())),
                    }
                } else {
                    None
                };
                
                Ok(LuauNode::If {
                    condition,
                    then_branch: Box::new(LuauNode::Block(then_branch)),
                    else_branch: else_branch.map(|b| Box::new(LuauNode::Block(b))),
                })
            },
            Expr::Call(call) => {
                let func = self.transform_expr(&call.func)?;
                
                let mut args = Vec::new();
                for arg in &call.args {
                    args.push(self.transform_expr(arg)?);
                }
                
                Ok(LuauNode::Call {
                    function: Box::new(func),
                    arguments: args,
                })
            },
            Expr::MethodCall(method_call) => {
                let receiver = self.transform_expr(&method_call.receiver)?;
                let method_name = method_call.method.to_string();
                
                let mut args = Vec::new();
                for arg in &method_call.args {
                    args.push(self.transform_expr(arg)?);
                }
                
                // Insert self as first argument
                Ok(LuauNode::MethodCall {
                    object: Box::new(receiver),
                    method: method_name,
                    arguments: args,
                })
            },
            Expr::Tuple(expr_tuple) => {
                let mut table = Table {
                    fields: Vec::new(),
                    optimization_hints: Default::default(),
                };
                
                // Create a Luau table with numeric indices
                for (i, elem) in expr_tuple.elems.iter().enumerate() {
                    let elem_value = self.transform_expr(elem)?;
                    table.fields.push((i.to_string(), elem_value));
                }
                
                // Special case: If it's a 3-element tuple, try to optimize as Vector3
                if expr_tuple.elems.len() == 3 {
                    // Set x, y, z field names for potential Vector3 optimization
                    if let Some((_, x_val)) = table.fields.get(0) {
                        if let Some((_, y_val)) = table.fields.get(1) {
                            if let Some((_, z_val)) = table.fields.get(2) {
                                // Create a new table with x, y, z fields instead of numeric indices
                                let mut vector_table = Table {
                                    fields: vec![
                                        ("x".to_string(), x_val.clone()),
                                        ("y".to_string(), y_val.clone()),
                                        ("z".to_string(), z_val.clone()),
                                    ],
                                    optimization_hints: Default::default(),
                                };
                                
                                // Mark this as a potential Vector3 for the optimizer
                                vector_table.optimization_hints.native_buffer_type = Some("Vector3".to_string());
                                
                                return Ok(LuauNode::Table(vector_table));
                            }
                        }
                    }
                }
                
                // If we get here, it's a regular tuple
                Ok(LuauNode::Table(table))
            },
            _ => Err(format!("Unsupported expression type: {}", expr.to_token_stream())),
        }
    }

    fn transform_vec_expr(&mut self, expr: &Expr) -> Result<Buffer, String> {
        // Analyze the Vec expression to determine:
        // 1. Initial capacity if specified
        // 2. Element type
        // 3. Usage patterns for optimization

        // This is a placeholder implementation
        Ok(Buffer {
            initial_size: 0,
            element_type: Default::default(),
            optimization_level: Default::default(),
        })
    }

    // Transform Rust structs to Luau tables with optimization hints
    fn transform_struct(&mut self, item_struct: ItemStruct) -> Result<LuauNode, String> {
        let mut table = Table {
            fields: Vec::new(),
            optimization_hints: Default::default(),
        };

        // Transform struct fields to table fields
        for field in item_struct.fields {
            if let Some(ident) = field.ident {
                let field_type = self.type_mapper.map_type(&field.ty);
                table.fields.push((ident.to_string(), LuauNode::Program(Program {
                    body: Vec::new(),
                    type_annotations: Default::default(),
                })));
            }
        }

        // Set optimization hints
        table.optimization_hints.pre_allocate = Some(table.fields.len());
        table.optimization_hints.stable_keys = true;

        Ok(LuauNode::Table(table))
    }

    fn transform_impl(&mut self, item_impl: ItemImpl) -> Result<LuauNode, String> {
        let mut methods = Vec::new();

        for item in item_impl.items {
            match item {
                ImplItem::Fn(method) => {
                    // Transform method to function
                    let mut func = self.transform_function(ItemFn {
                        attrs: method.attrs,
                        vis: method.vis,
                        sig: method.sig,
                        block: Box::new(method.block),
                    })?;

                    // If it's a method, add self parameter
                    if let LuauNode::Function(ref mut f) = func {
                        f.params.insert(0, Parameter {
                            name: "self".to_string(),
                            type_annotation: None,
                        });
                    }

                    methods.push(func);
                }
                _ => return Err(format!("Unsupported impl item: {}", item.to_token_stream())),
            }
        }

        // Create a table to hold the implementation
        let mut table = Table {
            fields: Vec::new(),
            optimization_hints: Default::default(),
        };

        // Add methods to the table
        for method in methods {
            if let LuauNode::Function(f) = method {
                table.fields.push((f.name.clone(), LuauNode::Function(f)));
            }
        }

        Ok(LuauNode::Table(table))
    }

    fn transform_const(&mut self, const_item: syn::ItemConst) -> Result<LuauNode, String> {
        // Transform constant to a local variable
        let name = const_item.ident.to_string();
        let expr = self.transform_expr(&const_item.expr)?;

        Ok(expr)
    }

    fn transform_static(&mut self, static_item: syn::ItemStatic) -> Result<LuauNode, String> {
        // Transform static item to a local variable
        let name = static_item.ident.to_string();
        let expr = self.transform_expr(&static_item.expr)?;

        Ok(expr)
    }
}
