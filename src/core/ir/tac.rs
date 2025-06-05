#![allow(dead_code)]

pub struct IR(Vec<Instruction>);

#[derive(Debug, Clone)]
pub enum BaseType {
    Int,
    Str,
    Bool,
    Float,
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    Boolean(bool),
    Str(String),
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    BitNegate,
    LogicalNegate,
    Negative,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Plus,
    Minus,
    Divide,
    Mod,
    Multiply,
    BitAnd,
    BitOr,
    BitXor,
    BitLShift,
    BitRShift,
    Equal,
    And,
    Or,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    ExclusiveRange,
    InclusiveRange,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Const(Value),
    EnumBegin {
        name: String,
    },
    EnumVariant {
        name: String,
    },
    EnumEnd,
    TemplateBegin {
        name: String,
    },
    TemplateExtends {
        name: String,
    },
    TemplateEnd,
    FieldBegin {
        name: String,
        ty: BaseType,
    },
    FieldConst(Value),
    FieldBinOp {
        left: Value,
        op: BinaryOp,
        right: Value,
    },
    FieldUnaryOp {
        value: Value,
        op: UnaryOp,
    },
    FieldAttr {
        name: String,
    },
    FieldEnd,
}
