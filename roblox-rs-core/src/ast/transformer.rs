//! AST transformation module.
//!
//! This module provides functionality for transforming a Rust AST into a Luau AST.

use syn::{File, Item, ItemFn, ItemStruct, ItemEnum, ItemImpl, ImplItem};
use thiserror::Error;

use super::luau::{LuauAst, LuauStmt, LuauExpr, LuauFunction, LuauTable};
use crate::error::Result;

/// Error that can occur during AST transformation.
#[derive(Error, Debug)]
pub enum TransformError {
    #[error("unsupported feature: {0}")]
    UnsupportedFeature(String),
    
    #[error("type error: {0}")]
    Type(String),
    
    #[error("transformation error: {0}")]
    Other(String),
}

impl From<TransformError> for crate::error::Error {
    fn from(err: TransformError) -> Self {
        crate::error::Error::Transform(err.to_string())
    }
}

/// Transform a Rust AST into a Luau AST.
pub fn transform_ast(ast: &File) -> Result<LuauAst> {
    let mut luau_ast = LuauAst::new();
    
    for item in &ast.items {
        transform_item(item, &mut luau_ast)?;
    }
    
    Ok(luau_ast)
}

/// Transform a top-level Rust item into Luau.
fn transform_item(item: &Item, ast: &mut LuauAst) -> Result<()> {
    match item {
        Item::Fn(item_fn) => transform_function(item_fn, ast)?,
        Item::Struct(item_struct) => transform_struct(item_struct, ast)?,
        Item::Enum(item_enum) => transform_enum(item_enum, ast)?,
        Item::Impl(item_impl) => transform_impl(item_impl, ast)?,
        Item::Use(_) => {
            // Imports are handled differently in Luau - they'll be resolved
            // during a separate pass
        }
        Item::Mod(_) => {
            // Modules will be translated into separate files or tables
            // depending on configuration
        }
        _ => {
            // For items we don't yet support, add a comment to the output
            let comment = format!("-- Unsupported Rust item: {:?}", item);
            ast.add_stmt(LuauStmt::Comment(comment));
        }
    }
    
    Ok(())
}

/// Transform a Rust function into a Luau function.
fn transform_function(func: &ItemFn, ast: &mut LuauAst) -> Result<()> {
    let name = func.sig.ident.to_string();
    let mut luau_func = LuauFunction::new(name);
    
    // Add parameters
    for param in &func.sig.inputs {
        match param {
            syn::FnArg::Typed(pat_type) => {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    luau_func.add_param(pat_ident.ident.to_string());
                }
            }
            _ => {
                return Err(TransformError::UnsupportedFeature(
                    "Self parameter not supported yet".to_string()
                ).into());
            }
        }
    }
    
    // Transform function body
    // In a real implementation, we would recursively transform each statement
    // For now, just add a placeholder
    
    luau_func.set_body(vec![
        LuauStmt::Comment(format!("-- Body of function {}", name)),
        LuauStmt::Return(Some(LuauExpr::Nil)),
    ]);
    
    // Add the function to the AST
    ast.add_stmt(LuauStmt::FunctionDecl(luau_func));
    
    Ok(())
}

/// Transform a Rust struct into a Luau table constructor function.
fn transform_struct(struct_item: &ItemStruct, ast: &mut LuauAst) -> Result<()> {
    let struct_name = struct_item.ident.to_string();
    let mut constructor_fn = LuauFunction::new(format!("create_{}", struct_name.to_lowercase()));
    
    // Add parameters for each field
    for field in &struct_item.fields {
        if let Some(ident) = &field.ident {
            constructor_fn.add_param(ident.to_string());
        }
    }
    
    // Create a table with the struct fields
    let mut table = LuauTable::new();
    
    for field in &struct_item.fields {
        if let Some(ident) = &field.ident {
            let field_name = ident.to_string();
            table.add_field(field_name.clone(), LuauExpr::Variable(field_name));
        }
    }
    
    // Return the table
    constructor_fn.set_body(vec![
        LuauStmt::Return(Some(LuauExpr::Table(table))),
    ]);
    
    // Add the constructor function to the AST
    ast.add_stmt(LuauStmt::FunctionDecl(constructor_fn));
    
    Ok(())
}

/// Transform a Rust enum into a Luau module with constants.
fn transform_enum(enum_item: &ItemEnum, ast: &mut LuauAst) -> Result<()> {
    let enum_name = enum_item.ident.to_string();
    
    // Create a table for the enum
    let mut enum_table = LuauTable::new();
    
    // Add a field for each variant
    for (i, variant) in enum_item.variants.iter().enumerate() {
        let variant_name = variant.ident.to_string();
        
        match &variant.fields {
            syn::Fields::Named(_) => {
                // For named fields, create a function that constructs the variant
                let function_name = format!("create_{}", variant_name.to_lowercase());
                enum_table.add_field(variant_name.clone(), LuauExpr::Variable(function_name));
            }
            syn::Fields::Unnamed(_) => {
                // For tuple variants, create a constructor function
                let function_name = format!("create_{}", variant_name.to_lowercase());
                enum_table.add_field(variant_name.clone(), LuauExpr::Variable(function_name));
            }
            syn::Fields::Unit => {
                // For unit variants, just use the index as a value
                enum_table.add_field(variant_name, LuauExpr::Number(i as f64));
            }
        }
    }
    
    // Create an assignment for the enum table
    ast.add_stmt(LuauStmt::LocalAssign(
        vec![enum_name],
        vec![LuauExpr::Table(enum_table)],
    ));
    
    Ok(())
}

/// Transform a Rust impl block into Luau methods.
fn transform_impl(impl_item: &ItemImpl, ast: &mut LuauAst) -> Result<()> {
    let type_name = match &*impl_item.self_ty {
        syn::Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                segment.ident.to_string()
            } else {
                return Err(TransformError::Other("Empty type path".to_string()).into());
            }
        }
        _ => {
            return Err(TransformError::UnsupportedFeature(
                "Complex impl self type not supported yet".to_string()
            ).into());
        }
    };
    
    // Add methods
    for item in &impl_item.items {
        if let ImplItem::Fn(method) = item {
            transform_method(&type_name, method, ast)?;
        }
    }
    
    Ok(())
}

/// Transform a Rust impl method into a Luau method.
fn transform_method(type_name: &str, method: &syn::ImplItemFn, ast: &mut LuauAst) -> Result<()> {
    let method_name = method.sig.ident.to_string();
    let full_method_name = format!("{}_{}", type_name.to_lowercase(), method_name);
    
    let mut luau_method = LuauFunction::new(full_method_name);
    
    // Handle self parameter
    let mut has_self = false;
    
    for param in &method.sig.inputs {
        match param {
            syn::FnArg::Receiver(_) => {
                has_self = true;
                luau_method.add_param("self".to_string());
            }
            syn::FnArg::Typed(pat_type) => {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    luau_method.add_param(pat_ident.ident.to_string());
                }
            }
        }
    }
    
    // Add the method to the type
    if has_self {
        // In a real implementation, we would add the method to the type's metatable
        // For now, just add a function with self parameter
        luau_method.set_body(vec![
            LuauStmt::Comment(format!("-- Body of method {}.{}", type_name, method_name)),
            LuauStmt::Return(Some(LuauExpr::Nil)),
        ]);
    } else {
        // Static method
        luau_method.set_body(vec![
            LuauStmt::Comment(format!("-- Body of static method {}.{}", type_name, method_name)),
            LuauStmt::Return(Some(LuauExpr::Nil)),
        ]);
    }
    
    // Add the method to the AST
    ast.add_stmt(LuauStmt::FunctionDecl(luau_method));
    
    Ok(())
} 