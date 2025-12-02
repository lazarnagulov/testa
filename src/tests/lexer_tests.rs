use crate::core::lexer::{
    Lexer,
    token::TokenKind::{self, *},
};

#[test]
fn lex_single_char_tokens() {
    let program = "(){}:[],.;=!+-/*&|^<>~%";
    let mut lexer = Lexer::new(program);
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
    let mut lexer = Lexer::new(program);
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
    let mut lexer = Lexer::new(program);
    
    expect_token(&mut lexer, vec![DoublePeriodEqual]);
}

#[test]
fn lex_range() {
    let program = "10..=20";
    let mut lexer = Lexer::new(program);
    expect_token(&mut lexer, vec![IntLiteral, DoublePeriodEqual, IntLiteral]);
}

#[test]
fn lex_string_tokens() {
    let program = "@generate @output $uuid john \"Peter\" 123 true false int float string 123.123 type constraint override #[readonly] numeric123";
    let mut lexer = Lexer::new(program);
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
            Identifier,
        ],
    );
}

#[test]
fn lex_literal_size() {
    let program = "john \"Peter\" 123";
    let mut lexer = Lexer::new(program);
    expect_token_size(&mut lexer, program, vec!["john", "\"Peter\"", "123"]);
}

#[test]
fn lex_span_locations() {
    let program = "abc def";
    let lexer = Lexer::new(program);

    let tokens: Vec<_> = lexer.map(|r| r.unwrap()).collect();

    assert_eq!(tokens[0].span.start.offset, 0);
    assert_eq!(tokens[0].span.end.offset, 3);
    assert_eq!(tokens[0].span.start.line, 1);
    assert_eq!(tokens[0].span.start.column, 1);
    assert_eq!(tokens[0].span.end.column, 4);

    assert_eq!(tokens[1].span.start.offset, 4);
    assert_eq!(tokens[1].span.end.offset, 7);
    assert_eq!(tokens[1].span.start.line, 1);
    assert_eq!(tokens[1].span.start.column, 5);
}

#[test]
fn lex_multiline_spans() {
    let program = "abc\ndef\nghi";
    let lexer = Lexer::new(program);

    let tokens: Vec<_> = lexer.map(|r| r.unwrap()).collect();

    assert_eq!(tokens[0].span.start.line, 1);
    assert_eq!(tokens[0].span.start.column, 1);

    assert_eq!(tokens[1].span.start.line, 2);
    assert_eq!(tokens[1].span.start.column, 1);

    assert_eq!(tokens[2].span.start.line, 3);
    assert_eq!(tokens[2].span.start.column, 1);
}

#[test]
fn lex_range_spans() {
    let program = "10..=250";
    let lexer = Lexer::new(program);

    let tokens: Vec<_> = lexer.map(|r| r.unwrap()).collect();

    assert_eq!(tokens[0].span.start.column, 1);
    assert_eq!(tokens[1].span.start.column, 3);
    assert_eq!(tokens[2].span.start.column, 6);
}

#[test]
fn lex_string_span() {
    let program = "x \"hello world\" y";
    let lexer = Lexer::new(program);

    let tokens: Vec<_> = lexer.map(|r| r.unwrap()).collect();

    let string_token = &tokens[1];
    assert_eq!(string_token.kind, StringLiteral);
    assert_eq!(string_token.span.start.offset, 2);
    assert_eq!(string_token.span.end.offset, 15);
    assert_eq!(
        &program[string_token.span.start.offset..string_token.span.end.offset],
        "\"hello world\""
    );
}

#[test]
fn lex_multichar_operator_span() {
    let program = "a == b";
    let lexer = Lexer::new(program);

    let tokens: Vec<_> = lexer.map(|r| r.unwrap()).collect();

    let op_token = &tokens[1];
    assert_eq!(op_token.kind, DoubleEqual);
    assert_eq!(op_token.span.start.offset, 2);
    assert_eq!(op_token.span.end.offset, 4);
    assert_eq!(op_token.span.len(), 2);
}

#[test]
fn lex_number_spans() {
    let program = "123 45.67";
    let lexer = Lexer::new(program);

    let tokens: Vec<_> = lexer.map(|r| r.unwrap()).collect();

    assert_eq!(tokens[0].span.len(), 3);
    assert_eq!(
        &program[tokens[0].span.start.offset..tokens[0].span.end.offset],
        "123"
    );

    assert_eq!(tokens[1].span.len(), 5);
    assert_eq!(
        &program[tokens[1].span.start.offset..tokens[1].span.end.offset],
        "45.67"
    );
}

#[test]
fn lex_comment_skips_correctly() {
    let program = "a // comment\nb";
    let lexer = Lexer::new(program);

    let tokens: Vec<_> = lexer.map(|r| r.unwrap()).collect();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, Identifier);
    assert_eq!(tokens[1].kind, Identifier);

    assert_eq!(tokens[1].span.start.line, 2);
}

#[test]
fn lex_attribute_span() {
    let program = "#[readonly] x";
    let lexer = Lexer::new(program);

    let tokens: Vec<_> = lexer.map(|r| r.unwrap()).collect();

    let attr_token = &tokens[0];
    assert_eq!(attr_token.kind, Tag);
    assert_eq!(
        &program[attr_token.span.start.offset..attr_token.span.end.offset],
        "#[readonly]"
    );
}

fn expect_token(lexer: &mut Lexer, expected: Vec<TokenKind>) {
    let token_kinds = lexer.map(|result| result.unwrap().kind).collect::<Vec<_>>();
    assert_eq!(token_kinds, expected);
}

fn expect_token_size(lexer: &mut Lexer, input: &str, expected: Vec<&str>) {
    let token_strings = lexer
        .map(|result| {
            let token = result.unwrap();
            &input[token.span.start.offset..token.span.end.offset]
        })
        .collect::<Vec<_>>();
    assert_eq!(token_strings, expected);
}
