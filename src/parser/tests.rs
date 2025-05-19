#[cfg(test)]
mod parser_tests {
    use core::panic;
    use std::vec;

    use crate::parser::{ast::{Expression, ExpressionKind, ExpressionStatemnt, InfixOperator, Statement}, parser::Parser, parser_error::ParserError};


    #[test]
    fn parse_empty_enum() {
        let program = "enum Role {}";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(program.0, vec![Statement::Enum { name: "Role".to_string(), variants: vec![] }]);
            },
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_single_variant_enum() {
        let program = "enum Role { User }";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(program.0, vec![Statement::Enum { 
                    name: "Role".to_string(), 
                    variants: vec!["User".to_string()] 
                }]);
            },
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_enum() {
        let program = "enum Role { User, Admin, Moderator }";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(program.0, vec![Statement::Enum { 
                    name: "Role".to_string(), 
                    variants: vec!["User".to_string(), "Admin".to_string(), "Moderator".to_string()] 
                }]);
            },
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_infix_expression() {
        let program = "2 + 10 * 20;";
        let mut parser = Parser::new(program);
        let solution = ExpressionStatemnt {
            expression: Expression::new(ExpressionKind::Infix{
                left: Box::new(Expression::new(ExpressionKind::IntLiteral(2), 0, 1)),
                operator: InfixOperator::Plus,
                right: Box::new(Expression::new(ExpressionKind::Infix { 
                    left: Box::new(Expression::new(ExpressionKind::IntLiteral(10), 4, 2)), 
                    operator: InfixOperator::Multiply, 
                    right: Box::new(Expression::new(ExpressionKind::IntLiteral(20), 9, 2))
                }, 4, 7)),
            }, 0, 11)
        };
        match parser.parse() {
            Ok(program) => {
                assert_eq!(program.0, vec![Statement::Expression(solution)]);
            },
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_output_directive() {
        let program = "@output csv;";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(program.0, vec![Statement::OutputDirective { 
                    argument: "csv".to_string(), 
                    options: vec![]
                }]);
            },
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_output_directive_options() {
        let program = "@output csv { delimiter = \";\"; }";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(program.0, vec![Statement::OutputDirective { 
                    argument: "csv".to_string(), 
                    options: vec![("delimiter".to_string(), ";".to_string())]
                }]);
            },
            Err(err) => handle_error(err),
        }
    }

    fn handle_error(error: ParserError) {
        match error {
            ParserError::Expected { expected, got } => panic!("Expected {} got {}", expected, got),
            ParserError::UnexpectedEOF => panic!("Unexpected end of file"),
            ParserError::Syntax(message) => panic!("{}", message),
        }
    }

}