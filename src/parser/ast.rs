use core::fmt;

#[derive(PartialEq, Eq, Debug)]
pub struct Program(pub Vec<Statement>);

#[derive(PartialEq, Eq, Debug)]
pub enum Statement {
    Expression(ExpressionStatemnt),
    Template {
        name: String,
        body: Vec<Field>,
    },
    OutputDirective {
        argument: String,
        options: Vec<Field>,
    },
    TypeDecl {
        name: String,
        data_type: Expression,
    },
    ConstraintDecl {
        name: String,
        constraint: ConstraintExpression,
    },
    Enum {
        name: String,
        variants: Vec<Variant>,
    },
    Resource {
        name: String,
        body: Vec<Field>,
    },
    Generate {
        template_name: Option<String>,
        body: Vec<Field>,
        count: Expression,
    },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Variant {
    pub name: String,
    pub weight: Option<Expression>,
}

impl Variant {
    pub fn new(name: String, weight: Option<Expression>) -> Self {
        Variant { name, weight }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
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
    pub expression: Expression,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub start: usize,
    pub size: usize,
}

impl Expression {
    pub fn new(kind: ExpressionKind, start: usize, size: usize) -> Self {
        Expression { kind, start, size }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ExpressionKind {
    IntLiteral(isize),
    FloatLiteral(String),
    StringLiteral(String),
    BooleanLiteral(bool),
    Identifier(String),
    Type(DataType),
    Prefix {
        operator: PrefixOperator,
        expression: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: InfixOperator,
        right: Box<Expression>,
    },
    FuncCall {
        arguments: Vec<Expression>,
    },
}

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

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ConstraintExpression {
    pub expression: Expression,
    pub constraint_kind: ConstraintKind,
}

impl ConstraintExpression {
    pub fn new(expression: Expression, constraint_kind: ConstraintKind) -> Self {
        ConstraintExpression {
            expression,
            constraint_kind,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ConstraintKind {
    Range,
    MultipleOf,
    Length,
    Matches,
    NotMatches,
    In,
    NotIn,
    Containts,
    StartsWith,
    EndsWith,
    Custom,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DataType {
    pub data_type_kind: DataTypeKind,
    pub constraints: Option<Vec<ConstraintExpression>>,
}

impl DataType {
    pub fn new(
        data_type_kind: DataTypeKind,
        constraints: Option<Vec<ConstraintExpression>>,
    ) -> Self {
        DataType {
            data_type_kind,
            constraints,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum DataTypeKind {
    Int,
    Str,
    Float,
}

impl fmt::Display for DataTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataTypeKind::Int => write!(f, "int"),
            DataTypeKind::Str => write!(f, "string"),
            DataTypeKind::Float => write!(f, "float"),
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
