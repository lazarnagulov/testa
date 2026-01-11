use crate::{
    analyser::{
        error::SemanticError,
        type_checker::{TypeChecker, types::Type},
    },
    ast::{Expression, PrefixOperator},
};

impl<'a> TypeChecker<'a> {
    pub(super) fn check_prefix_op(&mut self, op: PrefixOperator, expr: &Expression) -> Type {
        let expr_type = self.infer_type(expr);

        match op {
            PrefixOperator::Negative => {
                if !expr_type.is_numeric() && !expr_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidUnaryOperator {
                        operator: op.to_string(),
                        operand_type: expr_type.display(),
                        span: expr.span,
                    });
                }
                expr_type
            }
            PrefixOperator::LogicalNegate => {
                if !matches!(expr_type, Type::Boolean) && !expr_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidUnaryOperator {
                        operator: op.to_string(),
                        operand_type: expr_type.display(),
                        span: expr.span,
                    });
                }
                Type::Boolean
            }
            PrefixOperator::BitNegate => {
                if !matches!(expr_type, Type::Int) && !expr_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidUnaryOperator {
                        operator: op.to_string(),
                        operand_type: expr_type.display(),
                        span: expr.span,
                    });
                }
                Type::Int
            }
        }
    }
}
