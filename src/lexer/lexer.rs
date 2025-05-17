use std::{iter::Peekable, str::CharIndices};

use super::token::{Token, TokenKind};

pub struct Lexer<'src> {
    content:  &'src str,
    chars: Peekable<CharIndices<'src>>,
}

impl<'src> Lexer<'src> {

    pub fn new(program: &'src str) -> Self {
        Lexer {
            content: program,
            chars: program.char_indices().peekable()
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
            '[' => self.make_single_char_token(current_index, LBracket),
            ']' => self.make_single_char_token(current_index, RBracket),
            '$' => self.make_single_char_token(current_index, Dollar),
            '.' => self.make_single_char_token(current_index, Period),
            '+' => self.make_single_char_token(current_index, Plus),
            '-' => self.make_single_char_token(current_index, Minus),
            '*' => self.make_single_char_token(current_index, Asterisk),
            '&' => self.make_single_char_token(current_index, BitAnd),
            '|' => self.make_single_char_token(current_index, BitOr),
            '^' => self.make_single_char_token(current_index, BitXor),
            '=' => {
                self.next();
                if self.chars.next_if(|(_, next_char)| *next_char == '=').is_some() {
                    Token::new(DoubleEqual, current_index, 2)
                } else {
                    Token::new(SingleEqual, current_index, 1)
                }
            },
            '!' => {
                self.next();
                if self.chars.next_if(|(_, next_char)| *next_char == '=').is_some() {
                    Token::new(NotEqual, current_index, 2)
                } else {
                    Token::new(ExclamationMark, current_index, 1)
                }
            },
            '"' => {
                self.next();
                let size = self.read_string(current_index);
                println!("Found string: {:?}", self.content[current_index + 1..current_index + size].to_string());
                Token::new(StringLiteral, current_index + 1, size)
            },
            '<' => {
                self.next();
                if self
                    .chars
                    .next_if(|(_, next_char)| *next_char == '=')
                    .is_some()
                {
                    Token::new(LessThanOrEqual, current_index, 2)
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
                } else {
                    Token::new(GreaterThan, current_index, 1)
                }
            },
            '/' => {
                self.next();
                if self.chars.next_if(|(_, next_char)| *next_char == '/').is_some() {
                    self.skip_line();
                    self.next_token()
                } else {
                    Token::new(Slash, current_index, 1)
                }
            }
            '@' => {
                self.next();
                let directive = self.read_identifier(current_index);
                match directive {
                    "@output" => Token::new(Output, current_index, 6),
                    "@seed" => Token::new(Seed, current_index, 4),
                    _ => panic!("Invalid directive {}", directive)
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = self.read_identifier(current_index);
                match identifier {
                    "generate" => Token::new(Generate, current_index, 8),
                    "template" => Token::new(Template, current_index, 8),
                    "resource" => Token::new(Resource, current_index, 8),
                    ident => {
                        println!("Found identifier: {:?}", ident);
                        Token::new(Identifier, current_index, ident.len())
                    }
                }
            }
            '0'..='9' =>  {
                let size = self.read_number(current_index);
                println!("Found integer: {:?}", self.content[current_index..current_index + size].to_string());
                Token::new(IntLiteral, current_index, size)
            }
            c => panic!("Invalid token {}", c)
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
        while self.peek().is_some_and(|(_, c)| c.is_ascii_alphabetic() || c == '_') {
            let token = self.next().unwrap();
            last = token.0;
        }
        &self.content[position..=last]
    }

    fn read_number(&mut self, position: usize) -> usize {
        let mut last = position;
        while self.peek().is_some_and(|(_, c)| c.is_ascii_digit()) {
            let token = self.next().unwrap();
            last = token.0;
        }
        self.content[position..=last].len()
    }

    fn read_string(&mut self, position: usize) -> usize {
        let mut last = position;
        while self.peek().is_some_and(|(_, c)| c != '"') {
            let token = self.next().unwrap();
            last = token.0;
        }
        self.next();
        self.content[position..=last].len()
    }

    fn make_single_char_token(&mut self, current_index: usize, kind: TokenKind) -> Token {
        let token = Token::new(kind, current_index, 1);
        self.next();
        token
    }

    fn next(&mut self) -> Option<(usize, char)>  {
        self.chars.next()
    }

    fn skip_whitespaces(&mut self) {
        while self.peek().is_some_and(|(_,c)| c.is_whitespace()) {
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