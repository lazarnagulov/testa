pub mod error;

mod helpers;
mod token_stream;
mod expression;
mod statement;
mod data_type;

use std::iter::Peekable;
use std::path::Path;
use std::str::CharIndices;

use crate::core::ast::{
    Attribute, ConstraintExpression, ConstraintKind, Expression,
    ExpressionKind, PatternChar, PatternElement,
    Precedence, Program,
};
use crate::core::lexer::Lexer;
use crate::core::lexer::token::TokenKind::*;
use crate::core::parser::error::ParserError;
use crate::core::parser::token_stream::TokenStream;
use crate::core::utils::span::Span;

// TODO: Add lookups for prefix and infix expressions { TokenKind: fn () }
pub struct Parser<'src> {
    token_stream: TokenStream<'src>,
    source: &'src str,

    attributes: Vec<Attribute>,
    path: &'src Path,
}

impl<'src> Parser<'src> {
    pub fn new(program: &'src str, path: &'src Path) -> Self {
        let lexer = Lexer::new(program).peekable();
        Self {
            token_stream: TokenStream::new(lexer),
            source: program,
            attributes: Vec::new(),
            path,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut statements = vec![];
        while self.token_stream.has_next() {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        Ok(Program(statements))
    }


    fn parse_string_pattern(&mut self) -> Result<Expression, ParserError> {
        let span = self.token_stream.consume_token()?;
        let literal_span = self.token_stream.expect_token(StringLiteral)?;
        let literal = self.string_literal_content(literal_span);
        let mut chars = literal.char_indices().peekable();
        let elements = self.parse_pattern_elements(literal, &mut chars)?;
        // TODO: Include elements to span
        Ok(Expression::new(
            ExpressionKind::StringPattern(elements),
            span,
        ))
    }

    fn parse_pattern_elements(
        &self,
        literal: &str,
        chars: &mut Peekable<CharIndices>,
    ) -> Result<Vec<PatternElement>, ParserError> {
        let mut result = Vec::new();
        let mut literal_element = String::new();

        while let Some(&(_, ch)) = chars.peek() {
            if ch == '$' {
                let mut dollar_count = 0;
                while matches!(chars.peek(), Some(&(_, '$'))) {
                    chars.next();
                    dollar_count += 1;
                }

                if matches!(chars.peek(), Some(&(_, '{'))) {
                    if dollar_count > 1 {
                        literal_element.extend(std::iter::repeat_n('$', dollar_count - 1));
                    }
                    if !literal_element.is_empty() {
                        result.push(PatternElement::Literal(std::mem::take(
                            &mut literal_element,
                        ), Span::default()));
                    }
                    result.extend(self.parse_pattern_condition(literal, chars)?);
                } else {
                    literal_element.extend(std::iter::repeat_n('$', dollar_count));
                }
            } else {
                literal_element.push(ch);
                chars.next();
            }
        }

        if !literal_element.is_empty() {
            result.push(PatternElement::Literal(literal_element, Span::default()));
        }

        Ok(result)
    }

    fn parse_pattern_condition(
        &self,
        literal: &str,
        chars: &mut Peekable<CharIndices>,
    ) -> Result<Vec<PatternElement>, ParserError> {
        let mut result = Vec::new();
        chars.next();
        while let Some((current_index, current_char)) = chars.next() {
            if current_char == '}' {
                break;
            }
            match current_char {
                '[' => {
                    let mut last = current_index;
                    while let Some(char) = chars.peek() {
                        last = char.0;
                        if char.1 == ']' {
                            break;
                        }
                        chars.next();
                    }
                    if chars.peek().is_none_or(|(_, ch)| *ch != ']') {
                        return Err(ParserError::InvalidStringPattern {
                            span: Span::default(),
                            pattern: "Missing ]".to_owned(),
                        });
                    }

                    chars.next();
                    let mut parser = Parser::new(&literal[current_index + 1..last], self.path);
                    let count_expression = parser.parse_expression(Precedence::Lowest)?;

                    if let Some(PatternElement::RepeatChar {
                        ch: _,
                        count: _,
                        count_expression: expression,
                        span: _
                    }) = result.last_mut()
                    {
                        *expression = Some(count_expression);
                    } else {
                        return Err(ParserError::InvalidStringPattern {
                            span: Span::default(),
                            pattern: "Expected a, A or # before []".to_owned(),
                        });
                    }
                }
                'a' | 'A' | '#' => {
                    let mut count = 1;
                    while let Some((_, next_char)) = chars.peek() {
                        if *next_char == current_char {
                            count += 1;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    result.push(PatternElement::RepeatChar {
                        ch: match current_char {
                            'a' => PatternChar::Lowercase,
                            'A' => PatternChar::Uppercase,
                            '#' => PatternChar::Digit,
                            _ => unreachable!(),
                        },
                        count,
                        count_expression: None,
                        span: Span::default()
                    });
                }
                _ => {
                    return Err(ParserError::InvalidStringPattern {
                        span: Span::default(),
                        pattern: current_char.to_string(),
                    });
                }
            }
        }
        Ok(result)
    }

    fn parse_constraints(&mut self) -> Result<Vec<ConstraintExpression>, ParserError> {
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
                        span
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

    fn parse_literal(&mut self) -> Result<Expression, ParserError> {
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
                let number = self.token_text(span)
                    .parse()
                    .unwrap();
                Ok(Expression {
                    kind: ExpressionKind::IntLiteral(number),
                    span,
                })
            }
            kind => Err(ParserError::Expected{ expected: "literal".to_owned(), got: kind.to_string(), span }),
        };
        self.token_stream.next_token()?;        
        parsed
    }
}