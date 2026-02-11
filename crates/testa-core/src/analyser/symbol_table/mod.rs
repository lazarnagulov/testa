pub mod symbol;
pub mod symbol_table_builder;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use crate::{
    analyser::{
        error::SemanticError,
        symbol_table::symbol::{Scope, ScopeId, ScopeKind, Symbol, SymbolKind},
    },
    utils::Span,
};

#[derive(Default, Debug, Clone)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    global_scope: ScopeId,
    next_scope_id: usize,
    current_scope: ScopeId,
}

impl SymbolTable {
    pub fn new() -> Self {
        let global_scope = ScopeId(0);
        let global = Scope {
            id: global_scope,
            parent: None,
            symbols: HashMap::new(),
            kind: ScopeKind::Global,
        };
        Self {
            scopes: vec![global],
            global_scope: ScopeId(0),
            next_scope_id: 1,
            current_scope: global_scope,
        }
    }

    pub fn scopes(&self) -> &Vec<Scope> {
        &self.scopes
    }

    pub fn global_scope(&self) -> ScopeId {
        self.global_scope
    }

    pub fn enter_scope(&mut self, kind: ScopeKind) -> ScopeId {
        let scope_id = ScopeId(self.next_scope_id);
        self.next_scope_id += 1;

        let scope = Scope {
            id: scope_id,
            parent: Some(self.current_scope),
            symbols: HashMap::new(),
            kind,
        };

        self.scopes.push(scope);
        self.current_scope = scope_id;
        scope_id
    }

    pub fn exit_scope(&mut self) {
        if let Some(scope) = self.get_scope(self.current_scope) {
            if let Some(parent) = scope.parent {
                self.current_scope = parent;
            }
        }
    }

    fn get_scope(&self, id: ScopeId) -> Option<&Scope> {
        self.scopes.iter().find(|s| s.id == id)
    }

    fn get_scope_mut(&mut self, id: ScopeId) -> Option<&mut Scope> {
        self.scopes.iter_mut().find(|s| s.id == id)
    }

    pub fn get_current_scope(&self) -> Option<&Scope> {
        self.get_scope(self.current_scope)
    }

    pub fn current_scope_kind(&self) -> Option<&ScopeKind> {
        self.get_current_scope().map(|s| &s.kind)
    }

    pub fn get_definition_span(&self, name: &str) -> Option<Span> {
        self.lookup(name).map(|symbol| symbol.span)
    }

    pub fn get_template(&self, name: &str) -> Option<&Symbol> {
        self.lookup(name).and_then(|symbol| match &symbol.kind {
            SymbolKind::Template { .. } => Some(symbol),
            _ => None,
        })
    }

    pub fn get_all_symbols(&self) -> Vec<&Symbol> {
        self.scopes
            .iter()
            .flat_map(|scope| scope.symbols.values())
            .collect()
    }

    pub fn get_type(&self, name: &str) -> Option<&Symbol> {
        self.lookup(name).and_then(|symbol| match &symbol.kind {
            SymbolKind::TypeAlias { .. } => Some(symbol),
            _ => None,
        })
    }

    pub fn get_enum(&self, name: &str) -> Option<&Symbol> {
        self.lookup(name).and_then(|symbol| match &symbol.kind {
            SymbolKind::Enum { .. } => Some(symbol),
            _ => None,
        })
    }

    pub fn get_symbol_kind(&self, name: &str) -> Option<&SymbolKind> {
        self.lookup(name).map(|symbol| &symbol.kind)
    }

    pub fn lookup_current_scope(&self, name: &str) -> Option<&Symbol> {
        self.get_scope(self.current_scope)
            .and_then(|scope| scope.symbols.get(name))
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        let mut current = self.current_scope;

        loop {
            if let Some(scope) = self.get_scope(current) {
                if let Some(symbol) = scope.symbols.get(name) {
                    return Some(symbol);
                }

                match scope.parent {
                    Some(parent) => current = parent,
                    None => return None,
                }
            } else {
                return None;
            }
        }
    }

    pub fn insert(
        &mut self,
        name: String,
        kind: SymbolKind,
        span: Span,
    ) -> Result<(), SemanticError> {
        if self.lookup_current_scope(&name).is_some() {
            return Err(SemanticError::DuplicateDeclaration {
                span,
                message: format!(
                    "Symbol '{}' already declared in this scope at {}",
                    name, span
                ),
            });
        }

        let symbol = Symbol {
            name: name.clone(),
            kind,
            span,
            scope_id: self.current_scope,
        };

        if let Some(scope) = self.get_scope_mut(self.current_scope) {
            scope.symbols.insert(name, symbol);
            Ok(())
        } else {
            Err(SemanticError::InvalidContext {
                span,
                message: "Invalid scope".to_string(),
            })
        }
    }

    pub fn get_template_parents(&self, template_name: &str) -> Vec<String> {
        let mut parents = Vec::new();
        let mut current = template_name.to_string();

        while let Some(symbol) = self.lookup(&current) {
            if let SymbolKind::Template {
                parent: Some(parent),
                ..
            } = &symbol.kind
            {
                parents.push(parent.clone());
                current = parent.clone();
            } else {
                break;
            }
        }

        parents
    }

    pub fn dump(&self) {
        println!("=== Symbol Table ===");
        for scope in &self.scopes {
            println!("\nScope {:?} ({:?}):", scope.id, scope.kind);
            if let Some(parent) = scope.parent {
                println!("  Parent: {:?}", parent);
            }
            for (name, symbol) in &scope.symbols {
                println!("  {} ({:?}) at {:?}", name, symbol.kind, symbol.span);
            }
        }
    }

    pub fn check_inheritance_cycle(&self, template_name: &str) -> Result<Vec<String>, Vec<String>> {
        let mut visited = std::collections::HashSet::new();
        let mut path = Vec::new();
        let mut current = template_name.to_string();

        visited.insert(current.clone());
        path.push(current.clone());

        loop {
            let symbol = match self.lookup(&current) {
                Some(s) => s,
                None => {
                    return Ok(path);
                }
            };

            let parent = match &symbol.kind {
                SymbolKind::Template {
                    parent: Some(p), ..
                } => p.clone(),
                _ => {
                    return Ok(path);
                }
            };

            if visited.contains(&parent) {
                path.push(parent.clone());

                let cycle_start = path.iter().position(|t| t == &parent).unwrap();
                let cycle = path[cycle_start..].to_vec();

                return Err(cycle);
            }

            visited.insert(parent.clone());
            path.push(parent.clone());
            current = parent;
        }
    }

    pub fn get_inheritance_chain(&self, template_name: &str) -> Option<Vec<String>> {
        self.check_inheritance_cycle(template_name).ok()
    }
}
