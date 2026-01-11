use crate::{
    analyser::{
        error::SemanticError,
        type_checker::{TypeChecker, types::Type},
    },
    ast::{Expression, InfixOperator},
};

impl<'a> TypeChecker<'a> {
    pub(super) fn check_infix_op(
        &mut self,
        left: &Expression,
        op: InfixOperator,
        right: &Expression,
    ) -> Type {
        let left_type = self.infer_type(left);
        let right_type = self.infer_type(right);

        match op {
            InfixOperator::Plus => {
                if left_type.is_numeric() && right_type.is_numeric() {
                    if left_type == Type::Float || right_type == Type::Float {
                        Type::Float
                    } else {
                        Type::Int
                    }
                } else if matches!(left_type, Type::Str) && matches!(right_type, Type::Str) {
                    Type::Str
                } else if !left_type.is_unknown() && !right_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidBinaryOperator {
                        operator: op.to_string(),
                        left_type: left_type.display(),
                        right_type: right_type.display(),
                        span: left.span.merge(right.span),
                    });
                    Type::Unknown
                } else {
                    Type::Unknown
                }
            }

            InfixOperator::Minus | InfixOperator::Multiply | InfixOperator::Divide => {
                if !left_type.is_numeric() && !left_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidBinaryOperator {
                        operator: op.to_string(),
                        left_type: left_type.display(),
                        right_type: right_type.display(),
                        span: left.span.merge(right.span),
                    });
                    return Type::Unknown;
                }

                if !right_type.is_numeric() && !right_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidBinaryOperator {
                        operator: op.to_string(),
                        left_type: left_type.display(),
                        right_type: right_type.display(),
                        span: left.span.merge(right.span),
                    });
                    return Type::Unknown;
                }

                if left_type == Type::Float || right_type == Type::Float {
                    Type::Float
                } else {
                    Type::Int
                }
            }

            InfixOperator::Mod => {
                if !matches!(left_type, Type::Int) && !left_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidBinaryOperator {
                        operator: op.to_string(),
                        left_type: left_type.display(),
                        right_type: right_type.display(),
                        span: left.span.merge(right.span),
                    });
                }
                if !matches!(right_type, Type::Int) && !right_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidBinaryOperator {
                        operator: op.to_string(),
                        left_type: left_type.display(),
                        right_type: right_type.display(),
                        span: left.span.merge(right.span),
                    });
                }
                Type::Int
            }

            InfixOperator::BitAnd
            | InfixOperator::BitOr
            | InfixOperator::BitXor
            | InfixOperator::BitLShift
            | InfixOperator::BitRShift => {
                if !matches!(left_type, Type::Int) && !left_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidBinaryOperator {
                        operator: op.to_string(),
                        left_type: left_type.display(),
                        right_type: right_type.display(),
                        span: left.span.merge(right.span),
                    });
                }
                if !matches!(right_type, Type::Int) && !right_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidBinaryOperator {
                        operator: op.to_string(),
                        left_type: left_type.display(),
                        right_type: right_type.display(),
                        span: left.span.merge(right.span),
                    });
                }
                Type::Int
            }

            InfixOperator::And | InfixOperator::Or => {
                if !matches!(left_type, Type::Boolean) && !left_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidBinaryOperator {
                        operator: op.to_string(),
                        left_type: left_type.display(),
                        right_type: right_type.display(),
                        span: left.span.merge(right.span),
                    });
                }
                if !matches!(right_type, Type::Boolean) && !right_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidBinaryOperator {
                        operator: op.to_string(),
                        left_type: left_type.display(),
                        right_type: right_type.display(),
                        span: left.span.merge(right.span),
                    });
                }
                Type::Boolean
            }

            InfixOperator::Equal
            | InfixOperator::NotEqual
            | InfixOperator::LessThan
            | InfixOperator::GreaterThan
            | InfixOperator::LessThanOrEqual
            | InfixOperator::GreaterThanOrEqual => {
                if !left_type.is_compatible_with(&right_type)
                    && !left_type.is_unknown()
                    && !right_type.is_unknown()
                {
                    self.errors.push(SemanticError::InvalidBinaryOperator {
                        operator: op.to_string(),
                        left_type: left_type.display(),
                        right_type: right_type.display(),
                        span: left.span.merge(right.span),
                    });
                }
                Type::Boolean
            }

            InfixOperator::ExclusiveRange | InfixOperator::InclusiveRange => {
                if !left_type.is_numeric() && !left_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidBinaryOperator {
                        operator: op.to_string(),
                        left_type: left_type.display(),
                        right_type: right_type.display(),
                        span: left.span.merge(right.span),
                    });
                }
                if !right_type.is_numeric() && !right_type.is_unknown() {
                    self.errors.push(SemanticError::InvalidBinaryOperator {
                        operator: op.to_string(),
                        left_type: left_type.display(),
                        right_type: right_type.display(),
                        span: left.span.merge(right.span),
                    });
                }

                Type::Range
            }
        }
    }
}
