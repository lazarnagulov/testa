use crate::{
    lexer::{
        error::LexerError,
        token::{Token, TokenKind},
    },
    parser::Parser,
};

impl<'src, I> Parser<'src, I>
where
    I: Iterator<Item = Result<Token, LexerError>>,
{
    pub(crate) fn synchronize(&mut self) {
        self.attributes.clear();
        let _ = self.token_stream.consume_token();

        while self.token_stream.has_next() {
            let current_kind = self.token_stream.peek_kind();
            if matches!(current_kind, TokenKind::Semicolon) {
                let _ = self.token_stream.consume_token();
                return;
            }
            if matches!(current_kind, TokenKind::RBrace) {
                return;
            }
            if Self::is_statement_start(current_kind) {
                return;
            }

            if matches!(current_kind, TokenKind::Eof) {
                return;
            }

            let _ = self.token_stream.consume_token();
        }
    }

    #[inline]
    fn is_statement_start(kind: &TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::Output
                | TokenKind::Seed
                | TokenKind::OutputPath
                | TokenKind::Template
                | TokenKind::Resource
                | TokenKind::Type
                | TokenKind::Enum
                | TokenKind::Constraint
                | TokenKind::Override
                | TokenKind::Extend
                | TokenKind::Generate
                | TokenKind::Tag
        )
    }

    pub(crate) fn _synchronize_to_next_field(&mut self) {
        self.attributes.clear();

        while self.token_stream.has_next() {
            match self.token_stream.peek_kind() {
                TokenKind::Comma => {
                    let _ = self.token_stream.consume_token();
                    return;
                }
                TokenKind::RBrace | TokenKind::LBrace => return,
                kind if Self::is_statement_start(kind) => return,
                TokenKind::Eof => return,
                _ => {
                    let _ = self.token_stream.consume_token();
                }
            }
        }
    }

    pub(crate) fn _synchronize_expression(&mut self) {
        let mut depth = 0;

        while self.token_stream.has_next() {
            match self.token_stream.peek_kind() {
                TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => {
                    depth += 1;
                    let _ = self.token_stream.consume_token();
                }
                TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                    if depth == 0 {
                        return;
                    }
                    depth -= 1;
                    let _ = self.token_stream.consume_token();
                }
                TokenKind::Semicolon | TokenKind::Comma if depth == 0 => return,
                kind if depth == 0 && Self::is_statement_start(kind) => return,
                TokenKind::Eof => return,
                _ => {
                    let _ = self.token_stream.consume_token();
                }
            }
        }
    }
}
