use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenKind {
    // directives -- starts with @
    Output,
    Seed,

    // keywords
    Generate,
    Template,
    Resource,

    // syntax
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]
    Dollar,      // $
    Colon,       // :
    Comma,       // ,
    Period,      // .
    Semicolon,   // ;
    SingleEqual, // =

    // operators:
    ExclamationMark,    // !
    Plus,               // +
    Minus,              // -
    Slash,              // /
    Asterisk,           // *
    BitAnd,             // &
    BitOr,              // |
    BitXor,             // ^
    DoubleEqual,        // ==
    NotEqual,           // !=
    LessThan,           // <
    GreaterThan,        // >
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=

    Identifier,
    IntLiteral,
    StringLiteral,
    Eof
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TokenKind::Output => "@output",
            TokenKind::Seed => "@seed",
            TokenKind::Generate => "generate",
            TokenKind::Template => "template",
            TokenKind::Resource => "resource",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::Colon => ":",
            TokenKind::Comma => ",",
            TokenKind::Semicolon => ";",
            TokenKind::SingleEqual => "=",
            TokenKind::Identifier => "identifier",
            TokenKind::Eof => "EOF",
            TokenKind::ExclamationMark => "!",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Slash => "/",
            TokenKind::Asterisk => "*",
            TokenKind::BitAnd => "&",
            TokenKind::BitOr => "|",
            TokenKind::BitXor => "^",
            TokenKind::DoubleEqual => "==",
            TokenKind::NotEqual => "!=",
            TokenKind::LessThan => "<",
            TokenKind::GreaterThan => ">",
            TokenKind::LessThanOrEqual => "<=",
            TokenKind::GreaterThanOrEqual => ">=",
            TokenKind::IntLiteral => "integer literal",
            TokenKind::LBracket => "[",
            TokenKind::RBracket => "]",
            TokenKind::StringLiteral => "string literal",
            TokenKind::Dollar => "$",
            TokenKind::Period => "."
        };
        f.write_str(str)
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub size: usize,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, size: usize) -> Self {
        Token { kind, start, size }
    }
}