use testa_core::{
    ast::{Expression, ExpressionKind},
    utils::Span,
};

use crate::{
    AstLowering, Item, ItemId,
    module::{Constraint, ConstraintKind, Expr},
};

impl AstLowering {
    pub(super) fn find_item_id_by_name(&self, name: &str) -> Option<ItemId> {
        self.items
            .iter()
            .find(|item| self.string_pool.resolve(item.name()) == name)
            .map(Item::id)
    }

    pub(super) fn next_item_id(&mut self) -> ItemId {
        let id = ItemId(self.next_item_id);
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

    pub(super) fn extract_constraints(&mut self, expr: &Expression) -> Vec<Constraint> {
        let mut constraints = Vec::new();

        if let ExpressionKind::Type(data_type) = &expr.kind {
            if let Some(constraint_exprs) = &data_type.constraints {
                for constraint_expr in constraint_exprs {
                    use testa_core::ast::ConstraintKind as AstConstraintKind;

                    let value = self.lower_expr(&constraint_expr.expression);

                    let kind = match constraint_expr.kind {
                        AstConstraintKind::Range => {
                            if let ExpressionKind::Infix { left, right, .. } =
                                &constraint_expr.expression.kind
                            {
                                ConstraintKind::Range {
                                    min: self.lower_expr(left),
                                    max: self.lower_expr(right),
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
        }

        constraints
    }
}
