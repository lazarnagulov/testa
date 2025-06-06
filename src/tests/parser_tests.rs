use core::panic;
use std::{path::PathBuf, vec};

use crate::core::ast::nodes::{
    Attribute, ConstraintExpression, ConstraintKind, DataType, DataTypeKind, Element, Expression,
    ExpressionKind, ExpressionStatemnt, Field, InfixOperator, PatternChar, PatternElement,
    PrefixOperator, Program, Statement, Variant,
};
use crate::core::parser::{Parser, parser_error::ParserError};

#[test]
fn parse_tagged_template() {
    let program = "#[abstract] template User {}";
    let mut parser = Parser::new(program);
    match parser.parse() {
        Ok(program) => {
            assert_eq!(
                program.0,
                vec![Statement::Template {
                    attributes: vec![Attribute::new("abstract")],
                    name: "User".to_string(),
                    body: vec![],
                    parent: None,
                }]
            );
        }
        Err(err) => handle_error(err),
    }
}

#[test]
fn parse_tagged_template_field() {
    let program = "template User { #[primary_key] #[unique] id = string; }";
    let mut parser = Parser::new(program);
    match parser.parse() {
        Ok(program) => {
            assert_eq!(
                program.0,
                vec![Statement::Template {
                    attributes: Vec::new(),
                    name: "User".to_string(),
                    body: vec![Field::new(
                        "id".to_owned(),
                        Expression::new(
                            ExpressionKind::Type(DataType::new(DataTypeKind::Str, None)),
                            46,
                            6
                        ),
                        false,
                        vec![Attribute::new("primary_key"), Attribute::new("unique")]
                    )],
                    parent: None,
                }]
            );
        }
        Err(err) => handle_error(err),
    }
}

#[test]
fn parse_empty_template() {
    let program = "template User {}";
    let mut parser = Parser::new(program);
    match parser.parse() {
        Ok(program) => {
            assert_eq!(
                program.0,
                vec![Statement::Template {
                    attributes: Vec::new(),
                    name: "User".to_string(),
                    body: vec![],
                    parent: None,
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
        Expression::new(
            ExpressionKind::Type(DataType::new(DataTypeKind::Str, None)),
            23,
            6,
        ),
        false,
        Vec::new(),
    );
    match parser.parse() {
        Ok(program) => {
            assert_eq!(
                program.0,
                vec![Statement::Template {
                    name: "User".to_string(),
                    attributes: Vec::new(),
                    body: vec![field],
                    parent: None,
                }]
            );
        }
        Err(err) => handle_error(err),
    }
}

#[test]
fn parse_template() {
    let program =
        "template Product : Consumable { override name = string; quantity = int; price = float; }";
    let mut parser = Parser::new(program);
    let name_field = Field::new(
        "name".to_string(),
        Expression::new(
            ExpressionKind::Type(DataType::new(DataTypeKind::Str, None)),
            48,
            6,
        ),
        true,
        Vec::new(),
    );
    let quantity_field = Field::new(
        "quantity".to_string(),
        Expression::new(
            ExpressionKind::Type(DataType::new(DataTypeKind::Int, None)),
            67,
            3,
        ),
        false,
        Vec::new(),
    );
    let price_field = Field::new(
        "price".to_string(),
        Expression::new(
            ExpressionKind::Type(DataType::new(DataTypeKind::Float, None)),
            80,
            5,
        ),
        false,
        Vec::new(),
    );
    match parser.parse() {
        Ok(program) => {
            assert_eq!(
                program.0,
                vec![Statement::Template {
                    name: "Product".to_string(),
                    body: vec![name_field, quantity_field, price_field],
                    attributes: Vec::new(),
                    parent: Some("Consumable".to_owned()),
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
    let program = "@generate _ [10] { name = string; price = float; }";
    let mut parser = Parser::new(program);
    let name_field = Field::new(
        "name".to_string(),
        Expression::new(
            ExpressionKind::Type(DataType::new(DataTypeKind::Str, None)),
            26,
            6,
        ),
        false,
        Vec::new(),
    );
    let price_field = Field::new(
        "price".to_string(),
        Expression::new(
            ExpressionKind::Type(DataType::new(DataTypeKind::Float, None)),
            42,
            5,
        ),
        false,
        Vec::new(),
    );
    match parser.parse() {
        Ok(program) => {
            assert_eq!(
                program.0,
                vec![Statement::Generate {
                    template_name: None,
                    body: vec![name_field, price_field],
                    count: Expression::new(ExpressionKind::IntLiteral(10), 13, 2)
                }]
            );
        }
        Err(err) => handle_error(err),
    }
}

#[test]
fn parse_generate() {
    let program = "@generate User [10];";
    let mut parser = Parser::new(program);
    match parser.parse() {
        Ok(program) => {
            assert_eq!(
                program.0,
                vec![Statement::Generate {
                    template_name: Some("User".to_string()),
                    body: vec![],
                    count: Expression::new(ExpressionKind::IntLiteral(10), 16, 2)
                }]
            );
        }
        Err(err) => handle_error(err),
    }
}

#[test]
fn parse_missing_paren_generate() {
    let program = "@generate _ [10] { name = string; price = float;";
    let mut parser = Parser::new(program);
    expect_missing_paren(&mut parser);
}

#[test]
fn parse_weighted_variant_enum() {
    let program = "#[public] enum Role { User => 50; Admin => 10; Developer => 30; }";
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
                            Some(Expression::new(ExpressionKind::IntLiteral(50), 30, 2))
                        ),
                        Variant::new(
                            "Admin".to_string(),
                            Some(Expression::new(ExpressionKind::IntLiteral(10), 43, 2))
                        ),
                        Variant::new(
                            "Developer".to_string(),
                            Some(Expression::new(ExpressionKind::IntLiteral(30), 60, 2))
                        ),
                    ],
                    attributes: vec![Attribute::new("public")],
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
                    variants: vec![],
                    attributes: Vec::new(),
                }]
            );
        }
        Err(err) => handle_error(err),
    }
}

#[test]
fn parse_single_variant_enum() {
    let program = "enum Role { User; }";
    let mut parser = Parser::new(program);
    match parser.parse() {
        Ok(program) => {
            assert_eq!(
                program.0,
                vec![Statement::Enum {
                    name: "Role".to_string(),
                    variants: vec![Variant::new("User".to_string(), None)],
                    attributes: Vec::new(),
                }]
            );
        }
        Err(err) => handle_error(err),
    }
}

#[test]
fn parse_list_type() {
    let program = "[int][range=1..=5];";
    let mut parser = Parser::new(program);
    let expression = ExpressionStatemnt {
        expression: Expression::new(
            ExpressionKind::Type(DataType::new(
                DataTypeKind::List(Box::new(DataType::new(DataTypeKind::Int, None))),
                Some(vec![ConstraintExpression::new(
                    Expression::new(
                        ExpressionKind::Infix {
                            left: Box::new(Expression::new(ExpressionKind::IntLiteral(1), 12, 1)),
                            operator: InfixOperator::InclusiveRange,
                            right: Box::new(Expression::new(ExpressionKind::IntLiteral(5), 16, 1)),
                        },
                        12,
                        5,
                    ),
                    ConstraintKind::Range,
                )]),
            )),
            1,
            5,
        ),
    };
    expect_expression(&mut parser, expression);
}

#[test]
fn parse_list_expression() {
    let program = "[1 => 5; \"John\"; true => 25];";
    let mut parser = Parser::new(program);
    let expression = ExpressionStatemnt {
        expression: Expression::new(
            ExpressionKind::List(vec![
                Element::new(
                    Expression::new(ExpressionKind::IntLiteral(1), 1, 1),
                    Some(Expression::new(ExpressionKind::IntLiteral(5), 6, 1)),
                    1,
                    4,
                ),
                Element::new(
                    Expression::new(ExpressionKind::StringLiteral("John".to_owned()), 9, 6),
                    None,
                    9,
                    6,
                ),
                Element::new(
                    Expression::new(ExpressionKind::BooleanLiteral(true), 17, 4),
                    Some(Expression::new(ExpressionKind::IntLiteral(25), 25, 2)),
                    17,
                    8,
                ),
            ]),
            0,
            19,
        ),
    };
    expect_expression(&mut parser, expression);
}

#[test]
fn parse_extended_type() {
    let program = r#"
            #[public]
            type positive_int = int[range=0..=1024];
            type even_positive_int = extend positive_int with [multiple_of=2];
        "#;
    let mut parser = Parser::new(program);
    let data_type = Expression::new(
        ExpressionKind::Type(DataType::new(
            DataTypeKind::Int,
            Some(vec![ConstraintExpression::new(
                Expression::new(
                    ExpressionKind::Infix {
                        left: Box::new(Expression::new(ExpressionKind::IntLiteral(0), 65, 1)),
                        operator: InfixOperator::InclusiveRange,
                        right: Box::new(Expression::new(ExpressionKind::IntLiteral(1024), 69, 4)),
                    },
                    65,
                    8,
                ),
                ConstraintKind::Range,
            )]),
        )),
        0,
        0,
    );
    let extended_data_type = Expression::new(
        ExpressionKind::Type(DataType::new(
            DataTypeKind::Custom("positive_int".to_owned()),
            Some(vec![ConstraintExpression::new(
                Expression::new(ExpressionKind::IntLiteral(2), 151, 1),
                ConstraintKind::MultipleOf,
            )]),
        )),
        0,
        0,
    );

    match parser.parse() {
        Ok(program) => {
            assert_eq!(
                program.0,
                vec![
                    Statement::TypeDecl {
                        name: "positive_int".to_owned(),
                        data_type,
                        attributes: vec![Attribute::new("public")]
                    },
                    Statement::TypeDecl {
                        name: "even_positive_int".to_owned(),
                        data_type: extended_data_type,
                        attributes: Vec::new()
                    }
                ]
            );
        }
        Err(err) => handle_error(err),
    }
}

#[test]
fn parse_enum() {
    let program = "enum Role { User; Admin; Moderator; }";
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
                    ],
                    attributes: Vec::new(),
                }]
            );
        }
        Err(err) => handle_error(err),
    }
}

#[test]
fn parse_missing_paren_enum() {
    let program = "enum Role { User; Admin; Moderator;";
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
                expression: Box::new(Expression::new(ExpressionKind::BooleanLiteral(true), 5, 4)),
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
fn parse_output_path() {
    let program = "@output_path \"./example.csv\";";
    let mut parser = Parser::new(program);
    match parser.parse() {
        Ok(program) => {
            println!("{:?}", program);
            assert_eq!(
                program.0,
                vec![Statement::OutputPathDirective {
                    argument: PathBuf::from("./example.csv"),
                }]
            );
        }
        Err(err) => handle_error(err),
    }
}

#[test]
fn parse_pattern_dollar_case() {
    let program = "string_pattern \"dollar$$$$$$$$$$$$$$$$${aa}\";";
    let mut parser = Parser::new(program);
    let expression = ExpressionStatemnt {
        expression: Expression::new(
            ExpressionKind::StringPattern(vec![
                PatternElement::Literal("dollar$$$$$$$$$$$$$$$$".to_owned()),
                PatternElement::RepeatChar {
                    ch: PatternChar::Lowercase,
                    count: 2,
                    count_expression: None,
                },
            ]),
            0,
            0,
        ),
    };
    expect_expression(&mut parser, expression);
}

#[test]
fn parse_string_pattern() {
    let program = "string_pattern \"testa$}${aaa[10]}john${A[25]##[13]}\";";
    let mut parser = Parser::new(program);
    let solution = ExpressionStatemnt {
        expression: Expression::new(
            ExpressionKind::StringPattern(vec![
                PatternElement::Literal("testa$}".to_owned()),
                PatternElement::RepeatChar {
                    ch: PatternChar::Lowercase,
                    count: 3,
                    count_expression: Some(Expression::new(ExpressionKind::IntLiteral(10), 0, 2)),
                },
                PatternElement::Literal("john".to_owned()),
                PatternElement::RepeatChar {
                    ch: PatternChar::Uppercase,
                    count: 1,
                    count_expression: Some(Expression::new(ExpressionKind::IntLiteral(25), 0, 2)),
                },
                PatternElement::RepeatChar {
                    ch: PatternChar::Digit,
                    count: 2,
                    count_expression: Some(Expression::new(ExpressionKind::IntLiteral(13), 0, 2)),
                },
            ]),
            0,
            0,
        ),
    };
    expect_expression(&mut parser, solution);
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
                        Expression::new(ExpressionKind::StringLiteral(";".to_string()), 26, 3),
                        false,
                        Vec::new(),
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
        ParserError::UndefinedConstraint => panic!("Undefined constraint"),
        ParserError::InvalidStringPattern(pattern) => panic!("Invalid pattern {}", pattern),
        ParserError::InvalidAttribute(token) => panic!("Cannot put attribute on {}", token),
    }
}
