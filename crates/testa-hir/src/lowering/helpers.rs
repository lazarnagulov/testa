use std::collections::HashMap;

use testa_core::{
    ast::{Expression, ExpressionKind},
    utils::Span,
};

use crate::{
    AstLowering, Item, ItemId, Module,
    module::{Constraint, ConstraintKind, Expr, ItemRef},
};

impl AstLowering {
    pub(super) fn find_item_ref_by_name(
        &mut self,
        name: &str,
        imported: &HashMap<String, &Module>,
    ) -> Option<ItemRef> {
        if let Some(id) = self.find_local_item_id_by_name(name) {
            return Some(ItemRef::Local(id));
        }

        for (module_name, module) in imported {
            if let Some(item) = module.get_item_by_name(name) {
                let module_id = self.string_pool.intern(module_name);
                return Some(ItemRef::Imported {
                    module: module_id,
                    item: item.id(),
                });
            }
        }

        None
    }

    pub(super) fn find_local_item_id_by_name(&self, name: &str) -> Option<ItemId> {
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

        if let ExpressionKind::Type(data_type) = &expr.kind
            && let Some(constraint_exprs) = &data_type.constraints {
                for constraint_expr in constraint_exprs {
                    use testa_core::ast::ConstraintKind as AstConstraintKind;

                    let value = self.lower_expr(&constraint_expr.expression, &HashMap::new());

                    let kind = match constraint_expr.kind {
                        AstConstraintKind::Range => {
                            if let ExpressionKind::Infix { left, right, .. } =
                                &constraint_expr.expression.kind
                            {
                                ConstraintKind::Range {
                                    min: self.lower_expr(left, &HashMap::new()),
                                    max: self.lower_expr(right, &HashMap::new()),
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
