use syn::Type as RustType;
use quote::ToTokens;
use crate::ast::luau::{TypeAnnotation, PrimitiveType, TableType, BufferType, FunctionType};
use std::collections::{HashMap, HashSet};
mod optimizer;

pub use optimizer::{TypeOptimizer, TypeOptimizationHint, TypeLayout, LayoutOptimization};

// Use a placeholder implementation for now while we fix the full implementation
pub mod declarations;


/// Maps Rust types to Luau types with optimization hints
#[derive(Debug, Clone)]
pub struct TypeUsage {
    pub references: usize,
    pub mutations: usize,
    pub allocations: usize,
}

#[derive(Debug, Clone)]
pub struct TypeMutability {
    pub is_mutable: bool,
    pub mutable_fields: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct TypeField {
    pub name: String,
    pub type_name: String,
    pub size: usize,
    pub alignment: usize,
}

pub struct TypeMapper {
    // Cache commonly used type mappings
    type_map: HashMap<String, String>,
    type_sizes: HashMap<String, usize>,
    type_usage: HashMap<String, TypeUsage>,
    type_mutability: HashMap<String, TypeMutability>,
    type_fields: HashMap<String, Vec<TypeField>>,
    element_types: HashMap<String, String>,
    field_access: HashMap<String, usize>,
    type_cache: HashMap<String, TypeAnnotation>,
}

impl TypeMapper {
    pub fn new() -> Self {
        let mut mapper = Self {
            type_cache: HashMap::new(),
            type_map: HashMap::new(),
            type_sizes: HashMap::new(),
            type_usage: HashMap::new(),
            type_mutability: HashMap::new(),
            type_fields: HashMap::new(),
            element_types: HashMap::new(),
            field_access: HashMap::new(),
        };

        // Add built-in type sizes
        mapper.type_sizes.insert("i32".to_string(), 4);
        mapper.type_sizes.insert("i64".to_string(), 8);
        mapper.type_sizes.insert("f32".to_string(), 4);
        mapper.type_sizes.insert("f64".to_string(), 8);
        mapper.type_sizes.insert("bool".to_string(), 1);
        mapper.type_sizes.insert("string".to_string(), 16); // Approximate

        mapper.initialize_primitive_mappings();
        mapper
    }
    
    fn initialize_primitive_mappings(&mut self) {
        let mut type_sizes = HashMap::new();
        // Add built-in type sizes
        type_sizes.insert("i32".to_string(), 4);
        type_sizes.insert("i64".to_string(), 8);
        type_sizes.insert("f32".to_string(), 4);
        type_sizes.insert("f64".to_string(), 8);
        type_sizes.insert("bool".to_string(), 1);
        type_sizes.insert("string".to_string(), 16); // Approximate
        Self {
            type_sizes,
            type_usage: HashMap::new(),
            type_mutability: HashMap::new(),
            type_fields: HashMap::new(),
            element_types: HashMap::new(),
            field_access: HashMap::new(),
            type_cache: HashMap::new(),
        }
    }

    fn initialize_primitive_mappings(&mut self) {
        // Map Rust primitive types to Luau primitive types
        let mappings = [
            ("i8", PrimitiveType::Number),
            ("i16", PrimitiveType::Number),
            ("i32", PrimitiveType::Number),
            ("i64", PrimitiveType::Number),
            ("u8", PrimitiveType::Number),
            ("u16", PrimitiveType::Number),
            ("u32", PrimitiveType::Number),
            ("u64", PrimitiveType::Number),
            ("f32", PrimitiveType::Number),
            ("f64", PrimitiveType::Number),
            ("bool", PrimitiveType::Boolean),
            ("String", PrimitiveType::String),
            ("str", PrimitiveType::String),
            ("()", PrimitiveType::Nil),
        ];

        for (rust_type, luau_type) in mappings {
            self.type_cache.insert(
                rust_type.to_string(),
                TypeAnnotation::Primitive(luau_type),
            );
        }
    }

    /// Map a Rust type to a Luau type
    pub fn map_type(&mut self, rust_type: &RustType) -> TypeAnnotation {
        match rust_type {
            // Handle primitive types
            RustType::Path(type_path) => {
                if let Some(segment) = type_path.path.segments.last() {
                    let type_name = segment.ident.to_string();
                    
                    // Check cache first
                    if let Some(cached_type) = self.type_cache.get(&type_name) {
                        return cached_type.clone();
                    }

                    // Handle Vec<T> as buffer
                    if type_name == "Vec" {
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(syn::GenericArgument::Type(elem_type)) = args.args.first() {
                                let element_type = self.map_type(elem_type);
                                return TypeAnnotation::Buffer(Box::new(BufferType {
                                    element_type: Box::new(element_type),
                                    fixed_size: None,
                                }));
                            }
                        }
                    }

                    // Default to Nil for unknown types
                    TypeAnnotation::Primitive(PrimitiveType::Nil)
                } else {
                    TypeAnnotation::Primitive(PrimitiveType::Nil)
                }
            }

            // Handle array types as buffers with fixed size
            RustType::Array(array) => {
                let element_type = self.map_type(&array.elem);
                TypeAnnotation::Buffer(Box::new(BufferType {
                    element_type: Box::new(element_type),
                    fixed_size: Some(array.len.to_token_stream().to_string().parse().unwrap_or(0)),
                }))
            }

            // Handle tuple types as tables
            RustType::Tuple(tuple) => {
                let mut fields = HashMap::new();
                for (i, elem) in tuple.elems.iter().enumerate() {
                    fields.insert(i.to_string(), self.map_type(elem));
                }
                TypeAnnotation::Table(Box::new(TableType {
                    fields,
                    is_array: true,
                }))
            }

            // Handle function types
            RustType::BareFn(fn_type) => {
                let params = fn_type.inputs.iter()
                    .map(|arg| self.map_type(&arg.ty))
                    .collect();

                let return_type = match &fn_type.output {
                    syn::ReturnType::Default => TypeAnnotation::Primitive(PrimitiveType::Nil),
                    syn::ReturnType::Type(_, ty) => self.map_type(ty),
                };

                TypeAnnotation::Function(Box::new(FunctionType {
                    params,
                    return_type: Box::new(return_type),
                }))
            }

            // Default to Nil for other types
            _ => TypeAnnotation::Primitive(PrimitiveType::Nil),
        }
    }
}
