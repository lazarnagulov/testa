use crate::{
    analyser::{
        error::SemanticError,
        type_checker::{TypeChecker, types::Type},
    },
    ast::{ConstraintExpression, ConstraintKind, DataType},
    utils::Span,
};

impl<'a> TypeChecker<'a> {
    pub(super) fn check_constraints(&mut self, data_type: &DataType, base_type: &Type) {
        if let Some(constraints) = &data_type.constraints {
            for constraint in constraints {
                self.validate_constraint(constraint, base_type, data_type.span);
            }
        }
    }

    fn validate_constraint(
        &mut self,
        constraint: &ConstraintExpression,
        base_type: &Type,
        span: Span,
    ) {
        match constraint.kind {
            ConstraintKind::Range => {
                if !base_type.is_numeric() && !base_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidConstraintForType {
                        constraint: "range".to_string(),
                        type_name: base_type.display(),
                        span,
                    });
                }

                let expr_type = self.infer_type(&constraint.expression);
                if !matches!(expr_type, Type::Range) && !expr_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidConstraintValue {
                        constraint: "range".to_string(),
                        expected: "range expression (e.g., 1..10)".to_string(),
                        found: expr_type.display(),
                        span: constraint.span,
                    });
                }
            }

            ConstraintKind::Min | ConstraintKind::Max => {
                if !base_type.is_numeric() && !base_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidConstraintForType {
                        constraint: constraint.kind.to_string(),
                        type_name: base_type.display(),
                        span,
                    });
                }

                let expr_type = self.infer_type(&constraint.expression);
                if !expr_type.is_numeric() && !expr_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidConstraintValue {
                        constraint: constraint.kind.to_string(),
                        expected: "numeric value".to_string(),
                        found: expr_type.display(),
                        span: constraint.span,
                    });
                }
            }

            ConstraintKind::Length => {
                if !matches!(base_type, Type::Str) && !base_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidConstraintForType {
                        constraint: "length".to_string(),
                        type_name: base_type.display(),
                        span,
                    });
                }

                let expr_type = self.infer_type(&constraint.expression);
                if !matches!(expr_type, Type::Int) && !expr_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidConstraintValue {
                        constraint: "length".to_string(),
                        expected: "int".to_string(),
                        found: expr_type.display(),
                        span: constraint.span,
                    });
                }
            }

            ConstraintKind::Count => {
                if !matches!(base_type, Type::List(_)) && !base_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidConstraintForType {
                        constraint: "count".to_string(),
                        type_name: base_type.display(),
                        span,
                    });
                }

                let expr_type = self.infer_type(&constraint.expression);
                if !matches!(expr_type, Type::Range | Type::Int) && !expr_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidConstraintValue {
                        constraint: "count".to_string(),
                        expected: "int value or range expression (e.g., 1..10)".to_string(),
                        found: expr_type.display(),
                        span: constraint.span,
                    });
                }
            }

            ConstraintKind::MultipleOf => {
                if !base_type.is_numeric() && !base_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidConstraintForType {
                        constraint: "multiple_of".to_string(),
                        type_name: base_type.display(),
                        span,
                    });
                }

                let expr_type = self.infer_type(&constraint.expression);
                if !expr_type.is_numeric() && !expr_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidConstraintValue {
                        constraint: "multiple_of".to_string(),
                        expected: "numeric value".to_string(),
                        found: expr_type.display(),
                        span: constraint.span,
                    });
                }
            }

            ConstraintKind::Bias => {
                if !matches!(base_type, Type::Boolean) && !base_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidConstraintForType {
                        constraint: "bias".to_string(),
                        type_name: base_type.display(),
                        span,
                    });
                }

                let expr_type = self.infer_type(&constraint.expression);
                if !expr_type.is_numeric() && !expr_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidConstraintValue {
                        constraint: "bias".to_string(),
                        expected: "numeric value".to_string(),
                        found: expr_type.display(),
                        span: constraint.span,
                    });
                }
            }

            ConstraintKind::Custom => {
                self.infer_type(&constraint.expression);
            }
        }
    }
}
