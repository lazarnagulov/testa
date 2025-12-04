use super::Parser;
use crate::ast::{
    Expression, ExpressionKind, ExpressionStatemnt, InfixOperator, Precedence, PrefixOperator,
};
use crate::lexer::token::TokenKind::*;
use crate::parser::error::ParserError;
use crate::utils::Span;

macro_rules! parse_infix {
    ($self:expr, $expr:expr, $op:expr, $prec:expr) => {
        $self.parse_infix_expression($expr, $op, $prec)?
    };
}

impl<'src> Parser<'src> {
    pub(super) fn parse_expression_statement(&mut self) -> Result<ExpressionStatemnt, ParserError> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        let start = expression.span;
        let end = self.token_stream.expect_token(Semicolon)?;

        Ok(ExpressionStatemnt {
            expression,
            span: start.merge(end),
        })
    }

    pub(super) fn parse_list_expression(&mut self) -> Result<Expression, ParserError> {
        self.token_stream.consume_token()?;
        let elements = self.parse_list_elements()?;
        Ok(Expression::new(
            ExpressionKind::List(elements),
            Span::default(),
        ))
    }

    pub(super) fn parse_expression(
        &mut self,
        precedence: Precedence,
    ) -> Result<Expression, ParserError> {
        let mut expression = self.parse_primary_expression()?;

        while precedence < self.current_precendence() {
            use InfixOperator as Op;
            use Precedence as Prec;

            expression = match self.token_stream.peek_kind() {
                Asterisk => parse_infix!(self, expression, Op::Multiply, Prec::Product),
                Slash => parse_infix!(self, expression, Op::Divide, Prec::Product),
                Percent => parse_infix!(self, expression, Op::Mod, Prec::Product),
                Plus => parse_infix!(self, expression, Op::Plus, Prec::Sum),
                Minus => parse_infix!(self, expression, Op::Minus, Prec::Sum),
                BitAnd => parse_infix!(self, expression, Op::BitAnd, Prec::Bitwise),
                BitOr => parse_infix!(self, expression, Op::BitOr, Prec::Bitwise),
                BitXor => parse_infix!(self, expression, Op::BitXor, Prec::Bitwise),
                BitLShift => parse_infix!(self, expression, Op::BitLShift, Prec::Bitwise),
                BitRShift => parse_infix!(self, expression, Op::BitRShift, Prec::Bitwise),
                And => parse_infix!(self, expression, Op::And, Prec::Comparison),
                Or => parse_infix!(self, expression, Op::Or, Prec::Comparison),
                LessThan => parse_infix!(self, expression, Op::LessThan, Prec::Comparison),
                LessThanOrEqual => {
                    parse_infix!(self, expression, Op::LessThanOrEqual, Prec::Comparison)
                }
                GreaterThan => parse_infix!(self, expression, Op::GreaterThan, Prec::Comparison),
                GreaterThanOrEqual => {
                    parse_infix!(self, expression, Op::GreaterThanOrEqual, Prec::Comparison)
                }
                DoubleEqual => parse_infix!(self, expression, Op::Equal, Prec::Comparison),
                NotEqual => parse_infix!(self, expression, Op::NotEqual, Prec::Comparison),
                DoublePeriod => parse_infix!(self, expression, Op::ExclusiveRange, Prec::Range),
                DoublePeriodEqual => {
                    parse_infix!(self, expression, Op::InclusiveRange, Prec::Range)
                }
                token => {
                    return Err(ParserError::Syntax {
                        message: format!("Invalid operator: {}", token),
                        span: expression.span,
                    });
                }
            };
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

    pub(super) fn parse_literal(&mut self) -> Result<Expression, ParserError> {
        let token = self.token_stream.peek_token()?;
        let span = token.span;

        let parsed = match &token.kind {
            StringLiteral => {
                let literal = self.string_literal_content(span).to_string();
                Ok(Expression::new(
                    ExpressionKind::StringLiteral(literal),
                    span,
                ))
            }
            Identifier => {
                let literal = self.token_text(span).to_string();
                Ok(Expression::new(ExpressionKind::Identifier(literal), span))
            }
            FloatLiteral => {
                let literal = self.token_text(span).to_string();
                Ok(Expression::new(ExpressionKind::FloatLiteral(literal), span))
            }
            True => Ok(Expression::new(ExpressionKind::BooleanLiteral(true), span)),
            False => Ok(Expression::new(ExpressionKind::BooleanLiteral(false), span)),
            IntLiteral => {
                let number = self.token_text(span).parse().unwrap();
                Ok(Expression {
                    kind: ExpressionKind::IntLiteral(number),
                    span,
                })
            }
            kind => Err(ParserError::Expected {
                expected: "literal".to_owned(),
                got: kind.to_string(),
                span,
            }),
        };
        self.token_stream.next_token()?;
        parsed
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
            start_span.merge(end_span),
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
