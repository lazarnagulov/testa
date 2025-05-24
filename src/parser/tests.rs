#[cfg(test)]
mod parser_tests {
    use core::panic;
    use std::vec;

    use crate::parser::{
        ast::{
            DataType, Expression, ExpressionKind, ExpressionStatemnt, Field, InfixOperator,
            PrefixOperator, Program, Statement, Variant,
        },
        parser::Parser,
        parser_error::ParserError,
    };

    #[test]
    fn parse_empty_template() {
        let program = "template User {}";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(
                    program.0,
                    vec![Statement::Template {
                        name: "User".to_string(),
                        body: vec![]
                    }]
                );
            }
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_single_field_template() {
        let program = "template User { name = string; }";
        let mut parser = Parser::new(program);
        let field = Field::new(
            "name".to_string(),
            Expression::new(ExpressionKind::Type(DataType::Str), 23, 6),
        );
        match parser.parse() {
            Ok(program) => {
                assert_eq!(
                    program.0,
                    vec![Statement::Template {
                        name: "User".to_string(),
                        body: vec![field]
                    }]
                );
            }
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_template() {
        let program = "template Product { name = string; quantity = int; price = float; }";
        let mut parser = Parser::new(program);
        let name_field = Field::new(
            "name".to_string(),
            Expression::new(ExpressionKind::Type(DataType::Str), 26, 6),
        );
        let quantity_field = Field::new(
            "quantity".to_string(),
            Expression::new(ExpressionKind::Type(DataType::Int), 45, 3),
        );
        let price_field = Field::new(
            "price".to_string(),
            Expression::new(ExpressionKind::Type(DataType::Float), 58, 5),
        );
        match parser.parse() {
            Ok(program) => {
                assert_eq!(
                    program.0,
                    vec![Statement::Template {
                        name: "Product".to_string(),
                        body: vec![name_field, quantity_field, price_field]
                    }]
                );
            }
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_missing_paren_template() {
        let program = "template Invalid { name = string;";
        let mut parser = Parser::new(program);
        expect_missing_paren(&mut parser);
    }

    #[test]
    fn parse_anonymus_generate() {
        let program = "generate _ [10] { name = string; price = float; }";
        let mut parser = Parser::new(program);
        let name_field = Field::new(
            "name".to_string(),
            Expression::new(ExpressionKind::Type(DataType::Str), 25, 6),
        );
        let price_field = Field::new(
            "price".to_string(),
            Expression::new(ExpressionKind::Type(DataType::Float), 41, 5),
        );
        match parser.parse() {
            Ok(program) => {
                assert_eq!(
                    program.0,
                    vec![Statement::Generate {
                        template_name: None,
                        body: vec![name_field, price_field],
                        count: Expression::new(ExpressionKind::IntLiteral(10), 12, 2)
                    }]
                );
            }
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_generate() {
        let program = "generate User [10];";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(
                    program.0,
                    vec![Statement::Generate {
                        template_name: Some("User".to_string()),
                        body: vec![],
                        count: Expression::new(ExpressionKind::IntLiteral(10), 15, 2)
                    }]
                );
            }
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_missing_paren_generate() {
        let program = "generate _ [10] { name = string; price = float;";
        let mut parser = Parser::new(program);
        expect_missing_paren(&mut parser);
    }

    #[test]
    fn parse_weighted_variant_enum() {
        let program = "enum Role { User => 50, Admin => 10, Developer => 30 }";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(
                    program.0,
                    vec![Statement::Enum {
                        name: "Role".to_string(),
                        variants: vec![
                            Variant::new(
                                "User".to_string(),
                                Some(Expression::new(ExpressionKind::IntLiteral(50), 20, 2))
                            ),
                            Variant::new(
                                "Admin".to_string(),
                                Some(Expression::new(ExpressionKind::IntLiteral(10), 33, 2))
                            ),
                            Variant::new(
                                "Developer".to_string(),
                                Some(Expression::new(ExpressionKind::IntLiteral(30), 50, 2))
                            ),
                        ]
                    }]
                );
            }
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_empty_enum() {
        let program = "enum Role {}";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(
                    program.0,
                    vec![Statement::Enum {
                        name: "Role".to_string(),
                        variants: vec![]
                    }]
                );
            }
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_single_variant_enum() {
        let program = "enum Role { User }";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(
                    program.0,
                    vec![Statement::Enum {
                        name: "Role".to_string(),
                        variants: vec![Variant::new("User".to_string(), None)]
                    }]
                );
            }
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_enum() {
        let program = "enum Role { User, Admin, Moderator }";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(
                    program.0,
                    vec![Statement::Enum {
                        name: "Role".to_string(),
                        variants: vec![
                            Variant::new("User".to_string(), None),
                            Variant::new("Admin".to_string(), None),
                            Variant::new("Moderator".to_string(), None),
                        ]
                    }]
                );
            }
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_missing_paren_enum() {
        let program = "enum Role { User, Admin, Moderator,";
        let mut parser = Parser::new(program);
        expect_missing_paren(&mut parser);
    }

    #[test]
    fn parse_infix_expression() {
        let program = "2 + 10 * 20;";
        let mut parser = Parser::new(program);
        let solution = ExpressionStatemnt {
            expression: Expression::new(
                ExpressionKind::Infix {
                    left: Box::new(Expression::new(ExpressionKind::IntLiteral(2), 0, 1)),
                    operator: InfixOperator::Plus,
                    right: Box::new(Expression::new(
                        ExpressionKind::Infix {
                            left: Box::new(Expression::new(ExpressionKind::IntLiteral(10), 4, 2)),
                            operator: InfixOperator::Multiply,
                            right: Box::new(Expression::new(ExpressionKind::IntLiteral(20), 9, 2)),
                        },
                        4,
                        7,
                    )),
                },
                0,
                11,
            ),
        };
        expect_expression(&mut parser, solution);
    }

    #[test]
    fn parse_grouped_expression() {
        let program = "(2 + 3) * 5;";
        let mut parser = Parser::new(program);
        let solution = ExpressionStatemnt {
            expression: Expression::new(
                ExpressionKind::Infix {
                    left: Box::new(Expression::new(
                        ExpressionKind::Infix {
                            left: Box::new(Expression::new(ExpressionKind::IntLiteral(2), 1, 1)),
                            operator: InfixOperator::Plus,
                            right: Box::new(Expression::new(ExpressionKind::IntLiteral(3), 5, 1)),
                        },
                        0,
                        7,
                    )),
                    operator: InfixOperator::Multiply,
                    right: Box::new(Expression::new(ExpressionKind::IntLiteral(5), 10, 1)),
                },
                0,
                11,
            ),
        };
        expect_expression(&mut parser, solution);
    }

    #[test]
    fn parse_missing_paren_expression() {
        let program = "(2 << 3 & 5 >> 1;";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(_) => panic!("Program should have returned err."),
            Err(err) => match err {
                ParserError::Expected { expected, got } => {
                    assert_eq!(expected, ")".to_string());
                    assert_eq!(got, ";".to_string())
                }
                _ => panic!("Program should have returned exprected error"),
            },
        }
    }

    #[test]
    fn parse_prefix_expression() {
        let program = "-5; !true;";
        let mut parser = Parser::new(program);
        let negative_statement = Statement::Expression(ExpressionStatemnt {
            expression: Expression::new(
                ExpressionKind::Prefix {
                    operator: PrefixOperator::Negative,
                    expression: Box::new(Expression::new(ExpressionKind::IntLiteral(5), 1, 1)),
                },
                0,
                2,
            ),
        });
        let logical_not_statement = Statement::Expression(ExpressionStatemnt {
            expression: Expression::new(
                ExpressionKind::Prefix {
                    operator: PrefixOperator::LogicalNegate,
                    expression: Box::new(Expression::new(
                        ExpressionKind::BooleanLiteral(true),
                        5,
                        4,
                    )),
                },
                4,
                5,
            ),
        });
        let program = Program(vec![negative_statement, logical_not_statement]);
        expect_program(&mut parser, program);
    }

    #[test]
    fn parse_directive() {
        let program = "@output csv;";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(
                    program.0,
                    vec![Statement::OutputDirective {
                        argument: "csv".to_string(),
                        options: vec![]
                    }]
                );
            }
            Err(err) => handle_error(err),
        }
    }

    #[test]
    fn parse_directive_options() {
        let program = "@output csv { delimiter = \";\"; }";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(
                    program.0,
                    vec![Statement::OutputDirective {
                        argument: "csv".to_string(),
                        options: vec![Field::new(
                            "delimiter".to_string(),
                            Expression::new(ExpressionKind::StringLiteral(";".to_string()), 26, 3)
                        )]
                    }]
                );
            }
            Err(err) => handle_error(err),
        }
    }

    fn expect_missing_paren(parser: &mut Parser) {
        match parser.parse() {
            Ok(_) => panic!("Program should have returned err."),
            Err(err) => match err {
                ParserError::UnexpectedEOF => {}
                err => panic!(
                    "Program should have returned unexpected EOF instead of {:?}",
                    err
                ),
            },
        }
    }

    fn expect_program(parser: &mut Parser, program: Program) {
        match parser.parse() {
            Ok(p) => assert_eq!(p, program),
            Err(err) => handle_error(err),
        }
    }

    fn expect_expression(parser: &mut Parser, expression: ExpressionStatemnt) {
        match parser.parse() {
            Ok(program) => {
                assert_eq!(program.0, vec![Statement::Expression(expression)]);
            }
            Err(err) => handle_error(err),
        }
    }

    fn handle_error(error: ParserError) {
        match error {
            ParserError::Expected { expected, got } => panic!("Expected {} got {}", expected, got),
            ParserError::UnexpectedEOF => panic!("Unexpected end of file"),
            ParserError::InvalidDirective => panic!("Invalid directive"),
            ParserError::Syntax(message) => panic!("{}", message),
        }
    }
}
