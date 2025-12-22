use std::{collections::HashMap, hash::Hash};

use crate::{ast::Attribute, utils::Span};

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
        variants: Vec<String>,
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
