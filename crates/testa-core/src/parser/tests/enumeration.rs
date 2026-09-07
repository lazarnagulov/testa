use crate::{
    ast::{Expression, ExpressionKind, Statement},
    lexer::token::TokenKind,
    parser::{
        error::ParserError,
        tests::{int_literal, parser_from_tokens, span, token},
    },
    utils::Span,
};

#[test]
fn test_parse_enum() {
    // enum <> { <>; }
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Enum)),
            Ok(token(TokenKind::Identifier)),
            Ok(token(TokenKind::LBrace)),
            Ok(token(TokenKind::Identifier)),
            Ok(token(TokenKind::Semicolon)),
            Ok(token(TokenKind::RBrace)),
        ],
        "",
    );

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
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Enum)),
            Ok(token(TokenKind::Identifier)),
            Ok(token(TokenKind::LBrace)),
            Ok(token(TokenKind::Identifier)),
            Ok(token(TokenKind::Arrow)),
            Ok(int_literal(expr_span)),
            Ok(token(TokenKind::Semicolon)),
            Ok(token(TokenKind::RBrace)),
        ],
        "1",
    );

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
    let mut parser = parser_from_tokens(
        vec![
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
        ],
        "1",
    );

    let program = parser.parse().expect("parse failed");
    let Statement::Enum { variants, .. } = &program.0[0] else {
        panic!("expected enum");
    };

    assert_eq!(variants.len(), 2);
    assert!(variants[0].weight.is_some());
    assert!(variants[1].weight.is_none());
}

#[test]
#[ignore = "parser synchronization is not implemented"]
fn test_parse_enum_missing_identifier() {
    // enum { ... }
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Enum)),
            Ok(token(TokenKind::LBrace)),
            Ok(token(TokenKind::Identifier)),
            Ok(token(TokenKind::Semicolon)),
            Ok(token(TokenKind::RBrace)),
        ],
        "",
    );

    match parser.parse() {
        Ok(program) => panic!("expected error, got {:?}", program),

        Err(errors) => {
            assert_eq!(errors.len(), 1);
            assert!(matches!(
                &errors[0],
                ParserError::Expected {
                    span,
                    expected,
                    got,
                }
                if span == &Span::default()
                    && expected.as_str() == "identifier"
                    && got.as_str() == "{"
            ));
        }
    }
}
