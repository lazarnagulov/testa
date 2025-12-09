use std::path::Path;

use crate::{evaluator, evaluator::context::Context};
use testa_core::parser::{Parser, error::ParserError};

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
        Err(error) => handle_error(error),
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
        Err(error) => handle_error(error),
    }
}

fn handle_error(error: ParserError) {
    match error {
        ParserError::Expected {
            span,
            expected,
            got,
        } => {
            panic!("expected '{}' got '{}' in {}", expected, got, span)
        }
        ParserError::InvalidDirective { span } => panic!("{}", span.to_string()),
        ParserError::UnexpectedEof { span } => {
            panic!("{}", span.to_string())
        }
        ParserError::Syntax { span, message } => {
            panic!("{}: {}", span, message)
        }
        ParserError::UndefinedConstraint { span } => panic!("{}", span.to_string()),
        ParserError::InvalidStringPattern { span, pattern } => {
            panic!("{}: {}", span, pattern)
        }
        ParserError::InvalidAttribute { span, token } => panic!("{}: {}", span, token),
        ParserError::LexerError(lexer_error) => panic!("{}", lexer_error.to_string()),
    }
}
