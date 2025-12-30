use std::{collections::HashMap, hash::Hash};

use crate::{
    ast::{Attribute, Expression},
    utils::Span,
};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub span: Span,
    pub scope_id: ScopeId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Template {
        fields: Vec<String>,
        parent: Option<String>,
        attributes: Vec<Attribute>,
    },
    Enum {
        variants: Vec<VariantInfo>,
        attributes: Vec<Attribute>,
    },
    Resource {
        values: Vec<String>,
    },
    TypeAlias {
        name: String,
        attributes: Vec<Attribute>,
    },
    Variant {
        enum_name: String,
    },
    Field {
        template_name: String,
        is_override: bool,
        expression: Expression,
    },
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub id: ScopeId,
    pub parent: Option<ScopeId>,
    pub symbols: HashMap<String, Symbol>,
    pub kind: ScopeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ScopeId(pub usize);

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeKind {
    Global,
    Template { name: String },
    Enum { name: String },
    Generate { name: String },
    Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantInfo {
    pub name: String,
    pub weight: Option<Expression>,
}

impl VariantInfo {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            weight: None,
        }
    }

    pub fn with_weight(mut self, weight_expression: Expression) -> Self {
        self.weight = Some(weight_expression);
        self
    }
}
