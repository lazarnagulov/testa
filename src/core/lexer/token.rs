use std::{collections::HashMap, fmt::Display};

use once_cell::sync::Lazy;

use crate::core::{lexer::error::LexerError, utils::span::Span};

macro_rules! keywords {
    ($($str:expr => $kind:expr),* $(,)?) => {
        {
            let mut m = HashMap::new();
            $(m.insert($str, $kind);)*
            m
        }
    };
}

pub static KEYWORD_REGISTRY: Lazy<HashMap<&str, TokenKind>> = Lazy::new(|| {
    keywords! {
        "output" => TokenKind::Output,
        "generate" => TokenKind::Generate,
        "seed" => TokenKind::Seed,
        "output_path" => TokenKind::OutputPath,
        "pick" => TokenKind::Pick,
        "uuid" => TokenKind::Uuid,
        "template" => TokenKind::Template,
        "resource" => TokenKind::Resource,
        "with" => TokenKind::With,
        "extend" => TokenKind::Extend,
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "enum" => TokenKind::Enum,
        "int" => TokenKind::Int,
        "float" => TokenKind::Float,
        "bool" => TokenKind::Bool,
        "string" => TokenKind::Str,
        "override" => TokenKind::Override,
        "type" => TokenKind::Type,
        "string_template" => TokenKind::StringTemplate,
        "string_pattern" => TokenKind::StringPattern,
        "constraint" => TokenKind::Constraint,
    }
});

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenKind {
    // directives -- starts with @
    Output,
    Generate,
    Seed,
    OutputPath,

    // builtins -- starts with $
    Pick,
    Uuid,

    // keywords
    Template,
    StringTemplate,
    StringPattern,
    Resource,
    Constraint,
    Override,
    Extend,
    With,
    Type,
    Enum,
    False,
    True,
    Int,
    Float,
    Str,
    Bool,

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
    Error(LexerError),
    Tag,
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
            TokenKind::OutputPath => "@output_path",
            TokenKind::Generate => "generate",
            TokenKind::Template => "template",
            TokenKind::Resource => "resource",
            TokenKind::Int => "int",
            TokenKind::Float => "float",
            TokenKind::Str => "string",
            TokenKind::Bool => "bool",
            TokenKind::Extend => "extend",
            TokenKind::Override => "override",
            TokenKind::StringTemplate => "string_template",
            TokenKind::StringPattern => "string_pattern",
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
            TokenKind::Tag => "tag",
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
            TokenKind::Error(lexer_error) => &format!("error({})", lexer_error),
        };
        f.write_str(str)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }
}
