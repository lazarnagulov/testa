use core::panic;

use crate::{
    ast::{ConstraintKind, DataTypeKind, Expression, ExpressionKind, InfixOperator, Statement},
    lexer::token::TokenKind,
    parser::error::ParserError,
    utils::test_utils::*,
};

#[test]
fn test_parse_enum() {
    // enum <> { <>; }
    let mut parser = parser_from_tokens(vec![
        Ok(token(TokenKind::Enum)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::LBrace)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::Semicolon)),
        Ok(token(TokenKind::RBrace)),
    ], "");

    let program = parser.parse().expect("parse failed");
    let Statement::Enum { variants, .. } = &program.0[0] else {
        panic!("expected enum");
    };

    assert_eq!(variants.len(), 1);
    assert!(variants[0].weight.is_none());
}

#[test]
fn test_parse_weighted_enum() {
    let expr_span = span(0, 1, 1, 1, 1, 2);

    // enum <> { <> = 1; }
    let mut parser = parser_from_tokens(vec![
        Ok(token(TokenKind::Enum)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::LBrace)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::Arrow)),
        Ok(int_literal(expr_span)),
        Ok(token(TokenKind::Semicolon)),
        Ok(token(TokenKind::RBrace)),
    ], "1");

    let program = parser.parse().expect("parse failed");
    let Statement::Enum { variants, .. } = &program.0[0] else {
        panic!("expected enum");
    };

    assert!(matches!(
        variants[0].weight,
        Some(Expression {
            kind: ExpressionKind::IntLiteral(1),
            ..
        })
    ));
}

#[test]
fn test_parse_mixed_enum() {
    let expr_span = span(0, 1, 1, 1, 1, 2);

    // enum <> { <> = 1; <>; }
    let mut parser = parser_from_tokens(vec![
        Ok(token(TokenKind::Enum)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::LBrace)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::Arrow)),
        Ok(int_literal(expr_span)),
        Ok(token(TokenKind::Semicolon)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::Semicolon)),
        Ok(token(TokenKind::RBrace)),
    ], "1");

    let program = parser.parse().expect("parse failed");
    let Statement::Enum { variants, .. } = &program.0[0] else {
        panic!("expected enum");
    };

    assert_eq!(variants.len(), 2);
    assert!(variants[0].weight.is_some());
    assert!(variants[1].weight.is_none());
}

#[test]
fn test_parse_enum_missing_identifier() {
    // enum { ... }
    let mut parser = parser_from_tokens(vec![
        Ok(token(TokenKind::Enum)),
        Ok(token(TokenKind::LBrace)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::Semicolon)),
        Ok(token(TokenKind::RBrace)),
    ], "");

    match parser.parse() {
        Ok(program) => panic!("expected error, got {:?}", program),
        Err(ParserError::Expected { expected, got, .. }) => {
            assert_eq!(expected, "identifier");
            assert_eq!(got, "{");
        }
        Err(err) => panic!("unexpected error: {:?}", err),
    }
}

#[test]
fn test_parse_type_declaration() {
    let name_span = span(0, 1, 1, 5, 1, 6);

    // type Testa = int;
    let mut parser = parser_from_tokens(vec![
        Ok(token(TokenKind::Type)),
        Ok(identifier(name_span)),
        Ok(token(TokenKind::SingleEqual)),
        Ok(token(TokenKind::Int)),
        Ok(token(TokenKind::Semicolon)),
    ], "Testa");

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
    let mut parser = parser_from_tokens(vec![
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
    ], "range 1 2");

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
    let mut parser = parser_from_tokens(vec![
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
    ], "Testa New multiple_of 2");

    let program = parser.parse().expect("parse failed");

    assert_eq!(program.0.len(), 2);

    let Statement::TypeDecl { name, data_type, .. } = &program.0[1] else {
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

#[test]
fn test_parse_template() {
    let name_span = span(0,1,1,5,1,6);
    let field_span = span(6,1,7, 10, 1, 11);
    // template Testa { test = int; }
    let mut parser = parser_from_tokens(vec![
        Ok(token(TokenKind::Template)),
        Ok(identifier(name_span)),
        Ok(token(TokenKind::LBrace)),
        Ok(identifier(field_span)),
        Ok(token(TokenKind::SingleEqual)),
        Ok(token(TokenKind::Int)),
        Ok(token(TokenKind::Semicolon)),
        Ok(token(TokenKind::RBrace))
    ], "Testa test ");
    let program = parser.parse().expect("parse failed");
    assert_eq!(program.0.len(), 1);
    let Statement::Template { name, body, name_span: template_name_span, ..} = &program.0[0] else {
        panic!("expected first statement to be template");
    };

    assert_eq!(name, "Testa");
    assert!(template_name_span.is_some());
    assert_eq!(template_name_span, &Some(name_span));
    assert_eq!(body.len(), 1);
    let ExpressionKind::Type(field_expr_type) = &body[0].value.kind else {
      panic!("expected expression to be type");  
    };
    assert!(matches!(field_expr_type.kind, DataTypeKind::Int));
    assert_eq!(&body[0].name, "test");
}

#[test]
fn test_parse_multi_field_template() {
    let field_span = span(0,1,1, 6, 1, 7);
    // template Testa { <> = "test"; test = string; }
    let mut parser = parser_from_tokens(vec![
        Ok(token(TokenKind::Template)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::LBrace)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::SingleEqual)),
        Ok(string_literal(field_span)),
        Ok(token(TokenKind::Semicolon)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::SingleEqual)),
        Ok(token(TokenKind::Str)),
        Ok(token(TokenKind::Semicolon)),
        Ok(token(TokenKind::RBrace))
    ], "\"test\"");
    let program = parser.parse().expect("parse failed");
    assert_eq!(program.0.len(), 1);
    let Statement::Template { body, name_span: template_name_span, ..} = &program.0[0] else {
        panic!("expected first statement to be template");
    };

    assert!(template_name_span.is_some());
    assert_eq!(body.len(), 2);
    let ExpressionKind::StringLiteral(field_expr_type) = &body[0].value.kind else {
      panic!("expected expression to be string_literal");  
    };
    assert_eq!(field_expr_type, "test");

     let ExpressionKind::Type(field_expr_type) = &body[1].value.kind else {
      panic!("expected expression to be type");  
    };
    assert!(matches!(field_expr_type.kind, DataTypeKind::Str));
}

#[test]
fn test_parse_template_inheritance() {
    let parent_span = span(0,1,1, 6, 1, 7);
    let parent_field_span = span(7,1,8, 10, 1, 11);
    let int_literal_span = span(11,1,12, 13, 1, 14);
    let child_span = span(14, 1, 15, 19, 1, 20);
    // template Parent { age = 18 + int; };
    // template Child : Parent { override age = int; }
    let mut parser = parser_from_tokens(vec![
        // template Parent { age = 18 + int; };
        Ok(token(TokenKind::Template)),
        Ok(identifier(parent_span)),
        Ok(token(TokenKind::LBrace)),
        Ok(identifier(parent_field_span)),
        Ok(token(TokenKind::SingleEqual)),
        Ok(int_literal(int_literal_span)),
        Ok(token(TokenKind::Plus)),
        Ok(token(TokenKind::Int)),
        Ok(token(TokenKind::Semicolon)),
        Ok(token(TokenKind::RBrace)),

        // template Child : Parent { override age = int; }
        Ok(token(TokenKind::Template)),
        Ok(identifier(child_span)),
        Ok(token(TokenKind::Colon)),
        Ok(identifier(parent_span)),
        Ok(token(TokenKind::LBrace)),
        Ok(token(TokenKind::Override)),
        Ok(identifier(parent_field_span)),
        Ok(token(TokenKind::SingleEqual)),
        Ok(token(TokenKind::Int)),
        Ok(token(TokenKind::Semicolon)),
        Ok(token(TokenKind::RBrace)),
    ], "Parent age 18 Child");

    let program = parser.parse().expect("parse failed");
    assert_eq!(program.0.len(), 2);
    let Statement::Template { body, parent_name, ..} = &program.0[1] else {
        panic!("expected first statement to be template");
    };
    assert_eq!(parent_name, &Some("Parent".to_string()));
    assert_eq!(body.len(), 1);
    assert!(body[0].overridable);
}