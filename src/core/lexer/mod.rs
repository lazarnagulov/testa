pub mod lexer_error;
pub mod token;

use std::{iter::Peekable, str::CharIndices};

use crate::core::{
    lexer::{
        lexer_error::LexerError,
        token::{KEYWORD_REGISTRY, Token, TokenKind},
    },
    utils::span::Span,
};

#[derive(Clone, Debug)]
pub struct Lexer<'src> {
    content: &'src str,
    chars: Peekable<CharIndices<'src>>,

    pub line: usize,
    pub line_offset: usize,
    pub current_position: usize,
}

impl<'src> Lexer<'src> {

    pub fn new(program: &'src str) -> Self {
        Lexer {
            content: program,
            chars: program.char_indices().peekable(),
            line: 1,
            line_offset: 1,
            current_position: 0,
        }
    }

    fn peek(&mut self) -> Option<(usize, char)> {
        self.chars.peek().copied()
    }

    fn peek_n(&self, n: usize) -> Option<(usize, char)> {
        self.chars.clone().nth(n - 1)
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        use crate::core::lexer::token::TokenKind::*;
        let registry = &KEYWORD_REGISTRY;
        self.skip_whitespaces();

        let Some((current_index, current_char)) = self.peek() else {
            return Ok(Token::new(
                Eof,
                Span::new(self.content.len(), 1, self.line, self.line_offset),
            ));
        };

        match current_char {
            '(' => Ok(self.make_single_char_token(current_index, LParen)),
            ')' => Ok(self.make_single_char_token(current_index, RParen)),
            '{' => Ok(self.make_single_char_token(current_index, LBrace)),
            '}' => Ok(self.make_single_char_token(current_index, RBrace)),
            ':' => Ok(self.make_single_char_token(current_index, Colon)),
            ',' => Ok(self.make_single_char_token(current_index, Comma)),
            '#' => {
                self.advance();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '[')
                    .is_some()
                {
                    self.make_attribute(current_index)
                } else {
                    Err(LexerError::InvalidToken(
                        Span::new(current_index, 1, self.line, self.line_offset),
                        '#',
                    ))
                }
            }
            ';' => Ok(self.make_single_char_token(current_index, Semicolon)),
            '%' => Ok(self.make_single_char_token(current_index, Percent)),
            '[' => Ok(self.make_single_char_token(current_index, LBracket)),
            ']' => Ok(self.make_single_char_token(current_index, RBracket)),
            '.' => {
                self.advance();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '.')
                    .is_some()
                {
                    if self
                        .chars
                        .next_if(|(_, next_char)| *next_char == '=')
                        .is_some()
                    {
                        Ok(Token::new(
                            DoublePeriodEqual,
                            Span::new(current_index, 3, self.line, self.line_offset),
                        ))
                    } else {
                        Ok(Token::new(
                            DoublePeriod,
                            Span::new(current_index, 2, self.line, self.line_offset),
                        ))
                    }
                } else {
                    Ok(Token::new(
                        SinglePeriod,
                        Span::new(current_index, 1, self.line, self.line_offset),
                    ))
                }
            }
            '+' => Ok(self.make_single_char_token(current_index, Plus)),
            '-' => Ok(self.make_single_char_token(current_index, Minus)),
            '*' => Ok(self.make_single_char_token(current_index, Asterisk)),
            '&' => {
                self.advance();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '&')
                    .is_some()
                {
                    Ok(Token::new(
                        And,
                        Span::new(current_index, 2, self.line, self.line_offset),
                    ))
                } else {
                    Ok(Token::new(
                        BitAnd,
                        Span::new(current_index, 1, self.line, self.line_offset),
                    ))
                }
            }
            '~' => Ok(self.make_single_char_token(current_index, BitNegate)),
            '|' => {
                self.advance();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '|')
                    .is_some()
                {
                    Ok(Token::new(
                        Or,
                        Span::new(current_index, 2, self.line, self.line_offset),
                    ))
                } else {
                    Ok(Token::new(
                        BitOr,
                        Span::new(current_index, 1, self.line, self.line_offset),
                    ))
                }
            }
            '^' => Ok(self.make_single_char_token(current_index, BitXor)),
            '=' => {
                self.advance();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '=')
                    .is_some()
                {
                    Ok(Token::new(
                        DoubleEqual,
                        Span::new(current_index, 2, self.line, self.line_offset),
                    ))
                } else if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '>')
                    .is_some()
                {
                    Ok(Token::new(
                        Arrow,
                        Span::new(current_index, 2, self.line, self.line_offset),
                    ))
                } else {
                    Ok(Token::new(
                        SingleEqual,
                        Span::new(current_index, 1, self.line, self.line_offset),
                    ))
                }
            }
            '!' => {
                self.advance();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '=')
                    .is_some()
                {
                    Ok(Token::new(
                        NotEqual,
                        Span::new(current_index, 2, self.line, self.line_offset),
                    ))
                } else {
                    Ok(Token::new(
                        ExclamationMark,
                        Span::new(current_index, 1, self.line, self.line_offset),
                    ))
                }
            }
            '"' => {
                self.advance();
                let size = self.read_string(current_index)?;
                Ok(Token::new(
                    StringLiteral,
                    Span::new(current_index, size, self.line, self.line_offset),
                ))
            }
            '<' => {
                self.advance();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '=')
                    .is_some()
                {
                    Ok(Token::new(
                        LessThanOrEqual,
                        Span::new(current_index, 2, self.line, self.line_offset),
                    ))
                } else if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '<')
                    .is_some()
                {
                    Ok(Token::new(
                        BitLShift,
                        Span::new(current_index, 2, self.line, self.line_offset),
                    ))
                } else {
                    Ok(Token::new(
                        LessThan,
                        Span::new(current_index, 1, self.line, self.line_offset),
                    ))
                }
            }
            '>' => {
                self.advance();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '=')
                    .is_some()
                {
                    Ok(Token::new(
                        GreaterThanOrEqual,
                        Span::new(current_index, 2, self.line, self.line_offset),
                    ))
                } else if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '>')
                    .is_some()
                {
                    Ok(Token::new(
                        BitRShift,
                        Span::new(current_index, 2, self.line, self.line_offset),
                    ))
                } else {
                    Ok(Token::new(
                        GreaterThan,
                        Span::new(current_index, 1, self.line, self.line_offset),
                    ))
                }
            }
            '/' => {
                self.advance();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '/')
                    .is_some()
                {
                    self.skip_line();
                    self.next_token()
                } else {
                    Ok(Token::new(
                        Slash,
                        Span::new(current_index, 1, self.line, self.line_offset),
                    ))
                }
            }
            '$' => {
                self.advance();
                let builtin = self.read_identifier(current_index);
                match registry.get(builtin) {
                    Some(kind) => Ok(Token::new(
                        kind.clone(),
                        Span::new(current_index, builtin.len(), self.line, self.line_offset),
                    )),
                    None => Err(LexerError::InvalidBuiltIn(
                        Span::new(current_index, builtin.len(), self.line, self.line_offset),
                        builtin.to_string(),
                    )),
                }
            }
            '@' => {
                self.advance();
                let directive = self.read_identifier(current_index);
                match registry.get(directive) {
                    Some(kind) => Ok(Token::new(
                        kind.clone(),
                        Span::new(current_index, directive.len(), self.line, self.line_offset),
                    )),
                    None => Err(LexerError::InvalidDirective(
                        Span::new(current_index, directive.len(), self.line, self.line_offset),
                        directive.to_string(),
                    )),
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = self.read_identifier(current_index);
                match registry.get(identifier) {
                    Some(kind) => Ok(Token::new(
                        kind.clone(),
                        Span::new(current_index, identifier.len(), self.line, self.line_offset),
                    )),
                    None => Ok(Token::new(
                        Identifier,
                        Span::new(current_index, identifier.len(), self.line, self.line_offset),
                    )),
                }
            }
            '0'..='9' => self.make_number_token(current_index),
            c => Err(LexerError::InvalidToken(
                Span::new(current_index, 1, self.line, self.line_offset),
                c,
            )),
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

    fn read_identifier(&mut self, position: usize) -> &'src str {
        let mut last = position;
        while self
            .peek()
            .is_some_and(|(_, c)| c.is_ascii_alphanumeric() || c == '_')
        {
            let token = self.advance().unwrap();
            last = token.0;
        }
        &self.content[position..=last]
    }

    fn make_number_token(&mut self, position: usize) -> Result<Token, LexerError> {
        let mut last = position;
        let mut is_float = false;
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
                    Span::new(position, last - position + 1, self.line, self.line_offset),
                ));
            }

            let token = self
                .advance()
                .expect("Next should exist, it is checked in while.");
            if is_float && token.1 == '.' {
                return Err(LexerError::InvalidNumberLiteral(Span::new(
                    position,
                    last - position + 1,
                    self.line,
                    self.line_offset,
                )));
            } else if token.1 == '.' {
                is_float = true;
            }
            last = token.0;
        }

        if let Some((_, char)) = self.peek() {
            if !matches!(char, ' ' | ';' | ',' | ']' | ')') {
                return Err(LexerError::InvalidNumberLiteral(Span::new(
                    position,
                    last - position,
                    self.line,
                    self.line_offset,
                )));
            }
        }

        Ok(Token::new(
            if is_float {
                TokenKind::FloatLiteral
            } else {
                TokenKind::IntLiteral
            },
            Span::new(position, last - position + 1, self.line, self.line_offset),
        ))
    }

    fn read_string(&mut self, position: usize) -> Result<usize, LexerError> {
        let mut last = position;
        while self.peek().is_some_and(|(_, c)| c != '"') {
            let (current_position, ch) = self
                .advance()
                .expect("Next should exist, it is checked in while.");
            if ch == '\n' {
                return Err(LexerError::MissingChar(
                    Span::new(position, last - position, self.line, self.line_offset),
                    '"',
                ));
            }
            last = current_position;
        }
        self.advance().ok_or(LexerError::MissingChar(
            Span::new(position, last, self.line, self.line_offset),
            '"',
        ))?;
        Ok(2 + last - position)
    }

    fn make_attribute(&mut self, position: usize) -> Result<Token, LexerError> {
        let mut last = position;
        while self.peek().is_some_and(|(_, c)| c != ']') {
            let (current_position, _) = self
                .advance()
                .expect("This will always be Some, since it is checked in while");
            last = current_position;
        }
        self.advance().ok_or(LexerError::MissingChar(
            Span::new(position, last - position, self.line, self.line_offset),
            ']',
        ))?;
        Ok(Token::new(
            TokenKind::Tag,
            Span::new(position, last - position + 1, self.line, self.line_offset),
        ))
    }

    fn make_single_char_token(&mut self, current_index: usize, kind: TokenKind) -> Token {
        let token = Token::new(
            kind,
            Span::new(current_index, 1, self.line, self.line_offset),
        );
        self.advance();
        token
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        if let Some((idx, ch)) = self.chars.next() {
            self.current_position = idx;

            if ch == '\n' {
                self.line += 1;
                self.line_offset = 1;
            } else {
                self.line_offset += 1;
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
