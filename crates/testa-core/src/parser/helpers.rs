use super::Parser;
use crate::{
    lexer::{
        error::LexerError,
        token::{Token, TokenKind},
    },
    parser::error::ParserError,
    utils::Span,
};

impl<'src, I> Parser<'src, I>
where
    I: Iterator<Item = Result<Token, LexerError>>,
{
    pub(super) fn source_text(&self, span: Span) -> &'src str {
        &self.source[span.outer()]
    }

    pub(super) fn token_text(&self, span: Span) -> &'src str {
        &self.source[span.outer()]
    }

    pub(super) fn string_literal_content(&self, span: Span) -> &'src str {
        &self.source[span.inner()]
    }

    pub(super) fn attribute_text(&self, span: Span) -> &'src str {
        &self.source[span.start.offset + 2..span.end.offset - 1]
    }

    pub(super) fn parse_peeked_token_as_string(&mut self) -> Result<String, ParserError> {
        let token = self.token_stream.peek_token()?;
        let span = token.span;
        Ok(self.source_text(span).to_string())
    }

    pub(super) fn parse_identifier_as_string(&mut self) -> Result<String, ParserError> {
        let span = self.token_stream.expect_token(TokenKind::Identifier)?;
        Ok(self.source_text(span).to_string())
    }
}
