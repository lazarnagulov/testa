use std::collections::HashMap;

use testa_core::{
    analyser::result::AnalysisResult,
    ast::{Expression, ExpressionKind},
    utils::Span,
};

use crate::{
    AstLowering,
    lowering::context::ItemKind,
    module::node::{Constraint, ConstraintKind, Expr, GlobalItemId, Item, LocalItemId, StringId},
};

impl AstLowering {
    pub(super) fn resolve_item(&mut self, kind: ItemKind, name: &str) -> Option<GlobalItemId> {
        let name = self.string_pool.intern(name);
        self.context.resolve(kind, name)
    }

    pub(super) fn resolve_local_item(
        &mut self,
        kind: ItemKind,
        name: &str,
    ) -> (LocalItemId, StringId) {
        let name_id = self.string_pool.intern(name);

        let id = self.context.resolve(kind, name_id).unwrap_or_else(|| {
            panic!(
                "Item {:?} {:?} '{}' was not registered",
                name_id, kind, name
            )
        });

        (id.item, name_id)
    }

    pub(super) fn next_item_id(&mut self) -> LocalItemId {
        let id = LocalItemId(self.next_item_id);
        self.next_item_id += 1;
        id
    }

    pub(super) fn register_item(&mut self, item: Item, span: Span, name_span: Option<Span>) {
        self.source_map_builder.add_item(item.id(), span);

        if let Some(ns) = name_span {
            self.source_map_builder.add_identifier(item.name(), ns);
        }

        self.items.push(item);
    }

    pub(super) fn extract_constraints(
        &mut self,
        analysis: &AnalysisResult,
        expr: &Expression,
    ) -> Vec<Constraint> {
        let mut constraints = Vec::new();

        if let ExpressionKind::Type(data_type) = &expr.kind
            && let Some(constraint_exprs) = &data_type.constraints
        {
            for constraint_expr in constraint_exprs {
                use testa_core::ast::ConstraintKind as AstConstraintKind;

                let value = self.lower_expr(&constraint_expr.expression, analysis, &HashMap::new());

                let kind = match constraint_expr.kind {
                    AstConstraintKind::Range => {
                        if let ExpressionKind::Infix { left, right, .. } =
                            &constraint_expr.expression.kind
                        {
                            ConstraintKind::Range {
                                min: self.lower_expr(left, analysis, &HashMap::new()),
                                max: self.lower_expr(right, analysis, &HashMap::new()),
                            }
                        } else {
                            ConstraintKind::Range {
                                min: Expr::Int(0),
                                max: value.clone(),
                            }
                        }
                    }
                    AstConstraintKind::Min => ConstraintKind::Min,
                    AstConstraintKind::Max => ConstraintKind::Max,
                    AstConstraintKind::Length => ConstraintKind::Length,
                    AstConstraintKind::MultipleOf => ConstraintKind::MultipleOf,
                    AstConstraintKind::Bias => ConstraintKind::Bias,
                    _ => continue,
                };

                constraints.push(Constraint { kind, value });
            }
        }

        constraints
    }
}
