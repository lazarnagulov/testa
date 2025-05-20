#[allow(dead_code)]
use std::{iter::Peekable};

use crate::lexer::{lexer::Lexer, token::TokenKind::{self, *}};

use super::{ast::*, parser_error::ParserError};

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
            statements.push(stmt);
        }
        Ok(Program(statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.peek_kind() {
            Output => self.parse_output_directive(),
            Seed => todo!(),
            Template => todo!(),
            Resource => todo!(),
            Enum => self.parse_enum(),
            Generate => self.parse_generate(),
            _ => Ok(Statement::Expression(self.parse_expression_statement()?))
        }
    }

    fn parse_output_directive(&mut self) -> Result<Statement, ParserError> {
        self.lexer.next();
        let (start, size) = self.expect_token(Identifier)?;
        let mut options = vec![];
        
        if self.peek_kind() == &LBrace {
            self.lexer.next();
            options = self.parse_option_fields()?;
            self.expect_token(RBrace)?;
        } else {
            self.expect_token(Semicolon)?;
        }

        Ok(Statement::OutputDirective { argument: self.source[start..start + size].to_string(), options: options })
    }

    fn parse_enum(&mut self) -> Result<Statement, ParserError> {
        self.lexer.next();
        let (start, size) = self.expect_token(Identifier)?;
        let name = self.source[start..start + size].to_string();
        self.expect_token(LBrace)?;
        let variants = self.parse_parameters()?;
        Ok(Statement::Enum { name, variants })
    }

    fn parse_parameters(&mut self) -> Result<Vec<String>, ParserError> {
        let mut parameters = vec![];
        while self.peek_kind() == &Identifier {
            let (start, size) = self.expect_token(Identifier).unwrap();
            parameters.push(self.source[start..start + size].to_string());
            if self.peek_kind() == &RBrace {
                break;
            }
            self.expect_token(Comma)?;
        }
        self.lexer.next();
        Ok(parameters)
    }

    fn parse_option_fields(&mut self) -> Result<Vec<(String, String)>, ParserError> {
        let mut options = vec![];
        while self.peek_kind() == &Identifier {
            let (start, size) = self.expect_token(Identifier).unwrap();
            let key = self.source[start..start + size].to_string();
            self.expect_token(SingleEqual)?;
            let (start, size) = self.expect_token(StringLiteral)?;
            let value = self.source[start + 1..start + size - 1].to_string();
            self.expect_token(Semicolon)?;
            options.push((key, value));
        }
        Ok(options)
    }

    fn parse_generate(&mut self) -> Result<Statement, ParserError> {
        todo!()
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
                BitAnd => self.parse_infix_expression(expression, InfixOperator::BitAnd, Precedence::Lowest)?,
                BitOr => self.parse_infix_expression(expression, InfixOperator::BitOr, Precedence::Lowest)?,
                BitXor => self.parse_infix_expression(expression, InfixOperator::BitXor, Precedence::Lowest)?,
                LessThan => self.parse_infix_expression(expression, InfixOperator::LessThan, Precedence::Comparison)?,
                LessThanOrEqual => self.parse_infix_expression(expression, InfixOperator::LessThanOrEqual, Precedence::Comparison)?,
                GreaterThan => self.parse_infix_expression(expression, InfixOperator::GreaterThan, Precedence::Comparison)?,
                GreaterThanOrEqual => self.parse_infix_expression(expression, InfixOperator::GreaterThanOrEqual, Precedence::Comparison)?,
                DoubleEqual => self.parse_infix_expression(expression, InfixOperator::Equal, Precedence::Comparison)?,
                NotEqual => self.parse_infix_expression(expression, InfixOperator::NotEqual, Precedence::Comparison)?,
                token => return Err(ParserError::syntax_err(&format!("Invalid operator: {}", token)))
            }
        }
        Ok(expression)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, ParserError> {
        match &self.peek_kind()  {
            IntLiteral => Ok(self.parse_int_literal()?),
            True | False => Ok(self.parse_bool_literal()?),
            ExclamationMark => Ok(self.parse_prefix_expression(PrefixOperator::LogicalNot)?),
            Minus => Ok(self.parse_prefix_expression(PrefixOperator::Negative)?),
            LParen => Ok(self.parse_group_expression()?),
            _ => Err(ParserError::syntax_err("Invalid prefix expression"))
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
            PrefixOperator::LogicalNot => ExclamationMark,
            PrefixOperator::Negative => Minus,
        })?;
        let expression = self.parse_expression(Precedence::Prefix)?;
        let size = size + expression.size;
        Ok(Expression::new(ExpressionKind::Prefix { operator, expression: Box::new(expression) }, start, size))
    }

    fn parse_infix_expression(&mut self, left: Expression, operator: InfixOperator, precendence: Precedence) -> Result<Expression, ParserError> {
        println!("Op: {:?}", operator);
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
            Err(ParserError::syntax_err(&format!("Error: Expected {} but got {}", kind, token.kind)))
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
            LessThan | GreaterThan | LessThanOrEqual | GreaterThanOrEqual => Precedence::Comparison,
            BitAnd | BitOr | BitXor => Precedence::Bitwise,
            Plus | Minus => Precedence::Sum,
            Asterisk | Slash => Precedence::Product,
            LParen => Precedence::Group,
            _ => Precedence::Lowest
        }
    }

}