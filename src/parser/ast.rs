use core::fmt;


#[derive(PartialEq, Eq, Debug)]
pub struct Program(pub Vec<Statement>);

#[derive(PartialEq, Eq, Debug)]
pub enum Statement {
    Expression(ExpressionStatemnt),
    Template {
        name: String,
        body: Vec<Field>
    },
    OutputDirective {
        argument: String,
        options: Vec<Field>
    },
    Enum {
        name: String,
        variants: Vec<String>
    },
    Resource {
        name: String,
        body: Vec<Field>
    },
    Generate {
        template_name: Option<String>,
        body: Vec<Field>,
        count: Expression,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Field {
    pub name: String,
    pub value: Expression,
} 

impl Field {
    pub fn new(name: String, value: Expression) -> Self {
        Field { name, value }
    }
}


#[derive(PartialEq, Eq, Debug)]
pub struct ExpressionStatemnt {
    pub expression: Expression
}

#[derive(PartialEq, Eq, Debug)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub start: usize,
    pub size: usize
}

impl Expression {
    pub fn new(kind: ExpressionKind, start: usize, size: usize) -> Self {
        Expression { kind, start, size }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ExpressionKind {
    IntLiteral(isize),
    FloatLiteral(String),
    StringLiteral(String),
    BooleanLiteral(bool),
    Identifier(String),
    // TODO: add type constraints e.g. int<0..=32> [ range = 0..=100 ]
    Type(String),
    Prefix {
        operator: PrefixOperator,
        expression: Box<Expression>
    },
    Infix {
        left: Box<Expression>,
        operator: InfixOperator,
        right: Box<Expression>
    },
    FuncCall {
        arguments: Vec<Expression>
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PrefixOperator {
    BitNegate,
    LogicalNegate,
    Negative,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum InfixOperator {
    Plus,
    Minus,
    Divide,
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
    InclusiveRange
}

impl fmt::Display for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfixOperator::Plus => write!(f, "+"),
            InfixOperator::Minus => write!(f, "-"),
            InfixOperator::Divide => write!(f, "/"),
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


