pub mod token;

use std::{iter::Peekable, str::CharIndices};

use crate::core::lexer::token::{KEYWORD_REGISTRY, Token, TokenKind};

#[derive(Clone, Debug)]
pub struct Lexer<'src> {
    content: &'src str,
    chars: Peekable<CharIndices<'src>>,
}

impl<'src> Lexer<'src> {
    pub fn new(program: &'src str) -> Self {
        Lexer {
            content: program,
            chars: program.char_indices().peekable(),
        }
    }

    fn peek(&mut self) -> Option<(usize, char)> {
        self.chars.peek().copied()
    }

    fn peek_n(&self, n: usize) -> Option<(usize, char)> {
        self.chars.clone().nth(n - 1)
    }

    fn next_token(&mut self) -> Token {
        use crate::core::lexer::token::TokenKind::*;
        let registry = &KEYWORD_REGISTRY;
        self.skip_whitespaces();

        let Some((current_index, current_char)) = self.peek() else {
            return Token::new(Eof, self.content.len(), 1);
        };

        match current_char {
            '(' => self.make_single_char_token(current_index, LParen),
            ')' => self.make_single_char_token(current_index, RParen),
            '{' => self.make_single_char_token(current_index, LBrace),
            '}' => self.make_single_char_token(current_index, RBrace),
            ':' => self.make_single_char_token(current_index, Colon),
            ',' => self.make_single_char_token(current_index, Comma),
            '#' => {
                self.next();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '[')
                    .is_some()
                {
                    self.make_attribute(current_index)
                } else {
                    panic!("Invalid token #");
                }
            }
            ';' => self.make_single_char_token(current_index, Semicolon),
            '%' => self.make_single_char_token(current_index, Percent),
            '[' => self.make_single_char_token(current_index, LBracket),
            ']' => self.make_single_char_token(current_index, RBracket),
            '.' => {
                self.next();
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
                        Token::new(DoublePeriodEqual, current_index, 3)
                    } else {
                        Token::new(DoublePeriod, current_index, 2)
                    }
                } else {
                    Token::new(SinglePeriod, current_index, 1)
                }
            }
            '+' => self.make_single_char_token(current_index, Plus),
            '-' => self.make_single_char_token(current_index, Minus),
            '*' => self.make_single_char_token(current_index, Asterisk),
            '&' => {
                self.next();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '&')
                    .is_some()
                {
                    Token::new(And, current_index, 2)
                } else {
                    Token::new(BitAnd, current_index, 1)
                }
            }
            '~' => self.make_single_char_token(current_index, BitNegate),
            '|' => {
                self.next();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '|')
                    .is_some()
                {
                    Token::new(Or, current_index, 2)
                } else {
                    Token::new(BitOr, current_index, 1)
                }
            }
            '^' => self.make_single_char_token(current_index, BitXor),
            '=' => {
                self.next();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '=')
                    .is_some()
                {
                    Token::new(DoubleEqual, current_index, 2)
                } else if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '>')
                    .is_some()
                {
                    Token::new(Arrow, current_index, 2)
                } else {
                    Token::new(SingleEqual, current_index, 1)
                }
            }
            '!' => {
                self.next();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '=')
                    .is_some()
                {
                    Token::new(NotEqual, current_index, 2)
                } else {
                    Token::new(ExclamationMark, current_index, 1)
                }
            }
            '"' => {
                self.next();
                let size = self.read_string(current_index);
                Token::new(StringLiteral, current_index, size)
            }
            '<' => {
                self.next();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '=')
                    .is_some()
                {
                    Token::new(LessThanOrEqual, current_index, 2)
                } else if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '<')
                    .is_some()
                {
                    Token::new(BitLShift, current_index, 2)
                } else {
                    Token::new(LessThan, current_index, 1)
                }
            }
            '>' => {
                self.next();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '=')
                    .is_some()
                {
                    Token::new(GreaterThanOrEqual, current_index, 2)
                } else if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '>')
                    .is_some()
                {
                    Token::new(BitRShift, current_index, 2)
                } else {
                    Token::new(GreaterThan, current_index, 1)
                }
            }
            '/' => {
                self.next();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '/')
                    .is_some()
                {
                    self.skip_line();
                    self.next_token()
                } else {
                    Token::new(Slash, current_index, 1)
                }
            }
            '$' => {
                self.next();
                let builtin = self.read_identifier(current_index);
                match registry.get(builtin) {
                    Some(kind) => Token::new(kind.clone(), current_index, builtin.len()),
                    None => panic!("Invalid builtin {}", builtin),
                }
            }
            '@' => {
                self.next();
                let directive = self.read_identifier(current_index);
                match registry.get(directive) {
                    Some(kind) => Token::new(kind.clone(), current_index, directive.len()),
                    None => panic!("Invalid directive {}", directive),
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = self.read_identifier(current_index);
                match registry.get(identifier) {
                    Some(kind) => Token::new(kind.clone(), current_index, identifier.len()),
                    None => Token::new(Identifier, current_index, identifier.len()),
                }
            }
            '0'..='9' => self.make_number_token(current_index),
            c => panic!("Invalid token {}", c),
        }
    }

    fn skip_line(&mut self) {
        while let Some((_, c)) = self.peek() {
            if c == '\n' {
                return;
            }
            self.next();
        }
    }

    fn read_identifier(&mut self, position: usize) -> &'src str {
        let mut last = position;
        while self
            .peek()
            .is_some_and(|(_, c)| c.is_ascii_alphanumeric() || c == '_')
        {
            let token = self.next().unwrap();
            last = token.0;
        }
        &self.content[position..=last]
    }

    fn make_number_token(&mut self, position: usize) -> Token {
        let mut last = position;
        let mut is_float = false;
        while self
            .peek()
            .is_some_and(|(_, c)| c.is_ascii_digit() || c == '.')
        {
            if self.peek().unwrap().1 == '.' && self.peek_n(2).is_some_and(|(_, c)| c == '.') {
                return Token::new(
                    if is_float {
                        TokenKind::FloatLiteral
                    } else {
                        TokenKind::IntLiteral
                    },
                    position,
                    last - position + 1,
                );
            }

            let token = self.next().unwrap();
            if is_float && token.1 == '.' {
                panic!("Invalid float literal");
            } else if token.1 == '.' {
                is_float = true;
            }
            last = token.0;
        }

        if let Some((_, char)) = self.peek() {
            if !matches!(char, ' ' | ';' | ',' | ']' | ')') {
                panic!("Invalid int or float literal");
            }
        }

        Token::new(
            if is_float {
                TokenKind::FloatLiteral
            } else {
                TokenKind::IntLiteral
            },
            position,
            last - position + 1,
        )
    }

    fn read_string(&mut self, position: usize) -> usize {
        let mut last = position;
        while self.peek().is_some_and(|(_, c)| c != '"') {
            let (current_position, ch) = self.next().unwrap();
            if ch == '\n' {
                panic!("Invalid string literal: missing closing quote");
            }
            last = current_position;
        }
        match self.next() {
            Some(..) => {}
            None => panic!("Invalid string literal: missing closing quote"),
        }
        // Add "" to size
        2 + last - position
    }

    fn make_attribute(&mut self, position: usize) -> Token {
        let mut last = position;
        while self.peek().is_some_and(|(_, c)| c != ']') {
            let (current_position, _) = self
                .next()
                .expect("This will always be Some, since it is checked in while");
            last = current_position;
        }
        match self.next() {
            Some(..) => {}
            None => panic!("Invalid attribute: missing closing brace"),
        }
        Token::new(TokenKind::Tag, position, last - position + 1)
    }

    fn make_single_char_token(&mut self, current_index: usize, kind: TokenKind) -> Token {
        let token = Token::new(kind, current_index, 1);
        self.next();
        token
    }

    fn next(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }

    fn skip_whitespaces(&mut self) {
        while self.peek().is_some_and(|(_, c)| c.is_whitespace()) {
            self.next();
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token.kind == TokenKind::Eof {
            None
        } else {
            Some(token)
        }
    }
}
