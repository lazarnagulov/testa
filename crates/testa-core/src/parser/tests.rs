use crate::{
    ast::{Expression, ExpressionKind, Statement},
    lexer::{
        error::LexerError,
        token::{Token, TokenKind},
    },
    parser::Parser,
    utils::{Location, Span},
};

fn token(kind: TokenKind) -> Token {
    Token {
        kind,
        span: Span::default(),
    }
}

fn token_with_span(mut token: Token, span: Span) -> Token {
    token.span = span;
    token
}

fn parser_from_tokens(
    tokens: Vec<Result<Token, LexerError>>,
    source: &'static str
) -> Parser<'static, std::vec::IntoIter<Result<Token, LexerError>>> {
    Parser::new(tokens.into_iter(), source)
}

#[test]
fn test_parse_enum() {
    let mut parser = parser_from_tokens(vec![
        Ok(token(TokenKind::Enum)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::LBrace)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::Semicolon)),
        Ok(token(TokenKind::RBrace)),
    ], "");
    match parser.parse() {
        Ok(program) => {
            let enum_stmt = program.0.first().expect("Should have one statement");
            let Statement::Enum { variants, .. } = enum_stmt else {
                panic!("Should have enum statement");
            };

            assert_eq!(variants.len(), 1);
            assert!(variants[0].weight.is_none());
            assert_eq!(program.0.len(), 1);
        }
        Err(error) => panic!("{}", error),
    };
}

#[test]
fn test_parse_weighted_enum() {
    let expression_span = Span::new(Location::new(0, 1, 1), Location::new(1, 1, 2));
    let expression_token = token_with_span(
        token(TokenKind::IntLiteral),
        expression_span,
    );
    let mut parser = parser_from_tokens(vec![
        Ok(token(TokenKind::Enum)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::LBrace)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::Arrow)),
        Ok(expression_token),
        Ok(token(TokenKind::Semicolon)),
        Ok(token(TokenKind::RBrace)),
    ], "1");
    match parser.parse() {
        Ok(program) => {
            let enum_stmt = program.0.first().expect("Should have one statement");
            let Statement::Enum { variants, .. } = enum_stmt else {
                panic!("Should have enum statement");
            };

            assert_eq!(variants.len(), 1);
            assert!(variants[0].weight.is_some());
            assert!(matches!(variants[0].weight, Some(Expression {
                kind: ExpressionKind::IntLiteral(1),
                ..
            })));
            assert_eq!(program.0.len(), 1);
        }
        Err(error) => panic!("{}", error),
    };
}

#[test]
fn test_parse_mixed_enum() {
        let expression_span = Span::new(Location::new(0, 1, 1), Location::new(1, 1, 2));
    let expression_token = token_with_span(
        token(TokenKind::IntLiteral),
        expression_span,
    );
    let mut parser = parser_from_tokens(vec![
        Ok(token(TokenKind::Enum)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::LBrace)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::Arrow)),
        Ok(expression_token),
        Ok(token(TokenKind::Semicolon)),
        Ok(token(TokenKind::Identifier)),
        Ok(token(TokenKind::Semicolon)),
        Ok(token(TokenKind::RBrace)),
    ], "1");
    match parser.parse() {
        Ok(program) => {
            let enum_stmt = program.0.first().expect("Should have one statement");
            let Statement::Enum { variants, .. } = enum_stmt else {
                panic!("Should have enum statement");
            };

            assert_eq!(variants.len(), 2);
            assert!(variants[0].weight.is_some());
            assert!(matches!(variants[0].weight, Some(Expression {
                kind: ExpressionKind::IntLiteral(1),
                ..
            })));
            assert!(variants[1].weight.is_none());
            assert_eq!(program.0.len(), 1);
        }
        Err(error) => panic!("{}", error),
    }
}
