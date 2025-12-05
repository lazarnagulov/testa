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
    pub documentation: Option<String>,
}

#[derive(Default, Debug, Clone)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
    scopes: Vec<Scope>,
    global_scope: ScopeId,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = Self {
            symbols: HashMap::new(),
            scopes: Vec::new(),
            global_scope: ScopeId(0),
        };

        let global = table.create_scope(None, ScopeKind::Global);
        table.global_scope = global;
        table
    }

    pub fn scopes(&self) -> &Vec<Scope> {
        &self.scopes
    }

    pub fn global_scope(&self) -> ScopeId {
        self.global_scope
    }

    pub fn create_scope(&mut self, parent: Option<ScopeId>, kind: ScopeKind) -> ScopeId {
        let id = ScopeId(self.scopes.len());
        let scope = Scope {
            id,
            parent,
            symbols: HashMap::new(),
            kind,
        };
        self.scopes.push(scope);
        id
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Template {
        fields: Vec<FieldSymbol>,
        parent: Option<String>,
        attributes: Vec<Attribute>,
    },
    Enum {
        variants: Vec<Variant>,
    },
    Resource {
        values: Vec<String>,
    },
    TypeAlias {
        underlying_type: DataType,
    },
    Field {
        type_info: DataType,
        is_override: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldSymbol {
    pub name: String,
    pub type_info: DataType,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub id: ScopeId,
    pub parent: Option<ScopeId>,
    pub symbols: HashMap<String, String>,
    pub kind: ScopeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ScopeId(usize);

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeKind {
    Global,
    Template,
    Block,
}

#[derive(Default, Debug, Clone)]
pub struct ReferenceMap {
    definitions: HashMap<Span, Span>,
    references: HashMap<Span, Vec<Span>>,
}

impl ReferenceMap {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            references: HashMap::new(),
        }
    }
}
