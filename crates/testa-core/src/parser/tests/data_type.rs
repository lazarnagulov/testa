use crate::{
    ast::{ConstraintKind, DataTypeKind, Expression, ExpressionKind, InfixOperator, Statement},
    lexer::token::TokenKind,
    parser::tests::{identifier, int_literal, parser_from_tokens, span, token},
};

#[test]
fn test_parse_type_declaration() {
    let name_span = span(0, 1, 1, 5, 1, 6);

    // type Testa = int;
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Type)),
            Ok(identifier(name_span)),
            Ok(token(TokenKind::SingleEqual)),
            Ok(token(TokenKind::Int)),
            Ok(token(TokenKind::Semicolon)),
        ],
        "Testa",
    );

    let program = parser.parse().expect("parse failed");
    let Statement::TypeDecl { name, .. } = &program.0[0] else {
        panic!("expected type declaration");
    };

    assert_eq!(name, "Testa");
}

#[test]
fn test_parse_type_with_constraints() {
    let constraint_span = span(0, 1, 1, 5, 1, 6);
    let lower_span = span(6, 1, 7, 7, 1, 8);
    let upper_span = span(8, 1, 9, 9, 1, 10);

    // type <> = int [range=1..=2];
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Type)),
            Ok(token(TokenKind::Identifier)),
            Ok(token(TokenKind::SingleEqual)),
            Ok(token(TokenKind::Int)),
            Ok(token(TokenKind::LBracket)),
            Ok(identifier(constraint_span)),
            Ok(token(TokenKind::SingleEqual)),
            Ok(int_literal(lower_span)),
            Ok(token(TokenKind::DoublePeriodEqual)),
            Ok(int_literal(upper_span)),
            Ok(token(TokenKind::RBracket)),
            Ok(token(TokenKind::Semicolon)),
        ],
        "range 1 2",
    );

    let program = parser.parse().expect("parse failed");
    let Statement::TypeDecl { data_type, .. } = &program.0[0] else {
        panic!("expected type declaration");
    };

    let ExpressionKind::Type(data_type) = &data_type.kind else {
        panic!("expected type expression");
    };

    assert!(matches!(data_type.kind, DataTypeKind::Int));
    let constraints = data_type.constraints.as_ref().expect("missing constraints");
    assert_eq!(constraints.len(), 1);

    assert_eq!(
        constraints[0].expression,
        Expression {
            kind: ExpressionKind::Infix {
                left: Box::new(Expression {
                    kind: ExpressionKind::IntLiteral(1),
                    span: lower_span,
                }),
                operator: InfixOperator::InclusiveRange,
                right: Box::new(Expression {
                    kind: ExpressionKind::IntLiteral(2),
                    span: upper_span,
                }),
            },
            span: lower_span.merge(upper_span),
        }
    );
}

#[test]
fn test_parse_extend_with_type() {
    let base_span = span(0, 1, 1, 5, 1, 6);
    let new_span = span(6, 1, 7, 9, 1, 10);
    let constraint_span = span(10, 1, 11, 21, 1, 22);
    let value_span = span(22, 1, 23, 23, 1, 24);

    // type Testa = int;
    // type New = extend Testa with [multiple_of=2];
    let mut parser = parser_from_tokens(
        vec![
            // type Testa = int;
            Ok(token(TokenKind::Type)),
            Ok(identifier(base_span)),
            Ok(token(TokenKind::SingleEqual)),
            Ok(token(TokenKind::Int)),
            Ok(token(TokenKind::Semicolon)),
            // type New = extend Testa with [multiple_of=2];
            Ok(token(TokenKind::Type)),
            Ok(identifier(new_span)),
            Ok(token(TokenKind::SingleEqual)),
            Ok(token(TokenKind::Extend)),
            Ok(identifier(base_span)),
            Ok(token(TokenKind::With)),
            Ok(token(TokenKind::LBracket)),
            Ok(identifier(constraint_span)),
            Ok(token(TokenKind::SingleEqual)),
            Ok(int_literal(value_span)),
            Ok(token(TokenKind::RBracket)),
            Ok(token(TokenKind::Semicolon)),
        ],
        "Testa New multiple_of 2",
    );

    let program = parser.parse().expect("parse failed");

    assert_eq!(program.0.len(), 2);

    let Statement::TypeDecl {
        name, data_type, ..
    } = &program.0[1]
    else {
        panic!("expected second statement to be type declaration");
    };

    assert_eq!(name, "New");

    let ExpressionKind::Type(data_type) = &data_type.kind else {
        panic!("expected type expression");
    };

    let constraints = data_type.constraints.as_ref().expect("missing constraints");
    assert_eq!(constraints.len(), 1);

    let constraint = &constraints[0];
    assert!(matches!(constraint.kind, ConstraintKind::MultipleOf));

    assert_eq!(
        constraint.expression,
        Expression {
            kind: ExpressionKind::IntLiteral(2),
            span: value_span,
        }
    );
}
