#![allow(dead_code)]

use core::fmt;

pub struct IR(pub Vec<Block>);

pub struct Block {
    pub label: String,
    pub instructions: Vec<Instruction>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:", self.label)?;
        for instruction in &self.instructions {
            writeln!(f, "  {}", instruction)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum BaseType {
    Int,
    Str,
    Bool,
    Float,
}

impl fmt::Display for BaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseType::Int => write!(f, "int"),
            BaseType::Str => write!(f, "string"),
            BaseType::Bool => write!(f, "bool"),
            BaseType::Float => write!(f, "float"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Temp(usize),
    Int(i32),
    Float(f32),
    Boolean(bool),
    Str(String),
    Identifier(String),
    NoValue,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Temp(id) => write!(f, "t{id}"),
            Value::Int(value) => write!(f, "{value}"),
            Value::Float(value) => write!(f, "{value}"),
            Value::Boolean(value) => write!(f, "{value}"),
            Value::Str(value) => write!(f, "{value}"),
            Value::Identifier(value) => write!(f, "{value}"),
            Value::NoValue => write!(f, ""),
        }
    }
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
    BeginEnum {
        name: String,
    },
    EnumVariant {
        name: String,
        weight: Value,
    },
    Attr {
        name: String,
    },
    Constraint {
        name: String,
        value: Value,
    },
    BeginTemplate {
        name: String,
    },
    TemplateExtends {
        name: String,
    },
    BeginField {
        name: String,
    },
    Assign {
        target: Value,
        source: Value,
    },
    BegindType {
        name: String,
        base_type: BaseType,
    },
    Add {
        target: Value,
        left: Value,
        right: Value,
    },
    Sub {
        target: Value,
        left: Value,
        right: Value,
    },
    Mul {
        target: Value,
        left: Value,
        right: Value,
    },
    Div {
        target: Value,
        left: Value,
        right: Value,
    },
    Mod {
        target: Value,
        left: Value,
        right: Value,
    },
    BitAnd {
        target: Value,
        left: Value,
        right: Value,
    },
    BitOr {
        target: Value,
        left: Value,
        right: Value,
    },
    BitXor {
        target: Value,
        left: Value,
        right: Value,
    },
    BitNegate {
        target: Value,
        source: Value,
    },
    LShift {
        target: Value,
        left: Value,
        right: Value,
    },
    RShift {
        target: Value,
        left: Value,
        right: Value,
    },
    Negative {
        target: Value,
        source: Value,
    },
    EndType,
    EndField,
    EndTemplate,
    EndEnum,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::BeginEnum { name } => write!(f, "BEGIN_ENUM {name}"),
            Instruction::EnumVariant { name, weight } => {
                write!(f, "    VARIANT {} {}", name, weight)
            }
            Instruction::Attr { name } => write!(f, "    ATTR {}", name),
            Instruction::EndType => write!(f, "END_TYPE"),
            Instruction::BeginTemplate { name } => write!(f, "TEMPLATE {name}"),
            Instruction::TemplateExtends { name } => write!(f, "    EXTEND {name}"),
            Instruction::BeginField { name } => write!(f, "    FIELD {name}"),
            Instruction::Assign { target, source } => write!(f, "    {target} := {source}"),
            Instruction::Add {
                target,
                left,
                right,
            } => write!(f, "    {target} := {left} + {right}"),
            Instruction::Sub {
                target,
                left,
                right,
            } => write!(f, "    {target} := {left} - {right}"),
            Instruction::Mul {
                target,
                left,
                right,
            } => write!(f, "    {target} := {left} * {right}"),
            Instruction::Div {
                target,
                left,
                right,
            } => write!(f, "    {target} := {left} / {right}"),
            Instruction::Mod {
                target,
                left,
                right,
            } => write!(f, "    {target} := {left} % {right}"),
            Instruction::BitAnd {
                target,
                left,
                right,
            } => write!(f, "    {target} := {left} & {right}"),
            Instruction::BitOr {
                target,
                left,
                right,
            } => write!(f, "    {target} := {left} | {right}"),
            Instruction::BitXor {
                target,
                left,
                right,
            } => write!(f, "    {target} := {left} ^ {right}"),
            Instruction::BitNegate { target, source } => write!(f, "    {target} := ~{source}"),
            Instruction::LShift {
                target,
                left,
                right,
            } => write!(f, "    {target} := {left} << {right}"),
            Instruction::RShift {
                target,
                left,
                right,
            } => write!(f, "    {target} := {left} >> {right}"),
            Instruction::Negative { target, source } => write!(f, "    {target} = -{source}"),
            Instruction::BegindType { name, base_type } => {
                write!(f, "TYPE {name} : {base_type}")
            }
            Instruction::Constraint { name, value } => write!(f, "    CONSTRAINT {name} {value}"),
            Instruction::EndField => write!(f, "    END_FIELD"),
            Instruction::EndTemplate => write!(f, "END_TEMPLATE"),
            Instruction::EndEnum => write!(f, "END_ENUM"),
        }
    }
}
