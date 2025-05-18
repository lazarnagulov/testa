
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
        options: Vec<(String, String)>
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
        tamplate: Option<Expression>,
        count: Expression,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Field {
    pub name: String,
    pub value: Vec<Expression>
} 

#[derive(PartialEq, Eq, Debug)]
pub struct ExpressionStatemnt {
    pub expression: Expression
}

#[derive(PartialEq, Eq, Debug)]
pub struct Expression {
    kind: ExpressionKind,
    start: usize,
    size: usize
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
    Identifier(String),
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
    LogicalNot,
    Negative,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum InfixOperator {
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug)]
pub enum Precedence {
    Lowest,
    Equality,   // == or !=
    Comparison, // <, <=, >, >=
    Sum,        // + or -
    Product,    // * or /
    Group,      // ( )
    Prefix,     // -X or !X
}


