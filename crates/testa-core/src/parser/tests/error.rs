use crate::{lexer::token::TokenKind, parser::{error::ParserError, tests::{parser_from_tokens, token}}};

#[test]
fn error_expected_identifier_after_type() {
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Type)),
            Ok(token(TokenKind::SingleEqual)),
        ],
        "",
    );

    let err = parser.parse().unwrap_err();

    assert!(matches!(
        err[0],
        ParserError::Expected { ref expected, ref got, .. }
        if expected == "identifier" && got == "="
    ));
}

#[test]
fn error_expected_equal_in_field() {
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Template)),
            Ok(token(TokenKind::Identifier)),
            Ok(token(TokenKind::LBrace)),
            Ok(token(TokenKind::Identifier)),
            Ok(token(TokenKind::Int)),
        ],
        "",
    );

    let err = parser.parse().unwrap_err();

    assert!(matches!(err[0], ParserError::Expected { .. }));
}

#[test]
fn error_unexpected_token_in_enum() {
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Enum)),
            Ok(token(TokenKind::Identifier)),
            Ok(token(TokenKind::LBrace)),
            Ok(token(TokenKind::Arrow)), // invalid start of variant
        ],
        "",
    );

    let err = parser.parse().unwrap_err();

    assert!(matches!(err[0], ParserError::Expected { .. }));
}

#[test]
fn error_unexpected_eof_in_template() {
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Template)),
            Ok(token(TokenKind::Identifier)),
            Ok(token(TokenKind::LBrace)),
        ],
        "",
    );

    let err = parser.parse().unwrap_err();

    assert!(matches!(err[0], ParserError::UnexpectedEof { .. }));
}

#[test]
#[ignore = "parser attributes will behavior will be changed"]
fn error_invalid_attribute_position() {
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Tag)), // attribute without context
            Ok(token(TokenKind::Semicolon)),
        ],
        "",
    );

    let err = parser.parse().unwrap_err();

    assert!(matches!(
        err[0],
        ParserError::InvalidAttribute { .. }
    ));
}

#[test]
fn error_generic_syntax() {
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Semicolon)), // meaningless start
        ],
        "",
    );

    let err = parser.parse().unwrap_err();

    assert!(matches!(
        err[0],
        ParserError::Syntax { .. }
    ));
}

#[test]
fn error_multiple_errors() {
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Template)),
            Ok(token(TokenKind::LBrace)), // missing identifier
            Ok(token(TokenKind::Semicolon)), // nonsense
        ],
        "",
    );

    let err = parser.parse().unwrap_err();

    assert!(!err.is_empty());
}