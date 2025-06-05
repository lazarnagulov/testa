use std::{path::Path, vec};

use crate::core::lexer::{
    Lexer,
    token::TokenKind::{self, *},
};

#[test]
fn lex_single_char_tokens() {
    let program = "(){}:[],.;=!+-/*&|^<>~%";
    let mut lexer = Lexer::new(program, Path::new(""));
    expect_token(
        &mut lexer,
        vec![
            LParen,
            RParen,
            LBrace,
            RBrace,
            Colon,
            LBracket,
            RBracket,
            Comma,
            SinglePeriod,
            Semicolon,
            SingleEqual,
            ExclamationMark,
            Plus,
            Minus,
            Slash,
            Asterisk,
            BitAnd,
            BitOr,
            BitXor,
            LessThan,
            GreaterThan,
            BitNegate,
            Percent,
        ],
    );
}

#[test]
fn lex_two_char_tokens() {
    let program = "==!=<=>=<<>>&&||=>..";
    let mut lexer = Lexer::new(program, Path::new(""));
    expect_token(
        &mut lexer,
        vec![
            DoubleEqual,
            NotEqual,
            LessThanOrEqual,
            GreaterThanOrEqual,
            BitLShift,
            BitRShift,
            And,
            Or,
            Arrow,
            DoublePeriod,
        ],
    );
}

#[test]
fn lex_three_char_tokens() {
    let program = "..=";
    let mut lexer = Lexer::new(program, Path::new(""));
    expect_token(&mut lexer, vec![DoublePeriodEqual]);
}

#[test]
fn lex_range() {
    let program = "10..=20";
    let mut lexer = Lexer::new(program, Path::new(""));
    expect_token(&mut lexer, vec![IntLiteral, DoublePeriodEqual, IntLiteral]);
}

#[test]
fn lex_string_tokens() {
    let program = "@generate @output $uuid john \"Peter\" 123 true false int float string 123.123 type constraint override #[readonly]";
    let mut lexer = Lexer::new(program, Path::new(""));
    expect_token(
        &mut lexer,
        vec![
            Generate,
            Output,
            Uuid,
            Identifier,
            StringLiteral,
            IntLiteral,
            True,
            False,
            Int,
            Float,
            Str,
            FloatLiteral,
            Type,
            Constraint,
            Override,
            Tag,
        ],
    );
}

#[test]
fn lex_literal_size() {
    let program = "john \"Peter\" 123";
    let mut lexer = Lexer::new(program, Path::new(""));
    expect_token_size(&mut lexer, program, vec!["john", "\"Peter\"", "123"]);
}

fn expect_token(lexer: &mut Lexer, expected: Vec<TokenKind>) {
    let token_kinds = lexer
        .into_iter()
        .map(|token| token.kind)
        .collect::<Vec<_>>();
    assert_eq!(token_kinds, expected);
}

fn expect_token_size(lexer: &mut Lexer, input: &str, expected: Vec<&str>) {
    let token_kinds = lexer
        .into_iter()
        .map(|token| &input[token.span.start..token.span.start + token.span.size])
        .collect::<Vec<_>>();
    assert_eq!(token_kinds, expected);
}
