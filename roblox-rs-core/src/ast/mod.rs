//! Abstract Syntax Tree (AST) definitions and utilities.
//!
//! This module provides the AST structures for both Rust and Luau,
//! as well as utilities for parsing, transforming, and generating code.

pub mod luau;
pub mod parser;
pub mod transformer;
pub mod visitor;

use std::collections::HashMap;

/// A span of source code, represented by start and end byte indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// A simple identifier with an optional namespace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident {
    pub name: String,
    pub namespace: Option<String>,
    pub span: Span,
}

/// An AST node with attached span information.
#[derive(Debug, Clone, PartialEq)]
pub struct Node<T> {
    pub span: Span,
    pub node: T,
}

/// Type of literal value.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Nil,
}

/// A type reference, which can be a primitive type, a custom type, or a generic type.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeRef {
    /// A primitive type (i32, bool, etc.)
    Primitive(String),
    /// A custom type (MyStruct, etc.)
    Custom(Ident),
    /// A generic type (Vec<T>, etc.)
    Generic {
        base: Box<TypeRef>,
        params: Vec<TypeRef>,
    },
    /// A function type (Fn(A, B) -> C)
    Function {
        params: Vec<TypeRef>,
        return_type: Box<TypeRef>,
    },
    /// A tuple type ((A, B, C))
    Tuple(Vec<TypeRef>),
    /// An array type ([T; N])
    Array {
        element_type: Box<TypeRef>,
        size: Option<usize>,
    },
    /// A reference type (&T or &mut T)
    Reference {
        target: Box<TypeRef>,
        mutable: bool,
    },
}

/// A simple name->value attribute.
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
}

/// Attributes attached to an AST node.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Attributes {
    pub attrs: HashMap<String, Option<String>>,
}

impl Attributes {
    pub fn new() -> Self {
        Self {
            attrs: HashMap::new(),
        }
    }

    pub fn add(&mut self, attr: Attribute) {
        self.attrs.insert(attr.name, attr.value);
    }

    pub fn has(&self, name: &str) -> bool {
        self.attrs.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<&Option<String>> {
        self.attrs.get(name)
    }
}

/// Representation of a Luau AST.
pub use self::luau::LuauAst;

/// A node in the Luau AST.
pub use self::luau::LuauNode;

/// Metadata about the AST.
pub use self::luau::AstMetadata;

/// A Luau value.
pub use self::luau::LuauValue;

/// A binary operator.
pub use self::luau::BinaryOp;
