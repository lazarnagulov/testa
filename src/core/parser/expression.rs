use crate::core::ast::{Expression, ExpressionKind, ExpressionStatemnt, InfixOperator, Precedence, PrefixOperator};
use crate::core::lexer::token::TokenKind::*;
use crate::core::parser::error::ParserError;
use crate::core::utils::span::Span;
use super::Parser;


impl<'src> Parser<'src> {

    pub(super) fn parse_expression_statement(&mut self) -> Result<ExpressionStatemnt, ParserError> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        let span = self.token_stream.expect_token(Semicolon)?;

        Ok(ExpressionStatemnt { expression, span })
    }

    pub(super) fn parse_list_expression(&mut self) -> Result<Expression, ParserError> {
        self.token_stream.consume_token()?;
        let elements = self.parse_elements()?;
        Ok(Expression::new(ExpressionKind::List(elements), Span::default()))
    }

    pub(super) fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        let mut expression = self.parse_primary_expression()?;

        while precedence < self.current_precendence() {
            expression = match &self.token_stream.peek_kind() {
                Asterisk => self.parse_infix_expression(
                    expression,
                    InfixOperator::Multiply,
                    Precedence::Product,
                )?,
                Slash => self.parse_infix_expression(
                    expression,
                    InfixOperator::Divide,
                    Precedence::Product,
                )?,
                Percent => self.parse_infix_expression(
                    expression,
                    InfixOperator::Mod,
                    Precedence::Product,
                )?,
                Plus => {
                    self.parse_infix_expression(expression, InfixOperator::Plus, Precedence::Sum)?
                }
                Minus => {
                    self.parse_infix_expression(expression, InfixOperator::Minus, Precedence::Sum)?
                }
                BitAnd => self.parse_infix_expression(
                    expression,
                    InfixOperator::BitAnd,
                    Precedence::Bitwise,
                )?,
                BitOr => self.parse_infix_expression(
                    expression,
                    InfixOperator::BitOr,
                    Precedence::Bitwise,
                )?,
                BitXor => self.parse_infix_expression(
                    expression,
                    InfixOperator::BitXor,
                    Precedence::Bitwise,
                )?,
                BitLShift => self.parse_infix_expression(
                    expression,
                    InfixOperator::BitLShift,
                    Precedence::Bitwise,
                )?,
                BitRShift => self.parse_infix_expression(
                    expression,
                    InfixOperator::BitRShift,
                    Precedence::Bitwise,
                )?,
                And => self.parse_infix_expression(
                    expression,
                    InfixOperator::And,
                    Precedence::Comparison,
                )?,
                Or => self.parse_infix_expression(
                    expression,
                    InfixOperator::Or,
                    Precedence::Comparison,
                )?,
                LessThan => self.parse_infix_expression(
                    expression,
                    InfixOperator::LessThan,
                    Precedence::Comparison,
                )?,
                LessThanOrEqual => self.parse_infix_expression(
                    expression,
                    InfixOperator::LessThanOrEqual,
                    Precedence::Comparison,
                )?,
                GreaterThan => self.parse_infix_expression(
                    expression,
                    InfixOperator::GreaterThan,
                    Precedence::Comparison,
                )?,
                GreaterThanOrEqual => self.parse_infix_expression(
                    expression,
                    InfixOperator::GreaterThanOrEqual,
                    Precedence::Comparison,
                )?,
                DoubleEqual => self.parse_infix_expression(
                    expression,
                    InfixOperator::Equal,
                    Precedence::Comparison,
                )?,
                NotEqual => self.parse_infix_expression(
                    expression,
                    InfixOperator::NotEqual,
                    Precedence::Comparison,
                )?,
                DoublePeriod => self.parse_infix_expression(
                    expression,
                    InfixOperator::ExclusiveRange,
                    Precedence::Range,
                )?,
                DoublePeriodEqual => self.parse_infix_expression(
                    expression,
                    InfixOperator::InclusiveRange,
                    Precedence::Range,
                )?,
                token => {
                    return Err(ParserError::Syntax {
                        message: format!("Invalid operator: {}", token),
                        span: expression.span,
                    });
                }
            }
        }
        Ok(expression)
    }

    pub(super) fn parse_primary_expression(&mut self) -> Result<Expression, ParserError> {
        match &self.token_stream.peek_kind() {
            Int | Float | Str | Bool => Ok(self.parse_type()?),
            LBracket => {
                let kind = self.token_stream.peek_kind_n(2);
                match kind {
                    Int | Float | Str | Bool | LBracket | Identifier => self.parse_type(),
                    True | False | IntLiteral | StringLiteral | FloatLiteral => {
                        self.parse_list_expression()
                    }
                    obj => Err(ParserError::Expected {
                        expected: "data type or literal".to_owned(),
                        got: format!("{}", obj),
                        // TODO: think about how to get span
                        span: Span::default(),
                    }),
                }
            }
            Identifier | True | False | IntLiteral | StringLiteral | FloatLiteral => {
                Ok(self.parse_literal()?)
            }
            StringPattern => Ok(self.parse_string_pattern()?),
            BitNegate => Ok(self.parse_prefix_expression(PrefixOperator::BitNegate)?),
            ExclamationMark => Ok(self.parse_prefix_expression(PrefixOperator::LogicalNegate)?),
            Minus => Ok(self.parse_prefix_expression(PrefixOperator::Negative)?),
            LParen => Ok(self.parse_group_expression()?),
            kind => Err(ParserError::Syntax {
                message: format!("Invalid primary expression: {}", kind),
                span: Span::default(),
            }),
        }
    }

    pub(super) fn parse_group_expression(&mut self) -> Result<Expression, ParserError> {
        let start_span = self.token_stream.consume_token()?;
        let expression = self.parse_expression(Precedence::Lowest)?;
        match self.token_stream.peek_kind() {
            RParen => {
                let end_span = self.token_stream.expect_token(RParen).unwrap();
                Ok(Expression::new(expression.kind, start_span.merge(end_span)))
            }
            kind => Err(ParserError::Expected {
                expected: ")".to_string(),
                got: kind.to_string(),
                span: start_span,
            }),
        }
    }

    pub(super) fn parse_prefix_expression(
        &mut self,
        operator: PrefixOperator,
    ) -> Result<Expression, ParserError> {
        let op_span = self.token_stream.consume_token()?;  
        let expression = self.parse_expression(Precedence::Prefix)?;
        let span = op_span.merge(expression.span); 
        Ok(Expression::new(
            ExpressionKind::Prefix {
                operator,
                expression: Box::new(expression),
            },
            span,
        ))
    }

    pub(super) fn parse_infix_expression(
        &mut self,
        left: Expression,
        operator: InfixOperator,
        precedence: Precedence,
    ) -> Result<Expression, ParserError> {
        self.token_stream.next_token()?;        
        let right = self.parse_expression(precedence)?;

        let start_span = left.span;
        let end_span = right.span;

        Ok(Expression::new(
            ExpressionKind::Infix {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            },
            start_span.merge(end_span)
        ))
    }

    fn current_precendence(&mut self) -> Precedence {
        match self.token_stream.peek_kind() {
            DoubleEqual | NotEqual => Precedence::Equality,
            DoublePeriod | DoublePeriodEqual => Precedence::Range,
            LessThan | GreaterThan | LessThanOrEqual | GreaterThanOrEqual | And | Or => {
                Precedence::Comparison
            }
            BitAnd | BitOr | BitXor | BitLShift | BitRShift => Precedence::Bitwise,
            Plus | Minus => Precedence::Sum,
            Asterisk | Slash => Precedence::Product,
            LParen => Precedence::Group,
            _ => Precedence::Lowest,
        }
    }

}