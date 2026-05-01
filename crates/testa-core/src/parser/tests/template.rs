use crate::{ast::{DataTypeKind, ExpressionKind, Statement}, lexer::token::TokenKind, parser::tests::{identifier, int_literal, parser_from_tokens, span, string_literal, token}};

#[test]
fn test_parse_template() {
    let name_span = span(0, 1, 1, 5, 1, 6);
    let field_span = span(6, 1, 7, 10, 1, 11);
    // template Testa { test = int; }
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Template)),
            Ok(identifier(name_span)),
            Ok(token(TokenKind::LBrace)),
            Ok(identifier(field_span)),
            Ok(token(TokenKind::SingleEqual)),
            Ok(token(TokenKind::Int)),
            Ok(token(TokenKind::Semicolon)),
            Ok(token(TokenKind::RBrace)),
        ],
        "Testa test ",
    );
    let program = parser.parse().expect("parse failed");
    assert_eq!(program.0.len(), 1);
    let Statement::Template {
        name,
        body,
        name_span: template_name_span,
        ..
    } = &program.0[0]
    else {
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
    let field_span = span(0, 1, 1, 6, 1, 7);
    // template Testa { <> = "test"; test = string; }
    let mut parser = parser_from_tokens(
        vec![
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
            Ok(token(TokenKind::RBrace)),
        ],
        "\"test\"",
    );
    let program = parser.parse().expect("parse failed");
    assert_eq!(program.0.len(), 1);
    let Statement::Template {
        body,
        name_span: template_name_span,
        ..
    } = &program.0[0]
    else {
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
    let parent_span = span(0, 1, 1, 6, 1, 7);
    let parent_field_span = span(7, 1, 8, 10, 1, 11);
    let int_literal_span = span(11, 1, 12, 13, 1, 14);
    let child_span = span(14, 1, 15, 19, 1, 20);
    // template Parent { age = 18 + int; };
    // template Child : Parent { override age = int; }
    let mut parser = parser_from_tokens(
        vec![
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
        ],
        "Parent age 18 Child",
    );

    let program = parser.parse().expect("parse failed");
    assert_eq!(program.0.len(), 2);
    let Statement::Template {
        body, parent_name, ..
    } = &program.0[1]
    else {
        panic!("expected first statement to be template");
    };
    assert_eq!(parent_name, &Some("Parent".to_string()));
    assert_eq!(body.len(), 1);
    assert!(body[0].overridable);
}


#[test]
fn test_parse_struct() {
    let name_span = span(0, 1, 1, 5, 1, 6);
    let field_span = span(6, 1, 7, 10, 1, 11);
    // struct Testa { test = int; }
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Struct)),
            Ok(identifier(name_span)),
            Ok(token(TokenKind::LBrace)),
            Ok(identifier(field_span)),
            Ok(token(TokenKind::SingleEqual)),
            Ok(token(TokenKind::Int)),
            Ok(token(TokenKind::Semicolon)),
            Ok(token(TokenKind::RBrace)),
        ],
        "Testa test ",
    );
    let program = parser.parse().expect("parse failed");
    assert_eq!(program.0.len(), 1);
    let Statement::Struct {
        name,
        body,
        name_span: struct_name_span,
        ..
    } = &program.0[0]
    else {
        panic!("expected first statement to be template");
    };

    assert_eq!(name, "Testa");
    assert_eq!(*struct_name_span, name_span);
    assert_eq!(body.len(), 1);
    let ExpressionKind::Type(field_expr_type) = &body[0].value.kind else {
        panic!("expected expression to be type");
    };
    assert!(matches!(field_expr_type.kind, DataTypeKind::Int));
    assert_eq!(&body[0].name, "test");
}
