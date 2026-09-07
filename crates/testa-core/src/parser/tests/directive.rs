use std::path::PathBuf;

use crate::{
    ast::{Expression, ExpressionKind, Statement},
    lexer::token::TokenKind,
    parser::tests::{identifier, parser_from_tokens, span, string_literal, token},
};

#[test]
fn test_parse_output_directive() {
    let csv_span = span(0, 1, 1, 3, 1, 4);
    let config_span = span(4, 1, 5, 13, 1, 13);
    let config_option_span = span(14, 1, 15, 17, 1, 18);

    // @output csv { delimiter = ";"; }
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Output)),
            Ok(identifier(csv_span)),
            Ok(token(TokenKind::LBrace)),
            Ok(identifier(config_span)),
            Ok(token(TokenKind::SingleEqual)),
            Ok(string_literal(config_option_span)),
            Ok(token(TokenKind::Semicolon)),
            Ok(token(TokenKind::RBrace)),
        ],
        "csv delimiter \";\"",
    );
    let program = parser.parse().expect("parse failed");
    assert_eq!(program.0.len(), 1);
    let Statement::OutputDirective {
        argument, options, ..
    } = &program.0[0]
    else {
        panic!("expected first statement to be output directive");
    };
    assert_eq!(argument, "csv");
    assert_eq!(options.len(), 1);
    assert_eq!(options[0].name, "delimiter");
    assert!(matches!(
        options[0].value,
        Expression {
            kind: ExpressionKind::StringLiteral(ref s),
            span: s2
        } if s == ";" && s2 == config_option_span
    ));
}

#[test]
fn test_parse_import_directive() {
    let import_span = span(0, 1, 1, 3, 1, 4);
    // @import std;
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::Import)),
            Ok(identifier(import_span)),
            Ok(token(TokenKind::Semicolon)),
        ],
        "std",
    );
    let program = parser.parse().expect("parse failed");
    assert_eq!(program.0.len(), 1);
    let Statement::ImportDirective { argument, .. } = &program.0[0] else {
        panic!("expected first statement to be import directive");
    };
    assert_eq!(argument, &String::from("std"));
}

#[test]
fn test_parse_output_path_directive() {
    let output_path_span = span(0, 1, 1, 10, 1, 11);
    // @output_path "test.csv";
    let mut parser = parser_from_tokens(
        vec![
            Ok(token(TokenKind::OutputPath)),
            Ok(string_literal(output_path_span)),
            Ok(token(TokenKind::Semicolon)),
        ],
        "\"test.csv\"",
    );
    let program = parser.parse().expect("parse failed");
    assert_eq!(program.0.len(), 1);
    let Statement::OutputPathDirective { argument, .. } = &program.0[0] else {
        panic!("expected first statement to be output directive");
    };
    assert_eq!(argument, &PathBuf::from("test.csv"));
}
