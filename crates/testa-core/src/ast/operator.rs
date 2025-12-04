use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PrefixOperator {
    BitNegate,
    LogicalNegate,
    Negative,
}

impl fmt::Display for PrefixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrefixOperator::BitNegate => write!(f, "~"),
            PrefixOperator::LogicalNegate => write!(f, "!"),
            PrefixOperator::Negative => write!(f, "-"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum InfixOperator {
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

impl fmt::Display for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfixOperator::Plus => write!(f, "+"),
            InfixOperator::Minus => write!(f, "-"),
            InfixOperator::Divide => write!(f, "/"),
            InfixOperator::Mod => write!(f, "%"),
            InfixOperator::Multiply => write!(f, "*"),
            InfixOperator::BitAnd => write!(f, "&"),
            InfixOperator::BitOr => write!(f, "|"),
            InfixOperator::BitXor => write!(f, "^"),
            InfixOperator::BitLShift => write!(f, "<<"),
            InfixOperator::BitRShift => write!(f, ">>"),
            InfixOperator::Equal => write!(f, "="),
            InfixOperator::And => write!(f, "&&"),
            InfixOperator::Or => write!(f, "||"),
            InfixOperator::NotEqual => write!(f, "!="),
            InfixOperator::LessThan => write!(f, "<"),
            InfixOperator::GreaterThan => write!(f, ">"),
            InfixOperator::LessThanOrEqual => write!(f, "<="),
            InfixOperator::GreaterThanOrEqual => write!(f, ">="),
            InfixOperator::ExclusiveRange => write!(f, "exclusive range"),
            InfixOperator::InclusiveRange => write!(f, "inclusive range"),
        }
    }
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug)]
pub enum Precedence {
    Lowest,
    Equality,   // == or !=
    Range,      // .. or ..=
    Comparison, // <, <=, >, >=
    Bitwise,    // &, |, ^
    Sum,        // + or -
    Product,    // * or /
    Group,      // ( )
    Prefix,     // -X or !X
}
