use std::mem;
use std::path::PathBuf;

use super::Parser;
use crate::ast::{Attribute, Element, Field, Precedence, Variant};
use crate::lexer::error::LexerError;
use crate::lexer::token::{Token, TokenKind::*};
use crate::{ast::Statement, parser::error::ParserError};

impl<'src, I> Parser<'src, I>
where
    I: Iterator<Item = Result<Token, LexerError>>,
{
    pub(super) fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.token_stream.peek_kind() {
            Output | Seed => self.parse_directive(),
            OutputPath => self.parse_output_path(),
            Template => self.parse_template(),
            Struct => self.parse_struct(),
            Resource => todo!(),
            Tag => self.parse_attribute(),
            Enum => self.parse_enum(),
            Generate => self.parse_generate(),
            Type => self.parse_type_declaration(),
            _ => Ok(Statement::Expression(self.parse_expression_statement()?)),
        }
    }

    pub(super) fn parse_enum(&mut self) -> Result<Statement, ParserError> {
        let token = self.token_stream.next_token()?;
        let start = token.span;
        let name_span = self.token_stream.expect_token(Identifier)?;
        let name = self.token_text(name_span).to_string();

        self.token_stream.expect_token(LBrace)?;
        let variants = self.parse_enum_variants()?;
        Ok(Statement::Enum {
            name,
            variants,
            attributes: mem::take(&mut self.attributes),
            span: start.merge(self.token_stream.last_span()),
            name_span: Some(name_span),
        })
    }

    fn parse_enum_variants(&mut self) -> Result<Vec<Variant>, ParserError> {
        let mut variants = vec![];
        while self.token_stream.peek_kind() == &Identifier {
            let start = self.token_stream.peek_token()?.span;
            let name_span = self.token_stream.expect_token(Identifier)?;
            let name = self.token_text(name_span);
            let weight = if self.token_stream.peek_kind() == &Arrow {
                self.token_stream.next_token()?;
                Some(self.parse_expression(Precedence::Lowest)?)
            } else {
                None
            };
            variants.push(Variant::with_name_span(
                name,
                weight,
                start.merge(self.token_stream.last_span()),
                name_span,
            ));
            if self.token_stream.peek_kind() == &RBrace {
                break;
            }
            self.token_stream.expect_token(Semicolon)?;
        }
        self.token_stream.expect_token(RBrace)?;
        Ok(variants)
    }

    pub(super) fn parse_generate(&mut self) -> Result<Statement, ParserError> {
        let token = self.token_stream.next_token()?;
        let start = token.span;
        let name_span = self.token_stream.expect_token(Identifier)?;
        let name = self.token_text(name_span).to_string();
        self.token_stream.expect_token(LBracket)?;
        let count = self.parse_expression(Precedence::Lowest)?;
        self.token_stream.expect_token(RBracket)?;
        if name == "_" {
            let fields = self.parse_template_fields()?;
            Ok(Statement::Generate {
                template_name: None,
                template_name_span: None,
                body: fields,
                count,
                span: start.merge(self.token_stream.last_span()),
            })
        } else {
            let end = self.token_stream.expect_token(Semicolon)?;
            Ok(Statement::Generate {
                template_name: Some(name),
                template_name_span: Some(name_span),
                body: Vec::new(),
                count,
                span: start.merge(end),
            })
        }
    }

    pub(super) fn parse_output_path(&mut self) -> Result<Statement, ParserError> {
        let start = self.token_stream.consume_token()?;
        let argument = self.token_stream.expect_token(StringLiteral)?;
        let path = self.string_literal_content(argument);
        let end = self.token_stream.expect_token(Semicolon)?;

        Ok(Statement::OutputPathDirective {
            argument: PathBuf::from(path),
            span: start.merge(end),
        })
    }

    pub(super) fn parse_directive(&mut self) -> Result<Statement, ParserError> {
        let directive = self.token_stream.next_token()?;
        let start = directive.span;
        let argument = self.parse_identifier_as_string()?;
        let options: Vec<Field>;
        if self.token_stream.peek_kind() == &LBrace {
            options = self.parse_template_fields()?;
        } else {
            options = vec![];
            self.token_stream.expect_token(Semicolon)?;
        }
        match directive.kind {
            Output => Ok(Statement::OutputDirective {
                argument,
                options,
                span: start.merge(self.token_stream.last_span()),
            }),
            Seed => todo!(),
            _ => Err(ParserError::InvalidDirective {
                span: start.merge(self.token_stream.last_span()),
            }),
        }
    }

    pub(super) fn parse_list_elements(&mut self) -> Result<Vec<Element>, ParserError> {
        let mut elements = vec![];
        while matches!(
            self.token_stream.peek_kind(),
            False | True | StringLiteral | FloatLiteral | IntLiteral
        ) {
            let value = self.parse_expression(Precedence::Lowest)?;
            let start = value.span;
            let weight = if self.token_stream.peek_kind() == &Arrow {
                self.token_stream.next_token()?;
                let weight_expression = self.parse_expression(Precedence::Lowest)?;
                Some(weight_expression)
            } else {
                None
            };
            let end_token = self.token_stream.peek_token()?;
            if end_token.kind == RBracket {
                let end = self.token_stream.consume_token()?;
                elements.push(Element::new(value, weight, start.merge(end)));
                break;
            }
            let end = self.token_stream.expect_token(Semicolon)?;
            elements.push(Element::new(value, weight, start.merge(end)));
        }
        Ok(elements)
    }

    pub(super) fn parse_template_fields(&mut self) -> Result<Vec<Field>, ParserError> {
        self.token_stream.expect_token(LBrace)?;
        let mut field_attributes = Vec::new();
        let mut options = Vec::new();

        while matches!(self.token_stream.peek_kind(), Identifier | Override | Tag) {
            if self.token_stream.peek_kind() == &Tag {
                let span = self.token_stream.consume_token()?;
                let attribute = self.attribute_text(span);
                field_attributes.push(Attribute::Flag(attribute.to_owned(), span));
                continue;
            }

            let start_span = self.token_stream.peek_token()?.span;
            let overridable = self.token_stream.peek_kind() == &Override;
            if overridable {
                self.token_stream.consume_token()?;
            }

            let name_span = self.token_stream.expect_token(Identifier)?;
            let name = self.token_text(name_span);
            self.token_stream.expect_token(SingleEqual)?;
            let expression = self.parse_expression(Precedence::Lowest)?;
            let end_span = self.token_stream.expect_token(Semicolon)?;

            options.push(Field::with_name_span(
                name,
                expression,
                overridable,
                mem::take(&mut field_attributes),
                start_span.merge(end_span),
                name_span,
            ));
        }

        self.token_stream.expect_token(RBrace)?;
        Ok(options)
    }

    pub(super) fn parse_template(&mut self) -> Result<Statement, ParserError> {
        let token = self.token_stream.next_token()?;
        let start = token.span;
        let attributes = mem::take(&mut self.attributes);
        let name_span = self.token_stream.expect_token(Identifier)?;
        let name = self.token_text(name_span).to_string();

        let (parent_name, parent_span) = if self.token_stream.peek_kind() == &Colon {
            self.token_stream.consume_token()?;
            let parent_span = self.token_stream.expect_token(Identifier)?;
            let parent_name = self.token_text(parent_span);
            (Some(parent_name), Some(parent_span))
        } else {
            (None, None)
        };

        let fields = self.parse_template_fields()?;

        Ok(Statement::Template {
            parent_name: parent_name.map(String::from),
            parent_span,
            name,
            attributes,
            body: fields,
            span: start.merge(self.token_stream.last_span()),
            name_span: Some(name_span),
        })
    }

    pub(super) fn parse_type_declaration(&mut self) -> Result<Statement, ParserError> {
        let token = self.token_stream.next_token()?;
        let start = token.span;
        let name_span = self.token_stream.expect_token(Identifier)?;
        let name = self.token_text(name_span).to_string();
        self.token_stream.expect_token(SingleEqual)?;
        let data_type = self.parse_type()?;
        let end = self.token_stream.expect_token(Semicolon)?;
        Ok(Statement::TypeDecl {
            name,
            data_type,
            attributes: mem::take(&mut self.attributes),
            span: start.merge(end),
            name_span: Some(name_span),
        })
    }

    pub(super) fn parse_attribute(&mut self) -> Result<Statement, ParserError> {
        let span = self.token_stream.consume_token()?;
        let attribute = self.attribute_text(span);
        self.attributes
            .push(Attribute::Flag(attribute.to_owned(), span));
        match self.token_stream.peek_kind() {
            Template => self.parse_template(),
            Type => self.parse_type_declaration(),
            Enum => self.parse_enum(),
            Tag => self.parse_attribute(),
            tok => Err(ParserError::InvalidAttribute {
                token: tok.to_string(),
                span: self.token_stream.last_span(),
            }),
        }
    }

    pub(super) fn parse_struct(&mut self) -> Result<Statement, ParserError> {
        let token = self.token_stream.next_token()?;
        let start = token.span;
        let name_span = self.token_stream.expect_token(Identifier)?;
        let name = self.token_text(name_span).to_string();
        let fields = self.parse_template_fields()?;

        Ok(Statement::Struct {
            name,
            name_span,
            body: fields,
            span: start.merge(self.token_stream.last_span()),
        })
    }
}
