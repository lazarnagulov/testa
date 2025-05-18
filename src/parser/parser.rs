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
        todo!()
    }

    fn parse_expression(&mut self, _precendence: Precedence) -> Result<Expression, ParserError> {
        todo!()
    }

    fn parse_prefix_expression(&mut self, operator: PrefixOperator) -> Result<Expression, ParserError> {
        let (start, size) = self.expect_token(match operator {
            PrefixOperator::LogicalNot => TokenKind::ExclamationMark,
            PrefixOperator::Negative => TokenKind::Minus,
        })?;
        let expression = self.parse_expression(Precedence::Prefix)?;
        Ok(Expression::new(ExpressionKind::Prefix { operator, expression: Box::new(expression) }, start, size))
    }

    fn parse_infix_expression(&mut self, _operator: InfixOperator) -> Result<Expression, ParserError> {
        todo!()
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

    fn get_current_precendance(&mut self) -> Precedence {
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