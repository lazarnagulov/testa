use std::{iter::Peekable, str::CharIndices};

use super::token::{Token, TokenKind};

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

    fn next_token(&mut self) -> Token {
        use super::token::TokenKind::*;
        self.skip_whitespaces();

        let Some((current_index, current_char)) = self.peek() else {
            return Token::new(super::token::TokenKind::Eof, self.content.len(), 1);
        };

        match current_char {
            '(' => self.make_single_char_token(current_index, LParen),
            ')' => self.make_single_char_token(current_index, RParen),
            '{' => self.make_single_char_token(current_index, LBrace),
            '}' => self.make_single_char_token(current_index, RBrace),
            ':' => self.make_single_char_token(current_index, Colon),
            ',' => self.make_single_char_token(current_index, Comma),
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
                match builtin {
                    "$pick" => Token::new(Pick, current_index, 4),
                    "$uuid" => Token::new(Uuid, current_index, 4),
                    _ => panic!("Invalid builtin {}", builtin),
                }
            }
            '@' => {
                self.next();
                let directive = self.read_identifier(current_index);
                match directive {
                    "@output" => Token::new(Output, current_index, 6),
                    "@seed" => Token::new(Seed, current_index, 4),
                    _ => panic!("Invalid directive {}", directive),
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = self.read_identifier(current_index);
                match identifier {
                    "generate" => Token::new(Generate, current_index, 8),
                    "template" => Token::new(Template, current_index, 8),
                    "resource" => Token::new(Resource, current_index, 8),
                    "true" => Token::new(True, current_index, 4),
                    "false" => Token::new(False, current_index, 4),
                    "enum" => Token::new(Enum, current_index, 4),
                    "int" => Token::new(Int, current_index, 3),
                    "float" => Token::new(Float, current_index, 5),
                    "string" => Token::new(Str, current_index, 6),
                    ident => Token::new(Identifier, current_index, ident.len()),
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
            .is_some_and(|(_, c)| c.is_ascii_alphabetic() || c == '_')
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
            let token = self.next().unwrap();
            if is_float && token.1 == '.' {
                panic!("Invalid float literal");
            } else if token.1 == '.' {
                is_float = true;
            }
            last = token.0;
        }

        if let Some((_, char)) = self.peek() {
            if !matches!(char, ' ' | ';' | ']' | ')') {
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
