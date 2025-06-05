use std::path::Path;

use crate::{
    core::parser::{Parser, parser_error::ParserError},
    interpreter::{context::Context, evaluator},
};

#[test]
fn evaluate_type_declaration() {
    let source = "type even_positive_int = int [range=0..=1024, multiple_of=2];";
    let mut parser = Parser::new(source, Path::new(""));
    match parser.parse() {
        Ok(program) => {
            let mut context = Context::default();
            evaluator::evaluate(program, &mut context).unwrap();
            let data_type = context.get_type("even_positive_int");
            assert!(data_type.is_some());
            assert_eq!(data_type.unwrap().constraints.len(), 2);
        }
        Err(error) => handle_parser_error(error),
    }
}

#[test]
fn evluate_extend_type() {
    let source = "type positive_int = int [range=0..=1024]; type even_positive_int = extend positive_int with [multiple_of=2];";
    let mut parser = Parser::new(source, Path::new(""));
    match parser.parse() {
        Ok(program) => {
            let mut context = Context::default();
            evaluator::evaluate(program, &mut context).unwrap();
            let data_type = context.get_type("even_positive_int");
            assert!(data_type.is_some());
            // TODO: add proper assertions
            println!("{:?}", data_type);
        }
        Err(error) => handle_parser_error(error),
    }
}

fn handle_parser_error(error: ParserError) {
    match error {
        ParserError::Expected {
            span,
            expected,
            got,
        } => {
            panic!(
                "{}:{} ERROR: Expected '{}' but got '{}'",
                span.line, span.line_offset, expected, got
            )
        }
        ParserError::InvalidDirective(span) => panic!(
            "{}:{} ERROR: Invalid directive",
            span.line, span.line_offset
        ),
        ParserError::UnexpectedEOF => {
            panic!("ERROR: Missing enclosing \" or ;")
        }
        ParserError::Syntax(span, error) => {
            panic!("{}:{} ERROR: {}", span.line, span.line_offset, error)
        }
        ParserError::UndefinedConstraint(span) => panic!(
            "{}:{} ERROR: Undefined constraint",
            span.line, span.line_offset
        ),
        ParserError::InvalidStringPattern(span, pattern) => panic!(
            "{}:{} ERROR: Invalid pattern {}",
            span.line, span.line_offset, pattern
        ),
        ParserError::InvalidAttribute(span, token) => panic!(
            "{}:{} ERROR: Cannot put attribute on {}",
            span.line, span.line_offset, token
        ),
    }
}