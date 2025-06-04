pub mod ast;
pub mod parser_error;
pub mod tests;

use std::iter::Peekable;
use std::{path::PathBuf, str::CharIndices};

use crate::{
    lexer::{
        Lexer,
        token::TokenKind::{self, *},
    },
    parser::{
        ast::{
            ConstraintExpression, ConstraintKind, DataType, DataTypeKind, Element, Expression,
            ExpressionKind, ExpressionStatemnt, Field, InfixOperator, PatternChar, PatternElement,
            Precedence, PrefixOperator, Program, Statement, Variant,
        },
        parser_error::ParserError,
    },
};

// TODO: Add lookups for prefix and infix expressions { TokenKind: fn () }
pub struct Parser<'src> {
    lexer: Peekable<Lexer<'src>>,
    source: &'src str,
}

impl<'src> Parser<'src> {
    pub fn new(program: &'src str) -> Self {
        let lexer = Lexer::new(program).peekable();
        Parser {
            lexer,
            source: program,
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut statements = vec![];
        while self.lexer.peek().is_some() {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        Ok(Program(statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.peek_kind() {
            Output | Seed => self.parse_directive(),
            OutputPath => self.parse_output_path(),
            Template => self.parse_template(),
            Resource => todo!(),
            Enum => self.parse_enum(),
            Generate => self.parse_generate(),
            Type => self.parse_type_declaration(),
            _ => Ok(Statement::Expression(self.parse_expression_statement()?)),
        }
    }

    fn parse_output_path(&mut self) -> Result<Statement, ParserError> {
        self.consume_token();
        let argument = self.expect_token(StringLiteral)?;
        let path = &self.source[argument.0 + 1..argument.0 + argument.1 - 1];
        self.expect_token(Semicolon)?;
        Ok(Statement::OutputPathDirective {
            argument: PathBuf::from(path),
        })
    }

    fn parse_directive(&mut self) -> Result<Statement, ParserError> {
        let directive = self.lexer.next().unwrap();
        let argument = self.parse_identifier_as_string()?;
        let options: Vec<Field>;
        if self.peek_kind() == &LBrace {
            options = self.parse_fields()?;
        } else {
            options = vec![];
            self.expect_token(Semicolon)?;
        }
        match directive.kind {
            Output => Ok(Statement::OutputDirective { argument, options }),
            Seed => todo!(),
            _ => Err(ParserError::InvalidDirective),
        }
    }

    fn parse_template(&mut self) -> Result<Statement, ParserError> {
        self.lexer.next();
        let name = self.parse_identifier_as_string()?;
        let parent = if self.peek_kind() == &Colon {
            self.consume_token();
            Some(self.parse_identifier_as_string()?)
        } else {
            None
        };
        let fields = self.parse_fields()?;
        Ok(Statement::Template {
            parent,
            name,
            body: fields,
        })
    }

    fn parse_generate(&mut self) -> Result<Statement, ParserError> {
        self.lexer.next();
        let name = self.parse_identifier_as_string()?;
        self.expect_token(LBracket)?;
        let count = self.parse_expression(Precedence::Lowest)?;
        self.expect_token(RBracket)?;
        if name == "_" {
            let fields = self.parse_fields()?;
            Ok(Statement::Generate {
                template_name: None,
                body: fields,
                count,
            })
        } else {
            self.expect_token(Semicolon)?;
            Ok(Statement::Generate {
                template_name: Some(name),
                body: Vec::new(),
                count,
            })
        }
    }

    fn parse_enum(&mut self) -> Result<Statement, ParserError> {
        self.lexer.next();
        let name = self.parse_identifier_as_string()?;
        self.expect_token(LBrace)?;
        let variants = self.parse_variants()?;
        Ok(Statement::Enum { name, variants })
    }

    // FIXME: Something is wrong with start, size calculation
    fn parse_elements(&mut self) -> Result<Vec<Element>, ParserError> {
        let mut elements = vec![];
        while matches!(
            self.peek_kind(),
            False | True | StringLiteral | FloatLiteral | IntLiteral
        ) {
            let value = self.parse_expression(Precedence::Lowest)?;
            let start = value.start;
            let mut size = value.size;
            let weight = if self.peek_kind() == &Arrow {
                self.lexer.next();
                let weight_expression = self.parse_expression(Precedence::Lowest)?;
                size += weight_expression.size + 2;
                Some(weight_expression)
            } else {
                None
            };
            elements.push(Element::new(value, weight, start, size));
            if self.peek_kind() == &RBracket {
                break;
            }
            self.expect_token(Semicolon)?;
        }
        self.expect_token(RBracket)?;
        Ok(elements)
    }

    fn parse_variants(&mut self) -> Result<Vec<Variant>, ParserError> {
        let mut variants = vec![];
        while self.peek_kind() == &Identifier {
            let name = self.parse_identifier_as_string()?;
            let weight = if self.peek_kind() == &Arrow {
                self.lexer.next();
                Some(self.parse_expression(Precedence::Lowest)?)
            } else {
                None
            };
            variants.push(Variant::new(name, weight));
            if self.peek_kind() == &RBrace {
                break;
            }
            self.expect_token(Semicolon)?;
        }
        self.expect_token(RBrace)?;
        Ok(variants)
    }

    fn parse_fields(&mut self) -> Result<Vec<Field>, ParserError> {
        self.expect_token(LBrace)?;
        let mut options = vec![];
        while self.peek_kind() == &Identifier || self.peek_kind() == &Override {
            let overridable = self.peek_kind() == &Override;
            if overridable {
                self.consume_token();
            }
            let name = self.parse_identifier_as_string()?;
            self.expect_token(SingleEqual)?;
            let expression = self.parse_expression(Precedence::Lowest)?;
            self.expect_token(Semicolon)?;
            options.push(Field::new(name, expression, overridable));
        }
        self.expect_token(RBrace)?;
        Ok(options)
    }

    fn parse_peeked_token_as_string(&mut self) -> String {
        let token = self.lexer.peek().unwrap();
        let start = token.start;
        let size = token.size;
        self.source[start..start + size].to_string()
    }

    fn parse_identifier_as_string(&mut self) -> Result<String, ParserError> {
        let (start, size) = self.expect_token(Identifier)?;
        Ok(self.source[start..start + size].to_string())
    }

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatemnt, ParserError> {
        let expression = self.parse_expression(Precedence::Lowest)?;
        self.expect_token(Semicolon)?;

        Ok(ExpressionStatemnt { expression })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        let mut expression = self.parse_primary_expression()?;

        while precedence < self.current_precendence() {
            expression = match &self.peek_kind() {
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
                    return Err(ParserError::syntax_err(&format!(
                        "Invalid operator: {}",
                        token
                    )));
                }
            }
        }
        Ok(expression)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, ParserError> {
        match &self.peek_kind() {
            Int | Float | Str | Bool => Ok(self.parse_type()?),
            LBracket => {
                let kind = self.peek_kind_n(2);
                match kind {
                    Int | Float | Str | Bool | LBracket | Identifier => self.parse_type(),
                    True | False | IntLiteral | StringLiteral | FloatLiteral => {
                        self.parse_list_expression()
                    }
                    obj => Err(ParserError::expected(
                        "data type or literal",
                        &format!("{}", obj),
                    )),
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
            kind => Err(ParserError::syntax_err(&format!(
                "Invalid primary expression: {}",
                kind
            ))),
        }
    }

    fn parse_list_expression(&mut self) -> Result<Expression, ParserError> {
        let (start, _) = self.consume_token();
        let elements = self.parse_elements()?;
        let size = elements.iter().fold(0, |acc, element| acc + element.size) + 1;
        Ok(Expression::new(ExpressionKind::List(elements), start, size))
    }

    fn parse_list_type(&mut self) -> Result<Expression, ParserError> {
        self.consume_token();
        let data_type = self.parse_type()?;
        let start = data_type.start;
        let size = data_type.size;
        let ExpressionKind::Type(data_type) = data_type.kind else {
            unreachable!()
        };
        self.expect_token(RBracket)?;
        if self.peek_kind() == &LBracket {
            let consraints = self.parse_constraints()?;
            Ok(Expression::new(
                ExpressionKind::Type(DataType::new(
                    DataTypeKind::List(Box::new(data_type)),
                    Some(consraints),
                )),
                start,
                start + size + 1,
            ))
        } else {
            Ok(Expression::new(
                ExpressionKind::Type(DataType::new(DataTypeKind::List(Box::new(data_type)), None)),
                start,
                start + size + 1,
            ))
        }
    }

    fn parse_type_declaration(&mut self) -> Result<Statement, ParserError> {
        self.lexer.next();
        let name = self.parse_identifier_as_string()?;
        self.expect_token(SingleEqual)?;
        let data_type = self.parse_type()?;
        self.expect_token(Semicolon)?;
        Ok(Statement::TypeDecl { name, data_type })
    }

    fn parse_type(&mut self) -> Result<Expression, ParserError> {
        let token = self.lexer.peek().ok_or(ParserError::UnexpectedEOF)?;
        let start = token.start;
        let size = token.size;

        if token.kind == LBracket {
            return self.parse_list_type();
        }

        let data_type_kind = match token.kind {
            Int => DataTypeKind::Int,
            Float => DataTypeKind::Float,
            Str => DataTypeKind::Str,
            Bool => DataTypeKind::Boolean,
            Extend => {
                self.consume_token();
                let name = self.parse_identifier_as_string()?;
                let peek = self.peek_kind();
                if peek != &With {
                    return Err(ParserError::expected("with", &format!("{}", *peek)));
                }
                DataTypeKind::Custom(name)
            }
            Identifier => DataTypeKind::Custom(self.parse_peeked_token_as_string()),
            _ => unreachable!(),
        };

        self.lexer.next();
        if self.peek_kind() == &LBracket {
            let constraints = self.parse_constraints()?;
            // TODO: calculate start and size
            Ok(Expression::new(
                ExpressionKind::Type(DataType::new(data_type_kind, Some(constraints))),
                0,
                0,
            ))
        } else {
            Ok(Expression::new(
                ExpressionKind::Type(DataType::new(data_type_kind, None)),
                start,
                size,
            ))
        }
    }

    fn parse_string_pattern(&mut self) -> Result<Expression, ParserError> {
        let (start, _size) = self.consume_token();
        let (literal_start, literal_size) = self.expect_token(StringLiteral)?;
        let literal = self.source[literal_start + 1..literal_start + literal_size - 1].to_string();
        let mut chars = literal.char_indices().peekable();
        let elements = self.parse_pattern_elements(&literal, &mut chars)?;
        // TODO: Implement size
        Ok(Expression::new(
            ExpressionKind::StringPattern(elements),
            start,
            0,
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
                        )));
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
            result.push(PatternElement::Literal(literal_element));
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
                        return Err(ParserError::InvalidStringPattern("Missing ]".to_owned()));
                    }

                    chars.next();
                    let mut parser = Parser::new(&literal[current_index + 1..last]);
                    let count_expression = parser.parse_expression(Precedence::Lowest)?;

                    if let Some(PatternElement::RepeatChar {
                        ch: _,
                        count: _,
                        count_expression: expression,
                    }) = result.last_mut()
                    {
                        *expression = Some(count_expression);
                    } else {
                        return Err(ParserError::InvalidStringPattern(
                            "Expected a, A or # before []".to_owned(),
                        ));
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
                    });
                }
                _ => return Err(ParserError::InvalidStringPattern(current_char.to_string())),
            }
        }
        Ok(result)
    }

    fn parse_constraints(&mut self) -> Result<Vec<ConstraintExpression>, ParserError> {
        self.consume_token();
        let mut constraints: Vec<ConstraintExpression> = vec![];

        while self.peek_kind() == &Identifier {
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
                    if self.peek_kind() == &SingleEqual {
                        Err(ParserError::UndefinedConstraint)
                    } else {
                        let expression = self.parse_expression(Precedence::Lowest)?;
                        Ok(ConstraintExpression::new(
                            expression,
                            ConstraintKind::Custom,
                        ))
                    }
                }
            }?;
            constraints.push(constraint);
            if self.peek_kind() == &RBracket {
                break;
            }
            self.expect_token(Comma)?;
        }
        self.expect_token(RBracket)?;
        Ok(constraints)
    }

    fn parse_constraint_expression(
        &mut self,
        constraint_kind: ConstraintKind,
    ) -> Result<ConstraintExpression, ParserError> {
        self.expect_token(SingleEqual)?;
        let expression = self.parse_expression(Precedence::Lowest)?;
        Ok(ConstraintExpression::new(expression, constraint_kind))
    }

    fn parse_literal(&mut self) -> Result<Expression, ParserError> {
        let token = self.lexer.peek().ok_or(ParserError::UnexpectedEOF)?;
        let start = token.start;
        let size = token.size;
        let parsed = match &token.kind {
            StringLiteral => {
                let literal = self.source[start + 1..start + size - 1].to_string();
                Ok(Expression::new(
                    ExpressionKind::StringLiteral(literal),
                    start,
                    size,
                ))
            }
            Identifier => {
                let literal = self.source[start..start + size].to_string();
                Ok(Expression::new(
                    ExpressionKind::Identifier(literal),
                    start,
                    size,
                ))
            }
            FloatLiteral => {
                let literal = self.source[start..start + size].to_string();
                Ok(Expression::new(
                    ExpressionKind::FloatLiteral(literal),
                    start,
                    size,
                ))
            }
            True => Ok(Expression::new(
                ExpressionKind::BooleanLiteral(true),
                start,
                size,
            )),
            False => Ok(Expression::new(
                ExpressionKind::BooleanLiteral(false),
                start,
                size,
            )),
            IntLiteral => {
                let number = self.source[start..start + size].parse().unwrap();
                Ok(Expression {
                    kind: ExpressionKind::IntLiteral(number),
                    start,
                    size,
                })
            }
            kind => Err(ParserError::expected("literal", &kind.to_string())),
        };
        self.lexer.next();
        parsed
    }

    fn parse_group_expression(&mut self) -> Result<Expression, ParserError> {
        let (start, _) = self.consume_token();
        let expression = self.parse_expression(Precedence::Lowest)?;
        match self.peek_kind() {
            RParen => {
                let end = self.expect_token(RParen).unwrap().0;
                Ok(Expression::new(expression.kind, start, (end + 1) - start))
            }
            kind => Err(ParserError::Expected {
                expected: ")".to_string(),
                got: kind.to_string(),
            }),
        }
    }

    fn parse_prefix_expression(
        &mut self,
        operator: PrefixOperator,
    ) -> Result<Expression, ParserError> {
        let (start, size) = self.consume_token();
        let expression = self.parse_expression(Precedence::Prefix)?;
        let size = size + expression.size;
        Ok(Expression::new(
            ExpressionKind::Prefix {
                operator,
                expression: Box::new(expression),
            },
            start,
            size,
        ))
    }

    fn parse_infix_expression(
        &mut self,
        left: Expression,
        operator: InfixOperator,
        precendence: Precedence,
    ) -> Result<Expression, ParserError> {
        self.lexer.next();
        let right = self.parse_expression(precendence)?;
        let start = left.start;
        let end = right.start + right.size;

        Ok(Expression::new(
            ExpressionKind::Infix {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            },
            start,
            end - start,
        ))
    }

    fn expect_token(&mut self, kind: TokenKind) -> Result<(usize, usize), ParserError> {
        let token = self.lexer.next().ok_or(ParserError::UnexpectedEOF)?;
        if token.kind != kind {
            Err(ParserError::syntax_err(&format!(
                "Expected {} but got {}",
                kind, token.kind
            )))
        } else {
            Ok((token.start, token.size))
        }
    }

    fn consume_token(&mut self) -> (usize, usize) {
        let token = self.lexer.next().unwrap();
        (token.start, token.size)
    }

    fn peek_kind(&mut self) -> &TokenKind {
        self.lexer.peek().map_or(&Eof, |t| &t.kind)
    }

    fn peek_kind_n(&mut self, n: usize) -> TokenKind {
        self.lexer
            .clone()
            .nth(n - 1)
            .map_or(Eof, |token| token.kind)
    }

    fn current_precendence(&mut self) -> Precedence {
        match self.peek_kind() {
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
