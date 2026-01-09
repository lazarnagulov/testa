#![allow(unused)]

use testa_core::utils::Span;

#[derive(Debug, Clone)]
pub struct RawToken {
    pub line: u32,
    pub start: u32,
    pub length: u32,
    pub token_type: u32,
    pub modifiers: u32,
}

impl RawToken {
    pub fn new(span: Span, token_type: u32, modifiers: u32) -> Self {
        Self {
            line: span.start.line,
            start: span.start.column,
            length: span.end.column - span.start.column,
            token_type,
            modifiers,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Keyword = 0,
    Type = 1,
    Enum = 2,
    EnumMember = 3,
    Property = 4,
    String = 5,
    Number = 6,
    Operator = 7,
    Comment = 8,
    Macro = 9,
    Decorator = 10,
}

pub const MODIFIER_DECLARATION: u32 = 1 << 0;
pub const MODIFIER_DEFINITION: u32 = 1 << 1;
pub const MODIFIER_READONLY: u32 = 1 << 2;
