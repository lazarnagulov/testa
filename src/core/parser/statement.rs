use std::mem;
use std::path::PathBuf;

use crate::core::ast::{Attribute, Element, Field, Precedence, Variant};
use crate::core::utils::span::Span;
use crate::core::{ast::Statement, parser::error::ParserError};
use crate::core::lexer::token::TokenKind::*;
use super::Parser;

impl<'src> Parser<'src> {

    pub(super) fn parse_statement(&mut self) -> Result<Statement, ParserError> {
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

    pub(super) fn parse_enum(&mut self) -> Result<Statement, ParserError> {
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

    pub(super) fn parse_generate(&mut self) -> Result<Statement, ParserError> {
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

    
    pub(super) fn parse_output_path(&mut self) -> Result<Statement, ParserError> {
        let start = self.token_stream.consume_token()?;
        let argument = self.token_stream.expect_token(StringLiteral)?;
        let path = self.string_literal_content(argument);
        let end = self.token_stream.expect_token(Semicolon)?;
        
        Ok(Statement::OutputPathDirective {
            argument: PathBuf::from(path),
            span: start.merge(end) 
        })
    }

    pub(super) fn parse_directive(&mut self) -> Result<Statement, ParserError> {
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

    pub(super) fn parse_elements(&mut self) -> Result<Vec<Element>, ParserError> {
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

    pub(super) fn parse_fields(&mut self) -> Result<Vec<Field>, ParserError> {
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

    pub(super) fn parse_template(&mut self) -> Result<Statement, ParserError> {
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

    pub(super) fn parse_type_declaration(&mut self) -> Result<Statement, ParserError> {
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


    pub(super) fn parse_attribute(&mut self) -> Result<Statement, ParserError> {
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

}