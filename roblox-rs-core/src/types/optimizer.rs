use std::collections::{HashMap, HashSet};
use super::TypeMapper;

/// Type-based optimization system
pub struct TypeOptimizer {
    // Track type specializations
    specializations: HashMap<String, TypeSpecialization>,
    // Track type layouts
    layouts: HashMap<String, TypeLayout>,
    // Track trait implementations
    trait_impls: HashMap<String, HashSet<String>>,
    // Optimization hints based on types
    type_hints: Vec<TypeOptimizationHint>,
}

#[derive(Debug, Clone)]
pub struct TypeSpecialization {
    pub original_type: String,
    pub specialized_type: String,
    pub conditions: Vec<SpecializationCondition>,
    pub optimizations: Vec<TypeOptimization>,
}

#[derive(Debug, Clone)]
pub struct TypeLayout {
    pub fields: Vec<FieldInfo>,
    pub size: usize,
    pub alignment: usize,
    pub optimizations: Vec<LayoutOptimization>,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
    pub offset: usize,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub enum SpecializationCondition {
    SizeBelow(usize),
    SingleUse,
    ConstantValue,
    NoMutation,
}

#[derive(Debug, Clone)]
pub enum TypeOptimization {
    Inline,
    Unboxed,
    Vectorize,
    ConstPropagate,
    CustomLayout(String),
}

#[derive(Debug, Clone)]
pub enum LayoutOptimization {
    PackedFields,
    AlignedAccess,
    CacheOptimized,
    SplitHotCold,
}

#[derive(Debug, Clone)]
pub enum TypeOptimizationHint {
    UseSpecialization(String, String),
    OptimizeLayout(String, LayoutOptimization),
    VectorizeOperations(String),
    InlineType(String),
}

impl TypeOptimizer {
    pub fn new() -> Self {
        Self {
            specializations: HashMap::new(),
            layouts: HashMap::new(),
            trait_impls: HashMap::new(),
            type_hints: Vec::new(),
        }
    }

    /// Analyze a type for optimization opportunities
    pub fn analyze_type(&mut self, type_name: &str, type_info: &TypeMapper) -> Vec<TypeOptimizationHint> {
        let mut hints = Vec::new();

        // Check for specialization opportunities
        if let Some(spec) = self.check_specialization(type_name, type_info) {
            hints.push(TypeOptimizationHint::UseSpecialization(
                type_name.to_string(),
                spec.specialized_type,
            ));
        }

        // Check layout optimizations
        if let Some(layout) = self.analyze_layout(type_name, type_info) {
            for opt in &layout.optimizations {
                hints.push(TypeOptimizationHint::OptimizeLayout(
                    type_name.to_string(),
                    opt.clone(),
                ));
            }
        }

        // Check for vectorization
        if self.can_vectorize(type_name, type_info) {
            hints.push(TypeOptimizationHint::VectorizeOperations(
                type_name.to_string(),
            ));
        }

        // Check for inlining
        if self.should_inline(type_name, type_info) {
            hints.push(TypeOptimizationHint::InlineType(
                type_name.to_string(),
            ));
        }

        hints
    }

    /// Check if a type can be specialized
    fn check_specialization(&self, type_name: &str, type_info: &TypeMapper) -> Option<TypeSpecialization> {
        let size = type_info.get_type_size(type_name)?;
        let mutability = type_info.get_type_mutability(type_name)?;
        let usage = type_info.get_type_usage(type_name)?;

        let mut conditions = Vec::new();
        let mut optimizations = Vec::new();

        // Check size-based specialization
        if size < 16 {
            conditions.push(SpecializationCondition::SizeBelow(16));
            optimizations.push(TypeOptimization::Inline);
        }

        // Check usage-based specialization
        if usage.references == 1 {
            conditions.push(SpecializationCondition::SingleUse);
            optimizations.push(TypeOptimization::Unboxed);
        }

        // Check mutability-based specialization
        if !mutability.is_mutable {
            conditions.push(SpecializationCondition::NoMutation);
            optimizations.push(TypeOptimization::ConstPropagate);
        }

        if !optimizations.is_empty() {
            Some(TypeSpecialization {
                original_type: type_name.to_string(),
                specialized_type: format!("Specialized{}", type_name),
                conditions,
                optimizations,
            })
        } else {
            None
        }
    }

    /// Analyze type layout for optimizations
    fn analyze_layout(&self, type_name: &str, type_info: &TypeMapper) -> Option<TypeLayout> {
        let fields = type_info.get_type_fields(type_name)?;
        let mut layout_fields = Vec::new();
        let mut current_offset = 0;

        // Sort fields by size and access frequency
        let mut sorted_fields = fields.clone();
        sorted_fields.sort_by_key(|f| (type_info.get_field_access_frequency(&f.name), f.size));

        for field in sorted_fields {
            let size = field.size;
            let alignment = field.alignment;
            
            // Align field
            current_offset = (current_offset + alignment - 1) & !(alignment - 1);
            
            layout_fields.push(FieldInfo {
                name: field.name.clone(),
                type_name: field.type_name.clone(),
                offset: current_offset,
                size,
            });
            
            current_offset += size;
        }

        let mut optimizations = Vec::new();

        // Check if we can pack fields
        if current_offset > 64 {
            optimizations.push(LayoutOptimization::PackedFields);
        }

        // Check if we need aligned access
        if layout_fields.iter().any(|f| f.size >= 8) {
            optimizations.push(LayoutOptimization::AlignedAccess);
        }

        // Check if we should optimize for cache
        if layout_fields.len() > 8 {
            optimizations.push(LayoutOptimization::CacheOptimized);
        }

        Some(TypeLayout {
            fields: layout_fields,
            size: current_offset,
            alignment: 8, // Default alignment
            optimizations,
        })
    }

    /// Check if operations on this type can be vectorized
    fn can_vectorize(&self, type_name: &str, type_info: &TypeMapper) -> bool {
        // Check if type represents a numeric array or vector
        if let Some(element_type) = type_info.get_element_type(type_name) {
            match element_type.as_str() {
                "f32" | "f64" | "i32" | "i64" => true,
                _ => false,
            }
        } else {
            false
        }
    }

    /// Check if type should be inlined
    fn should_inline(&self, type_name: &str, type_info: &TypeMapper) -> bool {
        if let Some(size) = type_info.get_type_size(type_name) {
            // Inline small types that are frequently used
            size <= 32 && type_info.get_type_usage(type_name)
                .map(|u| u.references > 5)
                .unwrap_or(false)
        } else {
            false
        }
    }

    /// Generate Luau code for optimized type
    pub fn generate_type_code(&self, type_name: &str, layout: &TypeLayout) -> String {
        let mut code = String::new();

        // Generate optimized constructor
        code.push_str(&format!("function create{}(", type_name));
        let params: Vec<_> = layout.fields.iter()
            .map(|f| f.name.as_str())
            .collect();
        code.push_str(&params.join(", "));
        code.push_str(") \n");

        // Pre-allocate table if needed
        if layout.fields.len() > 4 {
            code.push_str("    local t = table.create(");
            code.push_str(&layout.fields.len().to_string());
            code.push_str(")\n");
        } else {
            code.push_str("    local t = {}\n");
        }

        // Initialize fields in optimal order
        for field in &layout.fields {
            code.push_str(&format!("    t.{} = {}\n", field.name, field.name));
        }

        code.push_str("    return t\n");
        code.push_str("end\n");

        code
    }

    /// Add a trait implementation
    pub fn add_trait_impl(&mut self, type_name: String, trait_name: String) {
        self.trait_impls
            .entry(type_name)
            .or_default()
            .insert(trait_name);
    }

    /// Get optimization hints for a type
    pub fn get_hints(&self) -> &[TypeOptimizationHint] {
        &self.type_hints
    }
}
