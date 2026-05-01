use super::*;
use crate::analyser::symbol_table::SymbolTable;
use crate::ast::{
    ConstraintExpression, ConstraintKind, DataType, DataTypeKind, Element, Expression,
    ExpressionKind, InfixOperator, PrefixOperator,
};
use crate::utils::Span;

fn check_expr(expr: Expression) -> Result<Type, Vec<SemanticError>> {
    let symbol_table = SymbolTable::new();
    let mut checker = TypeChecker::new(&symbol_table);
    let ty = checker.infer_type(&expr);

    if checker.errors.is_empty() {
        Ok(ty)
    } else {
        Err(checker.errors)
    }
}

fn dummy_span() -> Span {
    Span::default()
}

#[test]
fn test_infer_literals() {
    assert_eq!(
        check_expr(Expression {
            kind: ExpressionKind::IntLiteral(42),
            span: dummy_span()
        })
        .unwrap(),
        Type::Int
    );
    assert_eq!(
        check_expr(Expression {
            kind: ExpressionKind::FloatLiteral("3.14".to_string()),
            span: dummy_span()
        })
        .unwrap(),
        Type::Float
    );
    assert_eq!(
        check_expr(Expression {
            kind: ExpressionKind::StringLiteral("test".into()),
            span: dummy_span()
        })
        .unwrap(),
        Type::Str
    );
    assert_eq!(
        check_expr(Expression {
            kind: ExpressionKind::BooleanLiteral(true),
            span: dummy_span()
        })
        .unwrap(),
        Type::Boolean
    );
}

#[test]
fn test_valid_prefix_operators() {
    let expr = Expression {
        kind: ExpressionKind::Prefix {
            operator: PrefixOperator::Negative,
            expression: Box::new(Expression {
                kind: ExpressionKind::IntLiteral(10),
                span: dummy_span(),
            }),
        },
        span: dummy_span(),
    };
    assert_eq!(check_expr(expr).unwrap(), Type::Int);

    let expr = Expression {
        kind: ExpressionKind::Prefix {
            operator: PrefixOperator::LogicalNegate,
            expression: Box::new(Expression {
                kind: ExpressionKind::BooleanLiteral(false),
                span: dummy_span(),
            }),
        },
        span: dummy_span(),
    };
    assert_eq!(check_expr(expr).unwrap(), Type::Boolean);
}

#[test]
fn test_invalid_prefix_operators() {
    let expr = Expression {
        kind: ExpressionKind::Prefix {
            operator: PrefixOperator::Negative,
            expression: Box::new(Expression {
                kind: ExpressionKind::StringLiteral("hello".into()),
                span: dummy_span(),
            }),
        },
        span: dummy_span(),
    };
    let errors = check_expr(expr).unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(matches!(
        errors[0],
        SemanticError::InvalidUnaryOperator { .. }
    ));
}

#[test]
fn test_valid_infix_arithmetic() {
    // Int + Float = Float
    let expr = Expression {
        kind: ExpressionKind::Infix {
            left: Box::new(Expression {
                kind: ExpressionKind::IntLiteral(5),
                span: dummy_span(),
            }),
            operator: InfixOperator::Plus,
            right: Box::new(Expression {
                kind: ExpressionKind::FloatLiteral("5.5".to_string()),
                span: dummy_span(),
            }),
        },
        span: dummy_span(),
    };
    assert_eq!(check_expr(expr).unwrap(), Type::Float);

    // String + String = String
    let expr = Expression {
        kind: ExpressionKind::Infix {
            left: Box::new(Expression {
                kind: ExpressionKind::StringLiteral("A".into()),
                span: dummy_span(),
            }),
            operator: InfixOperator::Plus,
            right: Box::new(Expression {
                kind: ExpressionKind::StringLiteral("B".into()),
                span: dummy_span(),
            }),
        },
        span: dummy_span(),
    };
    assert_eq!(check_expr(expr).unwrap(), Type::Str);
}

#[test]
fn test_invalid_infix_arithmetic() {
    // Int + String = Error
    let expr = Expression {
        kind: ExpressionKind::Infix {
            left: Box::new(Expression {
                kind: ExpressionKind::IntLiteral(5),
                span: dummy_span(),
            }),
            operator: InfixOperator::Plus,
            right: Box::new(Expression {
                kind: ExpressionKind::StringLiteral("A".into()),
                span: dummy_span(),
            }),
        },
        span: dummy_span(),
    };
    let errors = check_expr(expr).unwrap_err();
    assert!(matches!(
        errors[0],
        SemanticError::InvalidBinaryOperator { .. }
    ));
}

#[test]
fn test_logical_operators() {
    // Bool AND Bool = Bool
    let expr = Expression {
        kind: ExpressionKind::Infix {
            left: Box::new(Expression {
                kind: ExpressionKind::BooleanLiteral(true),
                span: dummy_span(),
            }),
            operator: InfixOperator::And,
            right: Box::new(Expression {
                kind: ExpressionKind::BooleanLiteral(false),
                span: dummy_span(),
            }),
        },
        span: dummy_span(),
    };
    assert_eq!(check_expr(expr).unwrap(), Type::Boolean);
}

#[test]
fn test_homogeneous_list() {
    let elements = vec![
        Element {
            value: Expression {
                kind: ExpressionKind::IntLiteral(1),
                span: dummy_span(),
            },
            weight: None,
            span: dummy_span(),
        },
        Element {
            value: Expression {
                kind: ExpressionKind::IntLiteral(2),
                span: dummy_span(),
            },
            weight: None,
            span: dummy_span(),
        },
    ];
    let expr = Expression {
        kind: ExpressionKind::List(elements),
        span: dummy_span(),
    };

    assert_eq!(check_expr(expr).unwrap(), Type::List(Box::new(Type::Int)));
}

#[test]
fn test_heterogeneous_list_errors() {
    let elements = vec![
        Element {
            value: Expression {
                kind: ExpressionKind::IntLiteral(1),
                span: dummy_span(),
            },
            weight: None,
            span: dummy_span(),
        },
        Element {
            value: Expression {
                kind: ExpressionKind::StringLiteral("two".into()),
                span: dummy_span(),
            },
            weight: None,
            span: dummy_span(),
        },
    ];
    let expr = Expression {
        kind: ExpressionKind::List(elements),
        span: dummy_span(),
    };

    let errors = check_expr(expr).unwrap_err();
    assert!(matches!(errors[0], SemanticError::TypeMismatch { .. }));
}

#[test]
fn test_valid_length_constraint() {
    let symbol_table = SymbolTable::new();
    let mut checker = TypeChecker::new(&symbol_table);

    let data_type = DataType {
        kind: DataTypeKind::Str,
        constraints: Some(vec![ConstraintExpression {
            kind: ConstraintKind::Length,
            expression: Expression {
                kind: ExpressionKind::IntLiteral(10),
                span: dummy_span(),
            },
            span: dummy_span(),
        }]),
        span: dummy_span(),
    };

    checker.check_constraints(&data_type, &Type::Str);
    assert!(
        checker.errors.is_empty(),
        "Length constraint on string should not produce errors"
    );
}

#[test]
fn test_invalid_length_constraint_on_int() {
    let symbol_table = SymbolTable::new();
    let mut checker = TypeChecker::new(&symbol_table);

    let data_type = DataType {
        kind: DataTypeKind::Int,
        constraints: Some(vec![ConstraintExpression {
            kind: ConstraintKind::Length,
            expression: Expression {
                kind: ExpressionKind::IntLiteral(10),
                span: dummy_span(),
            },
            span: dummy_span(),
        }]),
        span: dummy_span(),
    };

    checker.check_constraints(&data_type, &Type::Int);
    assert_eq!(checker.errors.len(), 1);
    assert!(matches!(
        checker.errors[0],
        SemanticError::InvalidConstraintForType { .. }
    ));
}
