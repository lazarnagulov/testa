use std::{iter::Peekable, str::CharIndices};

use crate::{
    ast::{Expression, ExpressionKind, PatternChar, PatternElement, Precedence},
    lexer::token::TokenKind,
    parser::error::ParserError,
    utils::Span,
};

use super::Parser;

impl<'src> Parser<'src> {
    pub(super) fn parse_string_pattern(&mut self) -> Result<Expression, ParserError> {
        let start = self.token_stream.consume_token()?;
        let literal_span = self.token_stream.expect_token(TokenKind::StringLiteral)?;
        let literal = self.string_literal_content(literal_span);
        let mut chars = literal.char_indices().peekable();
        let elements = self.parse_pattern_elements(literal, &mut chars)?;
        Ok(Expression::new(
            ExpressionKind::StringPattern(elements),
            start.merge(self.token_stream.last_span()),
        ))
    }

    pub(super) fn parse_pattern_elements(
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
                        result.push(PatternElement::Literal(
                            std::mem::take(&mut literal_element),
                            Span::default(),
                        ));
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

    pub(super) fn parse_pattern_condition(
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
                    let mut parser = Parser::new(&literal[current_index + 1..last]);
                    let count_expression = parser.parse_expression(Precedence::Lowest)?;

                    if let Some(PatternElement::RepeatChar {
                        ch: _,
                        count: _,
                        count_expression: expression,
                        span: _,
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
                        span: Span::default(),
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
}
