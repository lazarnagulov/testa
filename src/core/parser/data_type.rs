use crate::core::{
    ast::{DataType, DataTypeKind, Expression, ExpressionKind},
    lexer::token::TokenKind::*,
    parser::error::ParserError,
    utils::span::Span,
};

use super::Parser;

impl<'src> Parser<'src> {
    pub(super) fn parse_list_type(&mut self) -> Result<Expression, ParserError> {
        self.token_stream.consume_token()?;
        let data_type = self.parse_type()?;
        let ExpressionKind::Type(data_type) = data_type.kind else {
            unreachable!()
        };
        self.token_stream.expect_token(RBracket)?;
        if self.token_stream.peek_kind() == &LBracket {
            let consraints = self.parse_constraints()?;
            Ok(Expression::new(
                ExpressionKind::Type(DataType::new(
                    DataTypeKind::List(Box::new(data_type)),
                    Some(consraints),
                    Span::default(),
                )),
                Span::default(),
            ))
        } else {
            Ok(Expression::new(
                ExpressionKind::Type(DataType::new(
                    DataTypeKind::List(Box::new(data_type)),
                    None,
                    Span::default(),
                )),
                Span::default(),
            ))
        }
    }

    pub(super) fn parse_type(&mut self) -> Result<Expression, ParserError> {
        let token = self.token_stream.peek_token()?;
        let span = token.span;

        if token.kind == LBracket {
            return self.parse_list_type();
        }

        let data_type_kind = match token.kind {
            Int => DataTypeKind::Int,
            Float => DataTypeKind::Float,
            Str => DataTypeKind::Str,
            Bool => DataTypeKind::Boolean,
            Extend => {
                self.token_stream.consume_token()?;
                let name = self.parse_identifier_as_string()?;
                let peek = self.token_stream.peek_kind();
                if peek != &With {
                    return Err(ParserError::Expected {
                        expected: "with".to_owned(),
                        got: format!("{}", *peek),
                        span,
                    });
                }
                DataTypeKind::Custom(name)
            }
            Identifier => DataTypeKind::Custom(self.parse_peeked_token_as_string()?),
            _ => unreachable!("has to be a type"),
        };

        self.token_stream.next_token()?;
        if self.token_stream.peek_kind() == &LBracket {
            let constraints = self.parse_constraints()?;
            // TODO: calculate start and size
            Ok(Expression::new(
                ExpressionKind::Type(DataType::new(
                    data_type_kind,
                    Some(constraints),
                    Span::default(),
                )),
                //TODO: add constraint size
                span,
            ))
        } else {
            Ok(Expression::new(
                ExpressionKind::Type(DataType::new(data_type_kind, None, span)),
                span,
            ))
        }
    }
}
