#[allow(dead_code)]
use std::{iter::Peekable};

use crate::lexer::{lexer::Lexer, token::TokenKind::{self, *}};

use super::{ast::*, parser_error::ParserError};

// TODO: Add lookups for prefix and infix expressions { TokenKind: fn () }
pub struct Parser<'src> {
    lexer: Peekable<Lexer<'src>>,
    source: &'src str
}

impl<'src> Parser<'src> {

    pub fn new(program: &'src str) -> Self {
        let lexer = Lexer::new(program).peekable();
        Parser {
            lexer,
            source: program
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut statements = vec![];
        while self.lexer.peek().is_some() {
            let stmt = self.parse_statement()?;
            println!("{:?}", stmt);
            statements.push(stmt);
        }
        Ok(Program(statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.peek_kind() {
            Output => todo!(),
            Seed => todo!(),
            Template => self.parse_template(),
            Resource => todo!(),
            Enum => self.parse_enum(),
            Generate => self.parse_generate(),
            _ => Ok(Statement::Expression(self.parse_expression_statement()?))
        }
    }

    fn parse_template(&mut self) -> Result<Statement, ParserError> {
        self.lexer.next();
        let name = self.parse_identifier_as_string()?;
        let fields = self.parse_fields()?;
        Ok(Statement::Template { name, body: fields })
    }

    fn parse_generate(&mut self) -> Result<Statement, ParserError> {
        self.lexer.next();
        let name = self.parse_identifier_as_string()?;
        self.expect_token(LBracket)?;
        let count = self.parse_expression(Precedence::Lowest)?;
        self.expect_token(RBracket)?;
        if name == "_" {
            let fields = self.parse_fields()?;
            Ok(Statement::Generate { template_name: None, body: fields, count })
        } else {
            self.expect_token(Semicolon)?;
            Ok(Statement::Generate { template_name: Some(name), body: vec![], count })
        }
    }


    fn parse_enum(&mut self) -> Result<Statement, ParserError> {
        self.lexer.next();
        let name = self.parse_identifier_as_string()?;
        self.expect_token(LBrace)?;
        let variants = self.parse_parameters(RBrace)?;
        Ok(Statement::Enum { name, variants })
    }

    fn parse_parameters(&mut self, delimiter: TokenKind) -> Result<Vec<String>, ParserError> {
        let mut parameters = vec![];
        while self.peek_kind() == &Identifier {
            parameters.push(self.parse_identifier_as_string()?);
            if self.peek_kind() == &delimiter {
                break;
            }
            self.expect_token(Comma)?;
        }
        self.expect_token(delimiter)?;
        Ok(parameters)
    }

    fn parse_fields(&mut self) -> Result<Vec<Field>, ParserError> {
        self.expect_token(LBrace)?;
        let mut options = vec![];
        while self.peek_kind() == &Identifier {
            let name = self.parse_identifier_as_string()?;
            self.expect_token(SingleEqual)?;
            let expression = self.parse_expression(Precedence::Lowest)?;
            self.expect_token(Semicolon)?;
            options.push(Field::new(name, expression));
        }
        self.expect_token(RBrace)?;
        Ok(options)
    }

    fn parse_identifier_as_string(&mut self) -> Result<String, ParserError> {
        let (start, size) = self.expect_token(Identifier)?;
        Ok(self.source[start..start + size].to_string())
    }

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatemnt, ParserError> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.expect_token(Semicolon)?;

        Ok(ExpressionStatemnt { expression })
    }

    fn parse_expression(&mut self, precendence: Precedence) -> Result<Expression, ParserError> {
        let mut expression = self.parse_primary_expression()?;
        
        while precendence < self.current_precendence() {
            expression = match &self.peek_kind() {
                Asterisk => self.parse_infix_expression(expression, InfixOperator::Multiply,Precedence::Product)?,
                Slash => self.parse_infix_expression(expression, InfixOperator::Divide,Precedence::Product)?,
                Plus => self.parse_infix_expression(expression, InfixOperator::Plus, Precedence::Sum)?,
                Minus => self.parse_infix_expression(expression, InfixOperator::Minus, Precedence::Sum)?,
                BitAnd => self.parse_infix_expression(expression, InfixOperator::BitAnd, Precedence::Bitwise)?,
                BitOr => self.parse_infix_expression(expression, InfixOperator::BitOr, Precedence::Bitwise)?,
                BitXor => self.parse_infix_expression(expression, InfixOperator::BitXor, Precedence::Bitwise)?,
                BitLShift => self.parse_infix_expression(expression, InfixOperator::BitLShift, Precedence::Bitwise)?,
                BitRShift => self.parse_infix_expression(expression, InfixOperator::BitRShift, Precedence::Bitwise)?,
                And => self.parse_infix_expression(expression, InfixOperator::And, Precedence::Comparison)?,
                Or => self.parse_infix_expression(expression, InfixOperator::Or, Precedence::Comparison)?,
                LessThan => self.parse_infix_expression(expression, InfixOperator::LessThan, Precedence::Comparison)?,
                LessThanOrEqual => self.parse_infix_expression(expression, InfixOperator::LessThanOrEqual, Precedence::Comparison)?,
                GreaterThan => self.parse_infix_expression(expression, InfixOperator::GreaterThan, Precedence::Comparison)?,
                GreaterThanOrEqual => self.parse_infix_expression(expression, InfixOperator::GreaterThanOrEqual, Precedence::Comparison)?,
                DoubleEqual => self.parse_infix_expression(expression, InfixOperator::Equal, Precedence::Comparison)?,
                NotEqual => self.parse_infix_expression(expression, InfixOperator::NotEqual, Precedence::Comparison)?,
                DoublePeriod => self.parse_infix_expression(expression, InfixOperator::ExclusiveRange, Precedence::Range)?,
                DoublePeriodEqual => self.parse_infix_expression(expression, InfixOperator::InclusiveRange, Precedence::Range)?,
                token => return Err(ParserError::syntax_err(&format!("Invalid operator: {}", token)))
            }
        }
        Ok(expression)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, ParserError> {
        match &self.peek_kind()  {
            Int | Float | Str => Ok(self.parse_type()?),
            Identifier => Ok(self.parse_identifier()?),
            IntLiteral => Ok(self.parse_int_literal()?),
            StringLiteral => Ok(self.parse_string_literal()?),
            True | False => Ok(self.parse_bool_literal()?),
            BitNegate => Ok(self.parse_prefix_expression(PrefixOperator::BitNegate)?),
            ExclamationMark => Ok(self.parse_prefix_expression(PrefixOperator::LogicalNegate)?),
            Minus => Ok(self.parse_prefix_expression(PrefixOperator::Negative)?),
            LParen => Ok(self.parse_group_expression()?),
            kind => Err(ParserError::syntax_err(&format!("Invalid primary expression: {}", kind)))
        }
    }

    fn parse_type(&mut self) -> Result<Expression, ParserError> {
        let token = self.lexer.peek().ok_or(ParserError::UnexpectedEOF)?;
        let start = token.start;
        let size = token.size;
        match &token.kind {
            Int | Float | Str => {
                self.lexer.next();
                let literal = self.source[start..start+size].to_string();
                Ok(Expression::new(ExpressionKind::Type(literal), start, size))
            },
            kind => Err(ParserError::expected("type", &kind.to_string()))
        }
    }

    fn parse_identifier(&mut self) -> Result<Expression, ParserError> {
        let token = self.lexer.peek().ok_or(ParserError::UnexpectedEOF)?;
        let start = token.start;
        let size = token.size;
        match &token.kind {
            Identifier => {
                self.lexer.next();
                let literal = self.source[start+1..start+size-1].to_string();
                Ok(Expression::new(ExpressionKind::Identifier(literal), start, size))
            },
            kind => Err(ParserError::expected("string literal", &kind.to_string()))
        }

    }

    fn parse_string_literal(&mut self) -> Result<Expression, ParserError> {
        let token = self.lexer.peek().ok_or(ParserError::UnexpectedEOF)?;
        let start = token.start;
        let size = token.size;
        match &token.kind {
            StringLiteral => {
                self.lexer.next();
                let literal = self.source[start+1..start+size-1].to_string();
                Ok(Expression::new(ExpressionKind::StringLiteral(literal), start, size))
            },
            kind => Err(ParserError::expected("string literal", &kind.to_string()))
        }
    }

    fn parse_bool_literal(&mut self) -> Result<Expression, ParserError> {
        let token = self.lexer.peek().ok_or(ParserError::UnexpectedEOF)?;
        let start = token.start;
        let size = token.size;

        match &token.kind {
            True => {
                self.lexer.next();
                Ok(Expression::new(ExpressionKind::BooleanLiteral(true), start, size))
            }
            False => {
                self.lexer.next();
                Ok(Expression::new(ExpressionKind::BooleanLiteral(false), start, size))
            }
            kind => Err(ParserError::expected("boolean literal", &kind.to_string()))
        }

    }

    fn parse_int_literal(&mut self) -> Result<Expression, ParserError> {
        let token = self.lexer.peek().ok_or(ParserError::UnexpectedEOF)?;
        let start = token.start;
        let size = token.size;
        if token.kind == IntLiteral {
            self.lexer.next();
            let number = (&self.source[start..start+size]).parse().unwrap();
            Ok(Expression { kind: ExpressionKind::IntLiteral(number), start, size })
        } else {
            Err(ParserError::Expected { expected: "int litral".to_string(), got: token.kind.to_string()})
        }
    }

    fn parse_group_expression(&mut self) -> Result<Expression, ParserError> {
        let start = self.expect_token(LParen).unwrap().0;
        let expression = self.parse_expression(Precedence::Lowest)?;
        match self.peek_kind() {
            RParen => {
                let end = self.expect_token(RParen).unwrap().0;
                Ok(Expression::new(expression.kind, start, (end + 1) - start))
            }
            kind => Err(ParserError::Expected { expected: ")".to_string(), got: kind.to_string()})
        }
    }

    fn parse_prefix_expression(&mut self, operator: PrefixOperator) -> Result<Expression, ParserError> {
        let (start, size) = self.expect_token(match operator {
            PrefixOperator::LogicalNegate => ExclamationMark,
            PrefixOperator::Negative => Minus,
            PrefixOperator::BitNegate => BitNegate,
        })?;
        let expression = self.parse_expression(Precedence::Prefix)?;
        let size = size + expression.size;
        Ok(Expression::new(ExpressionKind::Prefix { operator, expression: Box::new(expression) }, start, size))
    }

    fn parse_infix_expression(&mut self, left: Expression, operator: InfixOperator, precendence: Precedence) -> Result<Expression, ParserError> {
        self.lexer.next();
        let right= self.parse_expression(precendence)?;
        let start = left.start;
        let end = right.start + right.size; 

        Ok(Expression::new(ExpressionKind::Infix {
            left: Box::new(left), 
            operator, 
            right: Box::new(right)}, 
            start,
            end - start
        ))
    }

    fn expect_token(&mut self, kind: TokenKind) -> Result<(usize, usize), ParserError> {
        let token = self.lexer.next().ok_or(ParserError::UnexpectedEOF)?;
        if token.kind != kind {
            Err(ParserError::syntax_err(&format!("Expected {} but got {}", kind, token.kind)))
        } else {
            Ok((token.start, token.size))
        }
    }

    fn peek_kind(&mut self) -> &TokenKind {
        self.lexer.peek().map_or(&Eof, |t| &t.kind)
    }

    fn current_precendence(&mut self) -> Precedence {
        match self.peek_kind() {
            DoubleEqual | NotEqual => Precedence::Equality,
            DoublePeriod | DoublePeriodEqual => Precedence::Range,
            LessThan | GreaterThan | LessThanOrEqual | GreaterThanOrEqual | And | Or => Precedence::Comparison,
            BitAnd | BitOr | BitXor | BitLShift | BitRShift => Precedence::Bitwise,
            Plus | Minus => Precedence::Sum,
            Asterisk | Slash => Precedence::Product,
            LParen => Precedence::Group,
            _ => Precedence::Lowest
        }
    }

}