use std::{collections::HashMap, fmt::Display};

use once_cell::sync::Lazy;

pub static KEYWORD_REGISTRY: Lazy<HashMap<&str, TokenKind>> = Lazy::new(|| {
    let mut m: HashMap<&str, TokenKind> = HashMap::new();
    m.insert("@output", TokenKind::Output);
    m.insert("@generate", TokenKind::Generate);
    m.insert("@seed", TokenKind::Seed);
    m.insert("@output_path", TokenKind::OutputPath);
    m.insert("$pick", TokenKind::Pick);
    m.insert("$uuid", TokenKind::Uuid);
    m.insert("$template", TokenKind::Uuid);
    m.insert("template", TokenKind::Template);
    m.insert("resource", TokenKind::Resource);
    m.insert("with", TokenKind::With);
    m.insert("extend", TokenKind::Extend);
    m.insert("true", TokenKind::True);
    m.insert("false", TokenKind::False);
    m.insert("enum", TokenKind::Enum);
    m.insert("int", TokenKind::Int);
    m.insert("float", TokenKind::Float);
    m.insert("bool", TokenKind::Bool);
    m.insert("string", TokenKind::Str);
    m.insert("override", TokenKind::Override);
    m.insert("type", TokenKind::Type);
    m.insert("string_template", TokenKind::StringTemplate);
    m.insert("string_pattern", TokenKind::StringPattern);
    m.insert("constraint", TokenKind::Constraint);
    m
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
