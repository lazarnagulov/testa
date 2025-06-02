#[cfg(test)]
mod constraint_test {
    use crate::{
        evaluator::{context::Context, evaluator::evaluate},
        parser::{parser::Parser, parser_error::ParserError},
    };

    #[test]
    fn evaluate_type_declaration() {
        let source = "type even_positive_int = int [range=0..=1024, multiple_of=2];";
        let mut parser = Parser::new(source);
        match parser.parse() {
            Ok(program) => {
                let mut context = Context::default();
                evaluate(program, &mut context).unwrap();
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
        let mut parser = Parser::new(source);
        match parser.parse() {
            Ok(program) => {
                let mut context = Context::default();
                evaluate(program, &mut context).unwrap();
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
            ParserError::Expected { expected, got } => panic!("Expected {} got {}", expected, got),
            ParserError::UnexpectedEOF => panic!("Unexpected end of file"),
            ParserError::InvalidDirective => panic!("Invalid directive"),
            ParserError::Syntax(message) => panic!("{}", message),
            ParserError::UndefinedConstraint => panic!("Undefined constraint"),
            ParserError::InvalidStringPattern(pattern) => {
                panic!("Invalid string pattern '{}'", pattern)
            }
        }
    }
}
