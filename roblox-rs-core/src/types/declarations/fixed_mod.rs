use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::ast::luau::TypeAnnotation;
use crate::types::TypeMapper;

/// Represents a Luau type declaration, similar to TypeScript .d.ts files
#[derive(Debug, Clone)]
pub struct TypeDeclaration {
    pub name: String,
    pub type_annotation: TypeAnnotation,
    pub documentation: Option<String>,
}

/// Represents a namespace containing type declarations
#[derive(Debug, Clone)]
pub struct TypeNamespace {
    pub name: String,
    pub types: HashMap<String, TypeDeclaration>,
    pub sub_namespaces: HashMap<String, TypeNamespace>,
}

/// Manager for handling .d.rs type declaration files
pub struct TypeDeclarationManager {
    root_namespace: TypeNamespace,
    type_mapper: TypeMapper,
}

impl TypeDeclarationManager {
    pub fn new() -> Self {
        Self {
            root_namespace: TypeNamespace {
                name: "global".to_string(),
                types: HashMap::new(),
                sub_namespaces: HashMap::new(),
            },
            type_mapper: TypeMapper::new(),
        }
    }

    /// Parse a .d.rs file and load the type declarations
    pub fn parse_declaration_file(&mut self, file_path: &Path) -> Result<(), String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read declaration file: {}", e))?;

        // Parse the Rust-style type declarations
        let syntax = syn::parse_file(&content)
            .map_err(|e| format!("Failed to parse declarations: {}", e))?;

        for item in syntax.items {
            match item {
                // Handle module declarations as namespaces
                syn::Item::Mod(module) => {
                    self.process_module(&module)?;
                },
                // Handle type aliases as type declarations
                syn::Item::Type(type_alias) => {
                    self.process_type_alias(&type_alias)?;
                },
                // Handle structs as object type declarations
                syn::Item::Struct(struct_item) => {
                    self.process_struct(&struct_item)?;
                },
                // Handle traits as interface declarations
                syn::Item::Trait(trait_item) => {
                    self.process_trait(&trait_item)?;
                },
                // Handle enums as union types
                syn::Item::Enum(enum_item) => {
                    self.process_enum(&enum_item)?;
                },
                // Ignore other items
                _ => {},
            }
        }

        Ok(())
    }

    /// Process a module declaration
    fn process_module(&mut self, module: &syn::ItemMod) -> Result<(), String> {
        let namespace_name = module.ident.to_string();
        
        // Create or get namespace
        let namespace = self.get_or_create_namespace(&namespace_name);
        
        // Process module contents if available
        if let Some((_, items)) = &module.content {
            for item in items {
                match item {
                    syn::Item::Type(type_alias) => {
                        self.process_type_alias_into_namespace(type_alias, namespace)?;
                    },
                    syn::Item::Struct(struct_item) => {
                        self.process_struct_into_namespace(struct_item, namespace)?;
                    },
                    syn::Item::Trait(trait_item) => {
                        self.process_trait_into_namespace(trait_item, namespace)?;
                    },
                    syn::Item::Enum(enum_item) => {
                        self.process_enum_into_namespace(enum_item, namespace)?;
                    },
                    syn::Item::Mod(submodule) => {
                        // Process nested modules recursively
                        let subnamespace_name = submodule.ident.to_string();
                        let subnamespace = self.get_or_create_subnamespace(namespace, &subnamespace_name);
                        
                        if let Some((_, items)) = &submodule.content {
                            for item in items {
                                match item {
                                    syn::Item::Type(type_alias) => {
                                        self.process_type_alias_into_namespace(type_alias, subnamespace)?;
                                    },
                                    syn::Item::Struct(struct_item) => {
                                        self.process_struct_into_namespace(struct_item, subnamespace)?;
                                    },
                                    syn::Item::Trait(trait_item) => {
                                        self.process_trait_into_namespace(trait_item, subnamespace)?;
                                    },
                                    syn::Item::Enum(enum_item) => {
                                        self.process_enum_into_namespace(enum_item, subnamespace)?;
                                    },
                                    // Ignore other items in nested modules
                                    _ => {},
                                }
                            }
                        }
                    },
                    // Ignore other items
                    _ => {},
                }
            }
        }

        Ok(())
    }

    /// Process a type alias declaration
    fn process_type_alias(&mut self, type_alias: &syn::ItemType) -> Result<(), String> {
        let name = type_alias.ident.to_string();
        let documentation = self.extract_documentation(&type_alias.attrs);
        
        // Map the type to Luau
        let type_annotation = self.type_mapper.map_type(&type_alias.ty);
        
        self.root_namespace.types.insert(name.clone(), TypeDeclaration {
            name,
            type_annotation,
            documentation,
        });
        
        Ok(())
    }

    /// Process a type alias into a specific namespace
    fn process_type_alias_into_namespace(&self, type_alias: &syn::ItemType, namespace: &mut TypeNamespace) -> Result<(), String> {
        let name = type_alias.ident.to_string();
        let documentation = self.extract_documentation(&type_alias.attrs);
        
        // Map the type to Luau
        let type_annotation = self.type_mapper.map_type(&type_alias.ty);
        
        namespace.types.insert(name.clone(), TypeDeclaration {
            name,
            type_annotation,
            documentation,
        });
        
        Ok(())
    }

    /// Process a struct declaration
    fn process_struct(&mut self, struct_item: &syn::ItemStruct) -> Result<(), String> {
        let name = struct_item.ident.to_string();
        let documentation = self.extract_documentation(&struct_item.attrs);
        
        // Create an object type for the struct
        let mut fields = HashMap::new();
        
        for field in &struct_item.fields {
            if let Some(field_name) = field.ident.as_ref() {
                let field_name = field_name.to_string();
                let field_type = self.type_mapper.map_type(&field.ty);
                let field_doc = self.extract_documentation(&field.attrs);
                
                // If field has documentation, we could store it somewhere
                if field_doc.is_some() {
                    // For now, we're not using field documentation
                }
                
                fields.insert(field_name, field_type);
            }
        }
        
        let type_annotation = TypeAnnotation::Object(fields);
        
        self.root_namespace.types.insert(name.clone(), TypeDeclaration {
            name,
            type_annotation,
            documentation,
        });
        
        Ok(())
    }

    /// Process a struct into a specific namespace
    fn process_struct_into_namespace(&self, struct_item: &syn::ItemStruct, namespace: &mut TypeNamespace) -> Result<(), String> {
        let name = struct_item.ident.to_string();
        let documentation = self.extract_documentation(&struct_item.attrs);
        
        // Create an object type for the struct
        let mut fields = HashMap::new();
        
        for field in &struct_item.fields {
            if let Some(field_name) = field.ident.as_ref() {
                let field_name = field_name.to_string();
                let field_type = self.type_mapper.map_type(&field.ty);
                let field_doc = self.extract_documentation(&field.attrs);
                
                // If field has documentation, we could store it somewhere
                if field_doc.is_some() {
                    // For now, we're not using field documentation
                }
                
                fields.insert(field_name, field_type);
            }
        }
        
        let type_annotation = TypeAnnotation::Object(fields);
        
        namespace.types.insert(name.clone(), TypeDeclaration {
            name,
            type_annotation,
            documentation,
        });
        
        Ok(())
    }

    /// Process a trait declaration
    fn process_trait(&mut self, trait_item: &syn::ItemTrait) -> Result<(), String> {
        let name = trait_item.ident.to_string();
        let documentation = self.extract_documentation(&trait_item.attrs);
        
        // Create an interface type for the trait
        let mut methods = HashMap::new();
        
        for item in &trait_item.items {
            if let syn::TraitItem::Method(method) = item {
                let method_name = method.sig.ident.to_string();
                let method_doc = self.extract_documentation(&method.attrs);
                
                // Create function type from method signature
                let mut params = Vec::new();
                for input in &method.sig.inputs {
                    if let syn::FnArg::Typed(pat_type) = input {
                        let param_type = self.type_mapper.map_type(&pat_type.ty);
                        params.push(param_type);
                    }
                }
                
                // Map return type
                let return_type = match &method.sig.output {
                    syn::ReturnType::Default => TypeAnnotation::Nil,
                    syn::ReturnType::Type(_, ty) => self.type_mapper.map_type(ty),
                };
                
                let function_type = TypeAnnotation::Function {
                    params: params.iter().enumerate().map(|(i, p)| (format!("arg{}", i), p.clone())).collect(),
                    return_type: Box::new(return_type),
                };
                
                if method_doc.is_some() {
                    // For now, we're not using method documentation
                }
                
                methods.insert(method_name, function_type);
            }
        }
        
        let type_annotation = TypeAnnotation::Interface(methods);
        
        self.root_namespace.types.insert(name.clone(), TypeDeclaration {
            name,
            type_annotation,
            documentation,
        });
        
        Ok(())
    }

    /// Process a trait into a specific namespace
    fn process_trait_into_namespace(&self, trait_item: &syn::ItemTrait, namespace: &mut TypeNamespace) -> Result<(), String> {
        let name = trait_item.ident.to_string();
        let documentation = self.extract_documentation(&trait_item.attrs);
        
        // Create an interface type for the trait
        let mut methods = HashMap::new();
        
        for item in &trait_item.items {
            if let syn::TraitItem::Method(method) = item {
                let method_name = method.sig.ident.to_string();
                let method_doc = self.extract_documentation(&method.attrs);
                
                // Create function type from method signature
                let mut params = Vec::new();
                for input in &method.sig.inputs {
                    if let syn::FnArg::Typed(pat_type) = input {
                        let param_type = self.type_mapper.map_type(&pat_type.ty);
                        params.push(param_type);
                    }
                }
                
                // Map return type
                let return_type = match &method.sig.output {
                    syn::ReturnType::Default => TypeAnnotation::Nil,
                    syn::ReturnType::Type(_, ty) => self.type_mapper.map_type(ty),
                };
                
                let function_type = TypeAnnotation::Function {
                    params: params.iter().enumerate().map(|(i, p)| (format!("arg{}", i), p.clone())).collect(),
                    return_type: Box::new(return_type),
                };
                
                if method_doc.is_some() {
                    // For now, we're not using method documentation
                }
                
                methods.insert(method_name, function_type);
            }
        }
        
        let type_annotation = TypeAnnotation::Interface(methods);
        
        namespace.types.insert(name.clone(), TypeDeclaration {
            name,
            type_annotation,
            documentation,
        });
        
        Ok(())
    }

    /// Process an enum declaration
    fn process_enum(&mut self, enum_item: &syn::ItemEnum) -> Result<(), String> {
        let name = enum_item.ident.to_string();
        let documentation = self.extract_documentation(&enum_item.attrs);
        
        // Create a union type for the enum
        let mut variants = Vec::new();
        
        for variant in &enum_item.variants {
            let variant_name = variant.ident.to_string();
            let variant_doc = self.extract_documentation(&variant.attrs);
            
            // Handle each variant as a distinct type
            let variant_type = match &variant.fields {
                syn::Fields::Named(fields) => {
                    // Named fields become an object type
                    let mut fields_map = HashMap::new();
                    
                    for field in &fields.named {
                        if let Some(field_name) = &field.ident {
                            let field_name = field_name.to_string();
                            let field_type = self.type_mapper.map_type(&field.ty);
                            fields_map.insert(field_name, field_type);
                        }
                    }
                    
                    TypeAnnotation::Object(fields_map)
                },
                syn::Fields::Unnamed(fields) => {
                    // Unnamed fields become a tuple type
                    let mut types = Vec::new();
                    
                    for field in &fields.unnamed {
                        let field_type = self.type_mapper.map_type(&field.ty);
                        types.push(field_type);
                    }
                    
                    TypeAnnotation::Tuple(types)
                },
                syn::Fields::Unit => {
                    // Unit variants become string literals
                    TypeAnnotation::String(Some(variant_name.clone()))
                },
            };
            
            if variant_doc.is_some() {
                // For now, we're not using variant documentation
            }
            
            variants.push(variant_type);
        }
        
        let type_annotation = TypeAnnotation::Union(variants);
        
        self.root_namespace.types.insert(name.clone(), TypeDeclaration {
            name,
            type_annotation,
            documentation,
        });
        
        Ok(())
    }

    /// Process an enum into a specific namespace
    fn process_enum_into_namespace(&self, enum_item: &syn::ItemEnum, namespace: &mut TypeNamespace) -> Result<(), String> {
        let name = enum_item.ident.to_string();
        let documentation = self.extract_documentation(&enum_item.attrs);
        
        // Create a union type for the enum
        let mut variants = Vec::new();
        
        for variant in &enum_item.variants {
            let variant_name = variant.ident.to_string();
            let variant_doc = self.extract_documentation(&variant.attrs);
            
            // Handle each variant as a distinct type
            let variant_type = match &variant.fields {
                syn::Fields::Named(fields) => {
                    // Named fields become an object type
                    let mut fields_map = HashMap::new();
                    
                    for field in &fields.named {
                        if let Some(field_name) = &field.ident {
                            let field_name = field_name.to_string();
                            let field_type = self.type_mapper.map_type(&field.ty);
                            fields_map.insert(field_name, field_type);
                        }
                    }
                    
                    TypeAnnotation::Object(fields_map)
                },
                syn::Fields::Unnamed(fields) => {
                    // Unnamed fields become a tuple type
                    let mut types = Vec::new();
                    
                    for field in &fields.unnamed {
                        let field_type = self.type_mapper.map_type(&field.ty);
                        types.push(field_type);
                    }
                    
                    TypeAnnotation::Tuple(types)
                },
                syn::Fields::Unit => {
                    // Unit variants become string literals
                    TypeAnnotation::String(Some(variant_name.clone()))
                },
            };
            
            if variant_doc.is_some() {
                // For now, we're not using variant documentation
            }
            
            variants.push(variant_type);
        }
        
        let type_annotation = TypeAnnotation::Union(variants);
        
        namespace.types.insert(name.clone(), TypeDeclaration {
            name,
            type_annotation,
            documentation,
        });
        
        Ok(())
    }

    /// Get a namespace or create it if it doesn't exist
    fn get_or_create_namespace(&mut self, namespace_name: &str) -> &mut TypeNamespace {
        if !self.root_namespace.sub_namespaces.contains_key(namespace_name) {
            self.root_namespace.sub_namespaces.insert(namespace_name.to_string(), TypeNamespace {
                name: namespace_name.to_string(),
                types: HashMap::new(),
                sub_namespaces: HashMap::new(),
            });
        }
        
        self.root_namespace.sub_namespaces.get_mut(namespace_name).unwrap()
    }

    /// Get a subnamespace from a parent namespace or create it if it doesn't exist
    fn get_or_create_subnamespace<'a>(&self, parent: &'a mut TypeNamespace, namespace_name: &str) -> &'a mut TypeNamespace {
        if !parent.sub_namespaces.contains_key(namespace_name) {
            parent.sub_namespaces.insert(namespace_name.to_string(), TypeNamespace {
                name: namespace_name.to_string(),
                types: HashMap::new(),
                sub_namespaces: HashMap::new(),
            });
        }
        
        parent.sub_namespaces.get_mut(namespace_name).unwrap()
    }

    /// Extract documentation from attributes
    fn extract_documentation(&self, attrs: &[syn::Attribute]) -> Option<String> {
        let mut doc_lines = Vec::new();
        
        for attr in attrs {
            if attr.path().is_ident("doc") {
                if let Ok(doc) = attr.parse_args::<syn::LitStr>() {
                    doc_lines.push(doc.value());
                }
            }
        }
        
        if doc_lines.is_empty() {
            None
        } else {
            Some(doc_lines.join("\n"))
        }
    }

    /// Generate Luau type definitions from the loaded declarations
    pub fn generate_luau_types(&self) -> String {
        let mut output = String::new();
        
        // Add a header comment
        output.push_str("--[[Generated by Roblox-RS Type Declaration System]]\n");
        output.push_str("--[[DO NOT MODIFY: This file is automatically generated from .d.rs files]]\n\n");
        output.push_str("--!strict\n\n");
        output.push_str("-- Type Definitions for Roblox-RS\n\n");
        
        // Generate types from the global namespace
        self.generate_namespace_types(&self.root_namespace, &mut output, 0);
        
        // Generate types from other namespaces
        for (_, namespace) in &self.root_namespace.sub_namespaces {
            self.generate_namespace_types(namespace, &mut output, 0);
        }
        
        output
    }

    /// Generate types for a namespace
    fn generate_namespace_types(&self, namespace: &TypeNamespace, output: &mut String, indent_level: usize) {
        let indent = "    ".repeat(indent_level);
        
        // Skip the global namespace header
        if namespace.name != "global" {
            output.push_str(&format!("export type {} = {{\n", namespace.name));
        }
        
        // Generate type definitions for this namespace
        for (name, type_decl) in &namespace.types {
            if let Some(doc) = &type_decl.documentation {
                output.push_str(&format!("{}-- {}\n", indent, doc));
            }
            
            if namespace.name == "global" {
                // Top-level types
                output.push_str(&format!("{}export type {} = ", indent, name));
                self.generate_type_annotation(&type_decl.type_annotation, output, indent_level);
                output.push_str("\n\n");
            } else {
                // Namespace member types
                output.push_str(&format!("{}    {}: ", indent, name));
                self.generate_type_annotation(&type_decl.type_annotation, output, indent_level + 1);
                output.push_str(",\n");
            }
        }
        
        // Generate sub-namespaces
        for (_, sub_namespace) in &namespace.sub_namespaces {
            if namespace.name == "global" {
                // Top-level namespace
                self.generate_namespace_types(sub_namespace, output, indent_level);
            } else {
                // Nested namespace
                output.push_str(&format!("{}    {}: {{\n", indent, sub_namespace.name));
                self.generate_namespace_types(sub_namespace, output, indent_level + 1);
                output.push_str(&format!("{}    }},\n", indent));
            }
        }
        
        // Close the namespace if not global
        if namespace.name != "global" {
            output.push_str(&format!("{}}}\n\n", indent));
        }
    }

    /// Generate a type annotation
    fn generate_type_annotation(&self, type_ann: &TypeAnnotation, output: &mut String, indent_level: usize) {
        let indent = "    ".repeat(indent_level);
        
        match type_ann {
            TypeAnnotation::Any => {
                output.push_str("any");
            },
            TypeAnnotation::Number => {
                output.push_str("number");
            },
            TypeAnnotation::String(literal) => {
                if let Some(lit) = literal {
                    output.push_str(&format!("\"{}\"", lit));
                } else {
                    output.push_str("string");
                }
            },
            TypeAnnotation::Boolean => {
                output.push_str("boolean");
            },
            TypeAnnotation::Nil => {
                output.push_str("nil");
            },
            TypeAnnotation::Array(elem_type) => {
                output.push_str("Array<");
                self.generate_type_annotation(elem_type, output, indent_level);
                output.push_str(">");
            },
            TypeAnnotation::Map(key_type, value_type) => {
                output.push_str("Map<");
                self.generate_type_annotation(key_type, output, indent_level);
                output.push_str(", ");
                self.generate_type_annotation(value_type, output, indent_level);
                output.push_str(">");
            },
            TypeAnnotation::Function { params, return_type } => {
                output.push_str("(");
                
                let mut first = true;
                for (name, param_type) in params {
                    if !first {
                        output.push_str(", ");
                    }
                    first = false;
                    
                    output.push_str(&format!("{}: ", name));
                    self.generate_type_annotation(param_type, output, indent_level);
                }
                
                output.push_str(") => ");
                self.generate_type_annotation(return_type, output, indent_level);
            },
            TypeAnnotation::Object(fields) => {
                output.push_str("{\n");
                
                for (name, field_type) in fields {
                    output.push_str(&format!("{}    {}: ", indent, name));
                    self.generate_type_annotation(field_type, output, indent_level + 1);
                    output.push_str(",\n");
                }
                
                output.push_str(&format!("{}}}", indent));
            },
            TypeAnnotation::Tuple(types) => {
                output.push_str("[");
                
                let mut first = true;
                for tuple_type in types {
                    if !first {
                        output.push_str(", ");
                    }
                    first = false;
                    
                    self.generate_type_annotation(tuple_type, output, indent_level);
                }
                
                output.push_str("]");
            },
            TypeAnnotation::Union(types) => {
                let mut first = true;
                for union_type in types {
                    if !first {
                        output.push_str(" | ");
                    }
                    first = false;
                    
                    self.generate_type_annotation(union_type, output, indent_level);
                }
            },
            TypeAnnotation::Interface(methods) => {
                output.push_str("{\n");
                
                for (name, method_type) in methods {
                    output.push_str(&format!("{}    {}: ", indent, name));
                    self.generate_type_annotation(method_type, output, indent_level + 1);
                    output.push_str(",\n");
                }
                
                output.push_str(&format!("{}}}", indent));
            },
            _ => {
                // Default to any for complex types
                output.push_str("any");
            },
        }
    }

    /// Load all .d.rs files from a directory
    pub fn load_declarations_from_directory(&mut self, dir_path: &Path) -> Result<(), String> {
        let entries = fs::read_dir(dir_path)
            .map_err(|e| format!("Failed to read directory: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.ends_with(".d.rs") {
                            self.parse_declaration_file(&path)?;
                        }
                    }
                }
            } else if path.is_dir() {
                self.load_declarations_from_directory(&path)?;
            }
        }
        
        Ok(())
    }
}
