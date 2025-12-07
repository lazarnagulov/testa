use crate::{
    ast::{Attribute, Field, visitor::Visitor},
    symbol_table::{
        SymbolTable,
        symbol::{ScopeKind, SymbolKind},
    },
    utils::Span,
};

pub struct SymbolTableBuilder {
    table: SymbolTable,
}

impl Visitor for SymbolTableBuilder {
    fn visit_template(
        &mut self,
        parent: &Option<String>,
        attributes: &[Attribute],
        name: &str,
        body: &[Field],
        span: Span,
    ) {
        self.table
            .insert(
                name.to_string(),
                SymbolKind::Template {
                    parent: parent.clone(),
                    fields: body.iter().map(|f| f.name.clone()).collect(),
                    attributes: attributes.to_vec(),
                },
                span,
            )
            .ok();

        self.table.enter_scope(ScopeKind::Template);
    }
}


impl SymbolTableBuilder {
    pub fn finalize_template(&mut self) {
        self.table.exit_scope();
    }

    pub fn into_table(self) -> SymbolTable {
        self.table
    }
}