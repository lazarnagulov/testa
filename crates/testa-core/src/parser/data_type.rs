use crate::{
    ast::{DataType, DataTypeKind, Expression, ExpressionKind},
    lexer::{
        error::LexerError,
        token::{Token, TokenKind::*},
    },
    parser::error::ParserError,
};

use super::Parser;

impl<'src, I> Parser<'src, I>
where
    I: Iterator<Item = Result<Token, LexerError>>,
{
    pub(super) fn parse_list_type(&mut self) -> Result<Expression, ParserError> {
        let start_span = self.token_stream.consume_token()?;

        let data_type = self.parse_type()?;
        let ExpressionKind::Type(data_type) = data_type.kind else {
            unreachable!()
        };

        let rbracket_span = self.token_stream.expect_token(RBracket)?;

        if self.token_stream.peek_kind() == &LBracket {
            let constraints = self.parse_constraints()?;
            let end_span = self.token_stream.last_span();
            let full_span = start_span.merge(end_span);

            Ok(Expression::new(
                ExpressionKind::Type(DataType::new(
                    DataTypeKind::List(Box::new(data_type)),
                    Some(constraints),
                    full_span,
                )),
                full_span,
            ))
        } else {
            let full_span = start_span.merge(rbracket_span);

            Ok(Expression::new(
                ExpressionKind::Type(DataType::new(
                    DataTypeKind::List(Box::new(data_type)),
                    None,
                    full_span,
                )),
                full_span,
            ))
        }
    }

    pub(super) fn parse_type(&mut self) -> Result<Expression, ParserError> {
        let token = self.token_stream.peek_token()?;
        let start_span = token.span;

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
                        span: self.token_stream.peek_token()?.span,
                    });
                }
                DataTypeKind::Custom(name)
            }
            Identifier => DataTypeKind::Custom(self.parse_peeked_token_as_string()?),
            StringPattern => {
                return self.parse_string_pattern();
            }
            ref kind => {
                return Err(ParserError::Expected {
                    expected: "data type or string pattern".to_string(),
                    got: kind.to_string(),
                    span: start_span,
                });
            }
        };

        self.token_stream.next_token()?;
        let mut end_span = self.token_stream.last_span();

        if self.token_stream.peek_kind() == &LBracket {
            let constraints = self.parse_constraints()?;
            end_span = self.token_stream.last_span();
            let full_span = start_span.merge(end_span);

            Ok(Expression::new(
                ExpressionKind::Type(DataType::new(data_type_kind, Some(constraints), full_span)),
                full_span,
            ))
        } else {
            let full_span = start_span.merge(end_span);

            Ok(Expression::new(
                ExpressionKind::Type(DataType::new(data_type_kind, None, full_span)),
                full_span,
            ))
        }
    }
}
