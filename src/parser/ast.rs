
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
    StringLiteral(String),
    BooleanLiteral(bool),
    Identifier(String),
    // TODO: add type options e.g. int { 0..=100 }
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


