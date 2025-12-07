use std::{collections::HashMap, hash::Hash};

use crate::{
    ast::{Attribute, DataType, Variant},
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
        variants: Vec<Variant>,
        attributes: Vec<Attribute>,
    },
    Resource {
        values: Vec<String>,
    },
    TypeAlias {
        underlying_type: DataType,
    },
    Field {
        template_name: String,
        is_override: bool,
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
    Block,
}

#[derive(Default, Debug, Clone)]
pub struct ReferenceMap {
    _definitions: HashMap<Span, Span>,
    references: HashMap<Span, Vec<Span>>,
}

impl ReferenceMap {
    pub fn new() -> Self {
        Self {
            _definitions: HashMap::new(),
            references: HashMap::new(),
        }
    }

    pub fn add_reference(&mut self, span: Span, parent_span: &[Span]) -> Option<Vec<Span>> {
        self.references.insert(span, parent_span.to_vec())
    }
}
