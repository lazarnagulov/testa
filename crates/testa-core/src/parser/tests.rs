use crate::{lexer::{error::LexerError, token::{Token, TokenKind}}, parser::{Parser, token_stream::TokenStream}, utils::Span};


fn _token(kind: TokenKind) -> Token {
    Token {
        kind,
        span: Span::default(),
    }
}

fn _parser_from_tokens(
    tokens: Vec<Result<Token, LexerError>>
) -> Parser<'static, std::vec::IntoIter<Result<Token, LexerError>>> {
    Parser::from_token_stream(
        TokenStream::from_iterator(tokens.into_iter()),
        "",
    )
}