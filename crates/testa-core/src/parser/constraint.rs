use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{
    ast::{ConstraintExpression, ConstraintKind, Precedence},
    lexer::token::TokenKind::*,
    parser::error::ParserError,
};

use super::Parser;

static CONSTRAINTS: Lazy<HashMap<&'static str, ConstraintKind>> = Lazy::new(|| {
    use ConstraintKind::*;
    let mut m = HashMap::new();
    m.insert("range", Range);
    m.insert("multiple_of", MultipleOf);
    m.insert("length", Length);
    m.insert("bias", Bias);
    m.insert("min", Min);
    m.insert("max", Max);
    m.insert("count", Count);
    m
});

impl<'src> Parser<'src> {
    pub(super) fn parse_constraints(&mut self) -> Result<Vec<ConstraintExpression>, ParserError> {
        let start = self.token_stream.consume_token()?;
        let mut constraints = Vec::new();

        while self.token_stream.peek_kind() == &Identifier {
            let identifier = self.parse_identifier_as_string()?;

            let constraint = if let Some(kind) = CONSTRAINTS.get(identifier.as_str()) {
                self.parse_constraint_expression(kind.clone())?
            } else {
                if self.token_stream.peek_kind() == &SingleEqual {
                    return Err(ParserError::UndefinedConstraint {
                        span: self.token_stream.last_span(),
                    });
                }
                let expr = self.parse_expression(Precedence::Lowest)?;
                let span = start.merge(expr.span);
                ConstraintExpression::new(expr, ConstraintKind::Custom, span)
            };

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
        let start = self.token_stream.expect_token(SingleEqual)?;
        let expression = self.parse_expression(Precedence::Lowest)?;
        let end = expression.span;
        Ok(ConstraintExpression::new(
            expression,
            constraint_kind,
            start.merge(end),
        ))
    }
}
