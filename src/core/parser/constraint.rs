use crate::core::{
    ast::{ConstraintExpression, ConstraintKind, Precedence},
    lexer::token::TokenKind::*,
    parser::error::ParserError,
};

use super::Parser;

impl<'src> Parser<'src> {
    pub(super) fn parse_constraints(&mut self) -> Result<Vec<ConstraintExpression>, ParserError> {
        let span = self.token_stream.consume_token()?;
        let mut constraints: Vec<ConstraintExpression> = vec![];

        while self.token_stream.peek_kind() == &Identifier {
            let identifier = self.parse_identifier_as_string()?;
            let constraint = match identifier.as_str() {
                "range" => self.parse_constraint_expression(ConstraintKind::Range),
                "multiple_of" => self.parse_constraint_expression(ConstraintKind::MultipleOf),
                "length" => self.parse_constraint_expression(ConstraintKind::Length),
                "bias" => self.parse_constraint_expression(ConstraintKind::Bias),
                "min" => self.parse_constraint_expression(ConstraintKind::Min),
                "max" => self.parse_constraint_expression(ConstraintKind::Max),
                "count" => self.parse_constraint_expression(ConstraintKind::Count),
                _ => {
                    if self.token_stream.peek_kind() == &SingleEqual {
                        return Err(ParserError::UndefinedConstraint { span });
                    }
                    let expression = self.parse_expression(Precedence::Lowest)?;
                    let span = expression.span;
                    Ok(ConstraintExpression::new(
                        expression,
                        ConstraintKind::Custom,
                        span,
                    ))
                }
            }?;
            constraints.push(constraint);
            if self.token_stream.peek_kind() == &RBracket {
                break;
            }
            self.token_stream.expect_token(Comma)?;
        }
        self.token_stream.expect_token(RBracket)?;
        Ok(constraints)
    }

    fn parse_constraint_expression(
        &mut self,
        constraint_kind: ConstraintKind,
    ) -> Result<ConstraintExpression, ParserError> {
        self.token_stream.expect_token(SingleEqual)?;
        let expression = self.parse_expression(Precedence::Lowest)?;
        let span = expression.span;
        Ok(ConstraintExpression::new(expression, constraint_kind, span))
    }
}
