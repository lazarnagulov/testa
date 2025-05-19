#[allow(dead_code)]
use std::{iter::Peekable};

use crate::lexer::{lexer::Lexer, token::TokenKind};

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
            println!("{:?}", stmt);
            statements.push(stmt);
        }
        Ok(Program(statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.peek_kind() {
            TokenKind::Output => self.parse_output_directive(),
            TokenKind::Seed => todo!(),
            TokenKind::Template => todo!(),
            TokenKind::Resource => todo!(),
            TokenKind::Enum => self.parse_enum(),
            TokenKind::Generate => self.parse_generate(),
            _ => Ok(Statement::Expression(self.parse_expression_statement()?))
        }
    }

    fn parse_output_directive(&mut self) -> Result<Statement, ParserError> {
        self.lexer.next();
        let (start, size) = self.expect_token(TokenKind::Identifier)?;
        let mut options = vec![];
        
        if self.peek_kind() == &TokenKind::LBrace {
            self.lexer.next();
            options = self.parse_option_fields()?;
            self.expect_token(TokenKind::RBrace)?;
        } else {
            self.expect_token(TokenKind::Semicolon)?;
        }

        Ok(Statement::OutputDirective { argument: self.source[start..start + size].to_string(), options: options })
    }

    fn parse_enum(&mut self) -> Result<Statement, ParserError> {
        self.lexer.next();
        let (start, size) = self.expect_token(TokenKind::Identifier)?;
        let name = self.source[start..start + size].to_string();
        self.expect_token(TokenKind::LBrace)?;
        let variants = self.parse_parameters()?;
        Ok(Statement::Enum { name, variants })
    }

    fn parse_parameters(&mut self) -> Result<Vec<String>, ParserError> {
        let mut parameters = vec![];
        while self.peek_kind() == &TokenKind::Identifier {
            let (start, size) = self.expect_token(TokenKind::Identifier).unwrap();
            parameters.push(self.source[start..start + size].to_string());
            if self.peek_kind() == &TokenKind::RBrace {
                break;
            }
            self.expect_token(TokenKind::Comma)?;
        }
        self.lexer.next();
        Ok(parameters)
    }

    fn parse_option_fields(&mut self) -> Result<Vec<(String, String)>, ParserError> {
        let mut options = vec![];
        while self.peek_kind() == &TokenKind::Identifier {
            let (start, size) = self.expect_token(TokenKind::Identifier).unwrap();
            let key = self.source[start..start + size].to_string();
            self.expect_token(TokenKind::SingleEqual)?;
            let (start, size) = self.expect_token(TokenKind::StringLiteral)?;
            let value = self.source[start + 1..start + size - 1].to_string();
            self.expect_token(TokenKind::Semicolon)?;
            options.push((key, value));
        }
        Ok(options)
    }

    fn parse_generate(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatemnt, ParserError> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.expect_token(TokenKind::Semicolon)?;

        Ok(ExpressionStatemnt { expression })
    }

    fn parse_expression(&mut self, precendence: Precedence) -> Result<Expression, ParserError> {
        let mut expression = self.parse_expression_by_prefix()?;
        
        while precendence < self.current_precendence() {
            expression = match &self.peek_kind() {
                TokenKind::Asterisk => self.parse_infix_expression(expression, InfixOperator::Multiply,Precedence::Product)?,
                TokenKind::Slash => self.parse_infix_expression(expression, InfixOperator::Divide,Precedence::Product)?,
                TokenKind::Plus => self.parse_infix_expression(expression, InfixOperator::Plus, Precedence::Sum)?,
                TokenKind::Minus => self.parse_infix_expression(expression, InfixOperator::Minus, Precedence::Sum)?,
                TokenKind::BitAnd => self.parse_infix_expression(expression, InfixOperator::BitAnd, Precedence::Lowest)?,
                TokenKind::BitOr => self.parse_infix_expression(expression, InfixOperator::BitOr, Precedence::Lowest)?,
                TokenKind::BitXor => self.parse_infix_expression(expression, InfixOperator::BitXor, Precedence::Lowest)?,
                TokenKind::LessThan => self.parse_infix_expression(expression, InfixOperator::LessThan, Precedence::Comparison)?,
                TokenKind::LessThanOrEqual => self.parse_infix_expression(expression, InfixOperator::LessThanOrEqual, Precedence::Comparison)?,
                TokenKind::GreaterThan => self.parse_infix_expression(expression, InfixOperator::GreaterThan, Precedence::Comparison)?,
                TokenKind::GreaterThanOrEqual => self.parse_infix_expression(expression, InfixOperator::GreaterThanOrEqual, Precedence::Comparison)?,
                TokenKind::DoubleEqual => self.parse_infix_expression(expression, InfixOperator::Equal, Precedence::Comparison)?,
                TokenKind::NotEqual => self.parse_infix_expression(expression, InfixOperator::NotEqual, Precedence::Comparison)?,
                token => return Err(ParserError::syntax_err(&format!("Invalid operator: {}", token)))
            }
        }
        Ok(expression)
    }

    fn parse_expression_by_prefix(&mut self) -> Result<Expression, ParserError> {
        match &self.peek_kind()  {
            TokenKind::IntLiteral => Ok(self.parse_int_literal()?),
            TokenKind::ExclamationMark => Ok(self.parse_prefix_expression(PrefixOperator::LogicalNot)?),
            TokenKind::Minus => Ok(self.parse_prefix_expression(PrefixOperator::Negative)?),
            _ => Err(ParserError::syntax_err("Invalid prefix expression"))
        }
    }

    fn parse_int_literal(&mut self) -> Result<Expression, ParserError> {
        let token = self.lexer.peek().ok_or(ParserError::UnexpectedEOF)?;
        let start = token.start;
        let size = token.size;
        if token.kind == TokenKind::IntLiteral {
            self.lexer.next();
            let number = (&self.source[start..start+size]).parse().unwrap();
            Ok(Expression { kind: ExpressionKind::IntLiteral(number), start, size })
        } else {
            Err(ParserError::Expected { expected: "int litral".to_string(), got: token.kind.to_string()})
        }
    }


    fn parse_prefix_expression(&mut self, operator: PrefixOperator) -> Result<Expression, ParserError> {
        let (start, size) = self.expect_token(match operator {
            PrefixOperator::LogicalNot => TokenKind::ExclamationMark,
            PrefixOperator::Negative => TokenKind::Minus,
        })?;
        let expression = self.parse_expression(Precedence::Prefix)?;
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
            Err(ParserError::Syntax(format!("Error: Expected {} but got {}", kind, token.kind)))
        } else {
            Ok((token.start, token.size))
        }
    }

    fn peek_kind(&mut self) -> &TokenKind {
        self.lexer.peek().map_or(&TokenKind::Eof, |t| &t.kind)
    }

    fn current_precendence(&mut self) -> Precedence {
        match self.peek_kind() {
            TokenKind::DoubleEqual | TokenKind::NotEqual => Precedence::Equality,
            TokenKind::LessThan | TokenKind::GreaterThan | TokenKind::LessThanOrEqual | TokenKind::GreaterThanOrEqual => Precedence::Comparison,
            TokenKind::Plus | TokenKind::Minus => Precedence::Sum,
            TokenKind::Asterisk | TokenKind::Slash => Precedence::Product,
            TokenKind::LParen => Precedence::Group,
            _ => Precedence::Lowest
        }
    }

}