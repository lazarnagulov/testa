use std::fmt::Display;

// TODO: Add lookup table for directives, keywords and builtins.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenKind {
    // directives -- starts with @
    Output,
    Seed,

    // builtins -- starts with $
    Pick,
    Uuid,

    // keywords
    Generate,
    Template,
    Resource,
    Constraint,
    Extend,
    With,
    Type,
    Enum,
    False,
    True,
    Int,
    Float,
    Str,

    // syntax
    LParen,            // (
    RParen,            // )
    LBrace,            // {
    RBrace,            // }
    LBracket,          // [
    RBracket,          // ]
    Colon,             // :
    Comma,             // ,
    SinglePeriod,      // .
    Arrow,             // =>
    Semicolon,         // ;
    SingleEqual,       // =
    DoublePeriod,      // ..
    DoublePeriodEqual, //..=

    // operators:
    ExclamationMark,    // !
    Plus,               // +
    Minus,              // -
    Slash,              // /
    Percent,            // %
    Asterisk,           // *
    And,                // &&
    Or,                 // ||
    BitAnd,             // &
    BitOr,              // |
    BitXor,             // ^
    BitLShift,          // <<
    BitRShift,          // >>
    BitNegate,          // ~
    DoubleEqual,        // ==
    NotEqual,           // !=
    LessThan,           // <
    GreaterThan,        // >
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=

    Identifier,
    IntLiteral,
    StringLiteral,
    FloatLiteral,
    Eof,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TokenKind::Output => "@output",
            TokenKind::Seed => "@seed",
            TokenKind::Generate => "generate",
            TokenKind::Template => "template",
            TokenKind::Resource => "resource",
            TokenKind::Int => "int",
            TokenKind::Float => "float",
            TokenKind::Str => "string",
            TokenKind::Extend => "extend",
            TokenKind::With => "with",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::Colon => ":",
            TokenKind::Comma => ",",
            TokenKind::Semicolon => ";",
            TokenKind::SingleEqual => "=",
            TokenKind::Arrow => "=>",
            TokenKind::Identifier => "identifier",
            TokenKind::Constraint => "constraint",
            TokenKind::Type => "type",
            TokenKind::Eof => "EOF",
            TokenKind::ExclamationMark => "!",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::Asterisk => "*",
            TokenKind::And => "&&",
            TokenKind::Or => "||",
            TokenKind::BitAnd => "&",
            TokenKind::BitNegate => "~",
            TokenKind::BitOr => "|",
            TokenKind::BitXor => "^",
            TokenKind::DoubleEqual => "==",
            TokenKind::NotEqual => "!=",
            TokenKind::LessThan => "<",
            TokenKind::BitLShift => "<<",
            TokenKind::BitRShift => ">>",
            TokenKind::GreaterThan => ">",
            TokenKind::LessThanOrEqual => "<=",
            TokenKind::GreaterThanOrEqual => ">=",
            TokenKind::IntLiteral => "integer literal",
            TokenKind::LBracket => "[",
            TokenKind::RBracket => "]",
            TokenKind::StringLiteral => "string literal",
            TokenKind::FloatLiteral => "float literal",
            TokenKind::SinglePeriod => ".",
            TokenKind::DoublePeriod => "..",
            TokenKind::DoublePeriodEqual => "..=",
            TokenKind::Pick => "pick",
            TokenKind::Uuid => "uuid",
            TokenKind::Enum => "enum",
            TokenKind::False => "false",
            TokenKind::True => "true",
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
