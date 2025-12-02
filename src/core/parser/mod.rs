pub mod error;
mod helpers;
mod token_stream;

use std::iter::Peekable;
use std::mem;
use std::path::Path;
use std::{path::PathBuf, str::CharIndices};

use crate::core::ast::{
    Attribute, ConstraintExpression, ConstraintKind, DataType, DataTypeKind, Element, Expression,
    ExpressionKind, ExpressionStatemnt, Field, InfixOperator, PatternChar, PatternElement,
    Precedence, PrefixOperator, Program, Statement, Variant,
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

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.token_stream.peek_kind() {
            Output | Seed => self.parse_directive(),
            OutputPath => self.parse_output_path(),
            Template => self.parse_template(),
            Resource => todo!(),
            Tag => self.parse_attribute(),
            Enum => self.parse_enum(),
            Generate => self.parse_generate(),
            Type => self.parse_type_declaration(),
            _ => Ok(Statement::Expression(self.parse_expression_statement()?)),
        }
    }

    fn parse_output_path(&mut self) -> Result<Statement, ParserError> {
        self.token_stream.consume_token()?;
        let argument = self.token_stream.expect_token(StringLiteral)?;
        let path = self.string_literal_content(argument);
        self.token_stream.expect_token(Semicolon)?;
        Ok(Statement::OutputPathDirective {
            argument: PathBuf::from(path),
            span: Span::default()
        })
    }

    fn parse_directive(&mut self) -> Result<Statement, ParserError> {
        let directive = self.token_stream.next_token()?;
        let argument = self.parse_identifier_as_string()?;
        let options: Vec<Field>;
        if self.token_stream.peek_kind() == &LBrace {
            options = self.parse_fields()?;
        } else {
            options = vec![];
            self.token_stream.expect_token(Semicolon)?;
        }
        match directive.kind {
            Output => Ok(Statement::OutputDirective { argument, options, span: Span::default() }),
            Seed => todo!(),
            _ => Err(ParserError::InvalidDirective { span: directive.span }),
        }
    }

    fn parse_template(&mut self) -> Result<Statement, ParserError> {
        self.token_stream.next_token()?;
        let attributes = mem::take(&mut self.attributes);
        let name = self.parse_identifier_as_string()?;
        let parent = if self.token_stream.peek_kind() == &Colon {
            self.token_stream.consume_token()?;
            Some(self.parse_identifier_as_string()?)
        } else {
            None
        };
        let fields = self.parse_fields()?;
        Ok(Statement::Template {
            parent,
            name,
            attributes,
            body: fields,
            span: Span::default()
        })
    }

    fn parse_generate(&mut self) -> Result<Statement, ParserError> {
        self.token_stream.next_token()?;
        let name = self.parse_identifier_as_string()?;
        self.token_stream.expect_token(LBracket)?;
        let count = self.parse_expression(Precedence::Lowest)?;
        self.token_stream.expect_token(RBracket)?;
        if name == "_" {
            let fields = self.parse_fields()?;
            Ok(Statement::Generate {
                template_name: None,
                body: fields,
                count,
                span: Span::default()
            })
        } else {
            self.token_stream.expect_token(Semicolon)?;
            Ok(Statement::Generate {
                template_name: Some(name),
                body: Vec::new(),
                count,
                span: Span::default()
            })
        }
    }

    fn parse_enum(&mut self) -> Result<Statement, ParserError> {
        self.token_stream.next_token()?;
        let name = self.parse_identifier_as_string()?;
        self.token_stream.expect_token(LBrace)?;
        let variants = self.parse_variants()?;
        Ok(Statement::Enum {
            name,
            variants,
            attributes: mem::take(&mut self.attributes),
            span: Span::default()
        })
    }

    // FIXME: Something is wrong with start, size calculation
    fn parse_elements(&mut self) -> Result<Vec<Element>, ParserError> {
        let mut elements = vec![];
        while matches!(
            self.token_stream.peek_kind(),
            False | True | StringLiteral | FloatLiteral | IntLiteral
        ) {
            let value = self.parse_expression(Precedence::Lowest)?;
            let weight = if self.token_stream.peek_kind() == &Arrow {
                self.token_stream.next_token()?;
                let weight_expression = self.parse_expression(Precedence::Lowest)?;
                Some(weight_expression)
            } else {
                None
            };
            elements.push(Element::new(value, weight, Span::default()));
            if self.token_stream.peek_kind() == &RBracket {
                break;
            }
            self.token_stream.expect_token(Semicolon)?;
        }
        self.token_stream.expect_token(RBracket)?;
        Ok(elements)
    }

    fn parse_variants(&mut self) -> Result<Vec<Variant>, ParserError> {
        let mut variants = vec![];
        while self.token_stream.peek_kind() == &Identifier {
            let name = self.parse_identifier_as_string()?;
            let weight = if self.token_stream.peek_kind() == &Arrow {
                self.token_stream.next_token()?;
                Some(self.parse_expression(Precedence::Lowest)?)
            } else {
                None
            };
            variants.push(Variant::new(name, weight));
            if self.token_stream.peek_kind() == &RBrace {
                break;
            }
            self.token_stream.expect_token(Semicolon)?;
        }
        self.token_stream.expect_token(RBrace)?;
        Ok(variants)
    }

    fn parse_fields(&mut self) -> Result<Vec<Field>, ParserError> {
        self.token_stream.expect_token(LBrace)?;
        let mut field_attributes = Vec::new();
        let mut options = Vec::new();
        while self.token_stream.peek_kind() == &Identifier
            || self.token_stream.peek_kind() == &Override
            || self.token_stream.peek_kind() == &Tag
        {
            if self.token_stream.peek_kind() == &Tag {
                let span = self.token_stream.consume_token()?;
                let attribute = &self.source[span.start.offset + 2..span.end.offset];
                field_attributes.push(Attribute::Flag(attribute.to_owned(), span));
                continue;
            }
            let overridable = self.token_stream.peek_kind() == &Override;
            if overridable {
                self.token_stream.consume_token()?;
            }
            let name = self.parse_identifier_as_string()?;
            self.token_stream.expect_token(SingleEqual)?;
            let expression = self.parse_expression(Precedence::Lowest)?;
            let span = self.token_stream.expect_token(Semicolon)?;
            options.push(Field::new(
                name,
                expression,
                overridable,
                mem::take(&mut field_attributes),
                span
            ));
        }
        self.token_stream.expect_token(RBrace)?;
        Ok(options)
    }

    fn parse_peeked_token_as_string(&mut self) -> Result<String, ParserError> {
        let token = self.token_stream.peek_token()?;
        let span = token.span;
        Ok(self.source_text(span).to_string())
    }

    fn parse_identifier_as_string(&mut self) -> Result<String, ParserError> {
        let span = self.token_stream.expect_token(Identifier)?;
        Ok(self.source_text(span).to_string())
    }

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatemnt, ParserError> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        let span = self.token_stream.expect_token(Semicolon)?;

        Ok(ExpressionStatemnt { expression, span })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
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

    fn parse_primary_expression(&mut self) -> Result<Expression, ParserError> {
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

    fn parse_attribute(&mut self) -> Result<Statement, ParserError> {
        let span = self.token_stream.consume_token()?;
        let attribute = &self.source[span.start.offset + 2..span.end.offset];
        self.attributes.push(Attribute::Flag(attribute.to_owned(), Span::default()));
        match self.token_stream.peek_kind() {
            Template => self.parse_template(),
            Type => self.parse_type_declaration(),
            Enum => self.parse_enum(),
            Tag => self.parse_attribute(),
            tok => Err(ParserError::InvalidAttribute {
                token: tok.to_string(),
                span,
            }),
        }
    }

    fn parse_list_expression(&mut self) -> Result<Expression, ParserError> {
        self.token_stream.consume_token()?;
        let elements = self.parse_elements()?;
        Ok(Expression::new(ExpressionKind::List(elements), Span::default()))
    }

    fn parse_list_type(&mut self) -> Result<Expression, ParserError> {
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
                    Span::default()
                )),
                Span::default(),
            ))
        } else {
            Ok(Expression::new(
                ExpressionKind::Type(DataType::new(DataTypeKind::List(Box::new(data_type)), None, Span::default())),
                Span::default(),
            ))
        }
    }

    fn parse_type_declaration(&mut self) -> Result<Statement, ParserError> {
        self.token_stream.next_token()?;
        let name = self.parse_identifier_as_string()?;
        self.token_stream.expect_token(SingleEqual)?;
        let data_type = self.parse_type()?;
        let span = self.token_stream.expect_token(Semicolon)?;
        Ok(Statement::TypeDecl {
            name,
            data_type,
            attributes: mem::take(&mut self.attributes),
            span
        })
    }

    fn parse_type(&mut self) -> Result<Expression, ParserError> {
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
                    return Err(ParserError::Expected { expected: "with".to_owned(), got: format!("{}", *peek), span });
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
                ExpressionKind::Type(DataType::new(data_type_kind, Some(constraints), Span::default())),
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

    fn parse_group_expression(&mut self) -> Result<Expression, ParserError> {
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

    fn parse_prefix_expression(
        &mut self,
        operator: PrefixOperator,
    ) -> Result<Expression, ParserError> {
        let expression = self.parse_expression(Precedence::Prefix)?;
        let span = expression.span;
        Ok(Expression::new(
            ExpressionKind::Prefix {
                operator,
                expression: Box::new(expression),
            },
            span,
        ))
    }

    fn parse_infix_expression(
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