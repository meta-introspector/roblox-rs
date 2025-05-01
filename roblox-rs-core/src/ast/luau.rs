use std::collections::HashMap;

/// Represents a Luau AST node
#[derive(Debug, Clone)]
pub enum LuauNode {
    Program(Program),
    Function(Box<Function>),
    Table(Table),
    Buffer(Buffer),
    Binary {
        left: Box<LuauNode>,
        op: String,
        right: Box<LuauNode>,
    },
    Identifier(String),
    Block(Vec<LuauNode>),
    Return(Box<LuauNode>),
    Local {
        name: String,
        value: Box<LuauNode>,
    },
    If {
        condition: Box<LuauNode>,
        then_branch: Box<LuauNode>,
        else_branch: Option<Box<LuauNode>>,
    },
    Number(f64),
    String(String),
    Boolean(bool),
    Var(String),
    Call {
        func: Box<LuauNode>,
        args: Vec<LuauNode>,
    },
    Literal(LuauValue),
    BinaryExpr {
        left: Box<LuauNode>,
        op: BinaryOp,
        right: Box<LuauNode>,
    },
}

/// A Luau value.
#[derive(Debug, Clone)]
pub enum LuauValue {
    /// A nil value.
    Nil,
    /// A boolean value.
    Bool(bool),
    /// A number value.
    Number(f64),
    /// A string value.
    String(String),
}

/// A binary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Multiplication.
    Mul,
    /// Division.
    Div,
    /// Modulo.
    Mod,
    /// Equality.
    Eq,
    /// Inequality.
    Ne,
    /// Less than.
    Lt,
    /// Less than or equal.
    Le,
    /// Greater than.
    Gt,
    /// Greater than or equal.
    Ge,
    /// Logical and.
    And,
    /// Logical or.
    Or,
}

/// Top-level program node
#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<LuauNode>,
    pub type_annotations: HashMap<String, TypeAnnotation>,
}

/// Function declaration
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: Box<LuauNode>,
}

/// Table declaration with optimization hints
#[derive(Debug, Clone)]
pub struct Table {
    pub fields: Vec<(String, LuauNode)>,
    pub optimization_hints: TableOptimizationHints,
}

/// Buffer-specific node with optimization settings
#[derive(Debug, Clone)]
pub struct Buffer {
    pub initial_size: usize,
    pub element_type: TypeAnnotation,
    pub optimization_level: BufferOptimizationLevel,
}

/// Parameter in function declaration
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<TypeAnnotation>,
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(ty) = &self.type_annotation {
            write!(f, ": {}", ty)?;
        }
        Ok(())
    }
}

/// Type annotation with Luau-specific features
#[derive(Debug, Clone)]
pub enum TypeAnnotation {
    Number,
    String,
    Boolean,
    Any,
    Custom(String),
    Primitive(PrimitiveType),
    Table(Box<TableType>),
    Buffer(Box<BufferType>),
    Function(Box<FunctionType>),
    // Other types will be added as needed
}

impl std::fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeAnnotation::Number => write!(f, "number"),
            TypeAnnotation::String => write!(f, "string"),
            TypeAnnotation::Boolean => write!(f, "boolean"),
            TypeAnnotation::Any => write!(f, "any"),
            TypeAnnotation::Custom(name) => write!(f, "{}", name),
            TypeAnnotation::Primitive(p) => write!(f, "{:?}", p),
            TypeAnnotation::Table(_) => write!(f, "table"),
            TypeAnnotation::Buffer(_) => write!(f, "buffer"),
            TypeAnnotation::Function(_) => write!(f, "function"),
        }
    }
}

impl Default for TypeAnnotation {
    fn default() -> Self {
        Self::Primitive(PrimitiveType::default())
    }
}

/// Table optimization hints
#[derive(Debug, Clone, Default)]
pub struct TableOptimizationHints {
    pub pre_allocate: Option<usize>,
    pub array_like: bool,
    pub stable_keys: bool,
    pub native_buffer_type: Option<String>, // For conversion to Roblox types like Vector3, CFrame, Color3, etc.
    pub table_reuse: bool, // Flag to indicate if this table should be reused (for loop optimization)
}

/// Buffer optimization levels
#[derive(Debug, Clone, Copy)]
pub enum BufferOptimizationLevel {
    Default,
    Speed,
    Size,
}

impl Default for BufferOptimizationLevel {
    fn default() -> Self {
        Self::Default
    }
}

// Type-specific structures
#[derive(Debug, Clone)]
pub enum PrimitiveType {
    Number,
    String,
    Boolean,
    Nil,
}

impl Default for PrimitiveType {
    fn default() -> Self {
        Self::Nil
    }
}

#[derive(Debug, Clone)]
pub struct TableType {
    pub fields: HashMap<String, TypeAnnotation>,
    pub is_array: bool,
}

#[derive(Debug, Clone)]
pub struct BufferType {
    pub element_type: Box<TypeAnnotation>,
    pub fixed_size: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub params: Vec<TypeAnnotation>,
    pub return_type: Box<TypeAnnotation>,
}

/// Representation of a Luau AST.
#[derive(Debug, Clone)]
pub struct LuauAst {
    /// The statements in the AST.
    pub statements: Vec<crate::luau::LuauStmt>,
}

impl LuauAst {
    /// Create a new empty AST
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
    
    /// Add a statement to the AST
    pub fn add_stmt(&mut self, stmt: crate::luau::LuauStmt) {
        self.statements.push(stmt);
    }
}

/// Metadata about the AST.
#[derive(Debug, Clone)]
pub struct AstMetadata {
    /// The source file the AST was parsed from.
    pub source_file: Option<String>,
    /// Whether the AST has been optimized.
    pub optimized: bool,
}
