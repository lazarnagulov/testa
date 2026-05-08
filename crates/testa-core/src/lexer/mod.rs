pub mod error;
pub mod token;

#[cfg(test)]
mod tests;

use std::{iter::Peekable, str::CharIndices};

use crate::{
    lexer::{
        error::LexerError,
        token::{KEYWORD_REGISTRY, Token, TokenKind},
    },
    utils::{Location, Span},
};

#[derive(Clone, Debug)]
pub struct Lexer<'src> {
    content: &'src str,
    chars: Peekable<CharIndices<'src>>,

    pub line: u32,
    pub column: u32,
    pub current_offset: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(program: &'src str) -> Self {
        Lexer {
            content: program,
            chars: program.char_indices().peekable(),
            line: 1,
            column: 1,
            current_offset: 0,
        }
    }

    fn current_location(&self) -> Location {
        Location::new(self.current_offset, self.line, self.column)
    }

    fn peek(&mut self) -> Option<(usize, char)> {
        self.chars.peek().copied()
    }

    fn peek_n(&self, n: usize) -> Option<(usize, char)> {
        self.chars.clone().nth(n - 1)
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        use crate::lexer::token::TokenKind::*;
        let registry = &KEYWORD_REGISTRY;
        self.skip_whitespaces();

        let Some((_, current_char)) = self.peek() else {
            let loc = Location::new(self.content.len(), self.line, self.column);
            return Ok(Token::new(Eof, Span::single_char(loc)));
        };

        let start_location = self.current_location();

        match current_char {
            '(' => Ok(self.make_single_char_token(start_location, LParen)),
            ')' => Ok(self.make_single_char_token(start_location, RParen)),
            '{' => Ok(self.make_single_char_token(start_location, LBrace)),
            '}' => Ok(self.make_single_char_token(start_location, RBrace)),
            ':' => Ok(self.make_single_char_token(start_location, Colon)),
            ',' => Ok(self.make_single_char_token(start_location, Comma)),
            '#' => {
                self.advance();
                if self.consume_if('[') {
                    self.make_attribute(start_location)
                } else {
                    Err(LexerError::InvalidToken {
                        span: Span::single_char(start_location),
                        token: '#',
                    })
                }
            }
            ';' => Ok(self.make_single_char_token(start_location, Semicolon)),
            '%' => Ok(self.make_single_char_token(start_location, Percent)),
            '[' => Ok(self.make_single_char_token(start_location, LBracket)),
            ']' => Ok(self.make_single_char_token(start_location, RBracket)),
            '.' => {
                self.advance();
                if self.consume_if('.') {
                    if self.consume_if('=') {
                        Ok(Token::new(
                            DoublePeriodEqual,
                            Span::from_len(start_location, 3),
                        ))
                    } else {
                        Ok(Token::new(DoublePeriod, Span::from_len(start_location, 2)))
                    }
                } else {
                    Ok(Token::new(SinglePeriod, Span::single_char(start_location)))
                }
            }
            '+' => Ok(self.make_single_char_token(start_location, Plus)),
            '-' => Ok(self.make_single_char_token(start_location, Minus)),
            '*' => Ok(self.make_single_char_token(start_location, Asterisk)),
            '&' => {
                self.advance();
                if self.consume_if('&') {
                    Ok(Token::new(And, Span::from_len(start_location, 2)))
                } else {
                    Ok(Token::new(BitAnd, Span::single_char(start_location)))
                }
            }
            '~' => Ok(self.make_single_char_token(start_location, BitNegate)),
            '|' => {
                self.advance();
                if self.consume_if('|') {
                    Ok(Token::new(Or, Span::from_len(start_location, 2)))
                } else {
                    Ok(Token::new(BitOr, Span::single_char(start_location)))
                }
            }
            '^' => Ok(self.make_single_char_token(start_location, BitXor)),
            '=' => {
                self.advance();
                if self.consume_if('=') {
                    Ok(Token::new(DoubleEqual, Span::from_len(start_location, 2)))
                } else if self.consume_if('>') {
                    Ok(Token::new(Arrow, Span::from_len(start_location, 2)))
                } else {
                    Ok(Token::new(SingleEqual, Span::single_char(start_location)))
                }
            }
            '!' => {
                self.advance();
                if self.consume_if('=') {
                    Ok(Token::new(NotEqual, Span::from_len(start_location, 2)))
                } else {
                    Ok(Token::new(
                        ExclamationMark,
                        Span::single_char(start_location),
                    ))
                }
            }
            '"' => {
                self.advance();
                let end_location = self.read_string(start_location)?;
                Ok(Token::new(
                    StringLiteral,
                    Span::new(start_location, end_location),
                ))
            }
            '<' => {
                self.advance();
                if self.consume_if('=') {
                    Ok(Token::new(
                        LessThanOrEqual,
                        Span::from_len(start_location, 2),
                    ))
                } else if self.consume_if('<') {
                    Ok(Token::new(BitLShift, Span::from_len(start_location, 2)))
                } else {
                    Ok(Token::new(LessThan, Span::single_char(start_location)))
                }
            }
            '>' => {
                self.advance();
                if self.consume_if('=') {
                    Ok(Token::new(
                        GreaterThanOrEqual,
                        Span::from_len(start_location, 2),
                    ))
                } else if self.consume_if('>') {
                    Ok(Token::new(BitRShift, Span::from_len(start_location, 2)))
                } else {
                    Ok(Token::new(GreaterThan, Span::single_char(start_location)))
                }
            }
            '/' => {
                self.advance();
                if self.consume_if('/') {
                    self.skip_line();
                    self.next_token()
                } else {
                    Ok(Token::new(Slash, Span::single_char(start_location)))
                }
            }
            '$' => {
                self.advance();
                let (builtin, end_location) = self.read_identifier();
                match registry.get(builtin) {
                    Some(kind) => Ok(Token::new(
                        kind.clone(),
                        Span::new(start_location, end_location),
                    )),
                    None => Err(LexerError::InvalidBuiltIn {
                        span: Span::new(start_location, end_location),
                        value: builtin.to_string(),
                    }),
                }
            }
            '@' => {
                self.advance();
                let (directive, end_location) = self.read_identifier();
                match registry.get(directive) {
                    Some(kind) => Ok(Token::new(
                        kind.clone(),
                        Span::new(start_location, end_location),
                    )),
                    None => Err(LexerError::InvalidDirective {
                        span: Span::new(start_location, end_location),
                        value: directive.to_string(),
                    }),
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let (identifier, end_location) = self.read_identifier();
                match registry.get(identifier) {
                    Some(kind) => Ok(Token::new(
                        kind.clone(),
                        Span::new(start_location, end_location),
                    )),
                    None => Ok(Token::new(
                        Identifier,
                        Span::new(start_location, end_location),
                    )),
                }
            }
            '0'..='9' => self.make_number_token(start_location),
            c => Err(LexerError::InvalidToken {
                span: Span::single_char(start_location),
                token: c,
            }),
        }
    }

    fn skip_line(&mut self) {
        while let Some((_, c)) = self.peek() {
            if c == '\n' {
                return;
            }
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> (&'src str, Location) {
        let start = self.current_offset;
        let mut end_location = self.current_location();

        while self
            .peek()
            .is_some_and(|(_, c)| c.is_ascii_alphanumeric() || c == '_')
        {
            self.advance();
            end_location = self.current_location();
        }

        (&self.content[start..self.current_offset], end_location)
    }

    fn make_number_token(&mut self, start_location: Location) -> Result<Token, LexerError> {
        let mut is_float = false;
        let mut end_location = start_location;

        while self
            .peek()
            .is_some_and(|(_, c)| c.is_ascii_digit() || c == '.')
        {
            if self.peek().unwrap().1 == '.' && self.peek_n(2).is_some_and(|(_, c)| c == '.') {
                return Ok(Token::new(
                    if is_float {
                        TokenKind::FloatLiteral
                    } else {
                        TokenKind::IntLiteral
                    },
                    Span::new(start_location, end_location),
                ));
            }

            let (_, ch) = self
                .advance()
                .expect("Next should exist, it is checked in while.");

            if is_float && ch == '.' {
                return Err(LexerError::InvalidNumberLiteral {
                    span: Span::new(start_location, self.current_location()),
                    value: "#".to_string(),
                });
            } else if ch == '.' {
                is_float = true;
            }

            end_location = self.current_location();
        }

        if let Some((_, c)) = self.peek()
            && !matches!(c, ' ' | ';' | ',' | ']' | ')') {
                return Err(LexerError::InvalidNumberLiteral {
                    span: Span::new(start_location, end_location),
                    value: c.to_string(),
                });
            }
        Ok(Token::new(
            if is_float {
                TokenKind::FloatLiteral
            } else {
                TokenKind::IntLiteral
            },
            Span::new(start_location, end_location),
        ))
    }

    fn read_string(&mut self, start_location: Location) -> Result<Location, LexerError> {
        let mut end_location = self.current_location();

        while self.peek().is_some_and(|(_, c)| c != '"') {
            let (_, ch) = self
                .advance()
                .expect("Next should exist, it is checked in while.");

            if ch == '\n' {
                return Err(LexerError::MissingChar {
                    span: Span::new(start_location, self.current_location()),
                    expected: '"',
                });
            }

            end_location = self.current_location();
        }

        self.advance().ok_or_else(|| LexerError::MissingChar {
            span: Span::new(start_location, end_location),
            expected: '"',
        })?;

        Ok(self.current_location())
    }

    fn make_attribute(&mut self, start_location: Location) -> Result<Token, LexerError> {
        let mut end_location = self.current_location();

        while self.peek().is_some_and(|(_, c)| c != ']') {
            self.advance();
            end_location = self.current_location();
        }

        self.advance().ok_or_else(|| LexerError::MissingChar {
            span: Span::new(start_location, end_location),
            expected: ']',
        })?;

        Ok(Token::new(
            TokenKind::Tag,
            Span::new(start_location, self.current_location()),
        ))
    }

    fn consume_if(&mut self, expected: char) -> bool {
        if self.peek().is_some_and(|(_, c)| c == expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn make_single_char_token(&mut self, location: Location, kind: TokenKind) -> Token {
        let token = Token::new(kind, Span::single_char(location));
        self.advance();
        token
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        if let Some((idx, ch)) = self.chars.next() {
            self.current_offset = idx + ch.len_utf8();

            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }

            Some((idx, ch))
        } else {
            None
        }
    }

    fn skip_whitespaces(&mut self) {
        while self.peek().is_some_and(|(_, c)| c.is_whitespace()) {
            self.advance();
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(token) if token.kind == TokenKind::Eof => None,
            Ok(token) => Some(Ok(token)),
            Err(error) => Some(Err(error)),
        }
    }
}
