// use core::panic;
// use std::path::Path;
// use std::path::PathBuf;

// use testa_core::ast::{
//     Attribute, ConstraintKind, DataTypeKind, ExpressionKind, InfixOperator, PatternChar,
//     PatternElement, PrefixOperator, Statement, ExpressionStatemnt
// };
// use testa_core::parser::Parser;
// use testa_core::parser::error::ParserError;

// #[test]
// fn parse_tagged_template() {
//     let program = "#[abstract] template User {}";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             assert_eq!(program.0.len(), 1);
//             let Statement::Template {
//                 attributes,
//                 name,
//                 body,
//                 parent,
//                 ..
//             } = &program.0[0]
//             else {
//                 panic!("Expected Template statement");
//             };

//             let Attribute::Flag(attr_name, _) = &attributes[0] else {
//                 panic!("expected flag attribute");
//             };
//             assert_eq!(attr_name, "abstract");
//             assert_eq!(name, "User");
//             assert!(body.is_empty());
//             assert!(parent.is_none());
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_tagged_template_field() {
//     let program = "template User { #[primary_key] #[unique] id = string; }";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Template { body, .. } = &program.0[0] else {
//                 panic!("Expected Template statement");
//             };
//             assert_eq!(body.len(), 1);
//             assert_eq!(body[0].name, "id");
//             let attributes = &body[0].attributes;
//             assert_eq!(attributes.len(), 2);
//             let Attribute::Flag(name, _) = &attributes[0] else {
//                 panic!("expected flag attribute");
//             };
//             assert_eq!(name, "primary_key");
//             let Attribute::Flag(name, _) = &attributes[1] else {
//                 panic!("expected flag attribute");
//             };
//             assert_eq!(name, "unique");

//             assert!(!body[0].overridable);
//             match &body[0].value.kind {
//                 ExpressionKind::Type(dt) => assert!(matches!(dt.kind, DataTypeKind::Str)),
//                 _ => panic!("Expected Type expression"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_empty_template() {
//     let program = "template User {}";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             assert_eq!(program.0.len(), 1);
//             let Statement::Template {
//                 attributes,
//                 name,
//                 body,
//                 parent,
//                 ..
//             } = &program.0[0]
//             else {
//                 panic!("Expected Template statement");
//             };
//             assert!(attributes.is_empty());
//             assert_eq!(name, "User");
//             assert!(body.is_empty());
//             assert!(parent.is_none());
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_single_field_template() {
//     let program = "template User { name = string; }";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Template {
//                 name,
//                 body,
//                 attributes,
//                 parent,
//                 ..
//             } = &program.0[0]
//             else {
//                 panic!("Expected Template statement");
//             };
//             assert_eq!(name, "User");
//             assert!(attributes.is_empty());
//             assert!(parent.is_none());
//             assert_eq!(body.len(), 1);
//             assert_eq!(body[0].name, "name");
//             assert!(!body[0].overridable);
//             assert!(body[0].attributes.is_empty());
//             match &body[0].value.kind {
//                 ExpressionKind::Type(dt) => {
//                     assert!(matches!(dt.kind, DataTypeKind::Str));
//                     assert!(dt.constraints.is_none());
//                 }
//                 _ => panic!("Expected Type expression"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_template() {
//     let program =
//         "template Product : Consumable { override name = string; quantity = int; price = float; }";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Template {
//                 name,
//                 body,
//                 parent,
//                 attributes,
//                 ..
//             } = &program.0[0]
//             else {
//                 panic!("Expected Template statement");
//             };
//             assert_eq!(name, "Product");
//             assert_eq!(parent, &Some("Consumable".to_owned()));
//             assert!(attributes.is_empty());
//             assert_eq!(body.len(), 3);

//             assert_eq!(body[0].name, "name");
//             assert!(body[0].overridable);
//             match &body[0].value.kind {
//                 ExpressionKind::Type(dt) => assert!(matches!(dt.kind, DataTypeKind::Str)),
//                 _ => panic!("Expected Type expression"),
//             }

//             assert_eq!(body[1].name, "quantity");
//             assert!(!body[1].overridable);
//             match &body[1].value.kind {
//                 ExpressionKind::Type(dt) => assert!(matches!(dt.kind, DataTypeKind::Int)),
//                 _ => panic!("Expected Type expression"),
//             }

//             assert_eq!(body[2].name, "price");
//             assert!(!body[2].overridable);
//             match &body[2].value.kind {
//                 ExpressionKind::Type(dt) => assert!(matches!(dt.kind, DataTypeKind::Float)),
//                 _ => panic!("Expected Type expression"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_missing_paren_template() {
//     let program = "template Invalid { name = string;";
//     let mut parser = Parser::new(program, Path::new(""));
//     expect_missing_paren(&mut parser);
// }

// #[test]
// fn parse_anonymus_generate() {
//     let program = "@generate _ [10] { name = string; price = float; }";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Generate {
//                 template_name,
//                 body,
//                 count,
//                 ..
//             } = &program.0[0]
//             else {
//                 panic!("Expected Generate statement");
//             };
//             assert_eq!(template_name, &None);
//             assert_eq!(body.len(), 2);

//             assert_eq!(body[0].name, "name");
//             match &body[0].value.kind {
//                 ExpressionKind::Type(dt) => assert!(matches!(dt.kind, DataTypeKind::Str)),
//                 _ => panic!("Expected Type expression"),
//             }

//             assert_eq!(body[1].name, "price");
//             match &body[1].value.kind {
//                 ExpressionKind::Type(dt) => assert!(matches!(dt.kind, DataTypeKind::Float)),
//                 _ => panic!("Expected Type expression"),
//             }

//             match &count.kind {
//                 ExpressionKind::IntLiteral(v) => assert_eq!(*v, 10),
//                 _ => panic!("Expected IntLiteral"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_generate() {
//     let program = "@generate User [10];";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Generate {
//                 template_name,
//                 body,
//                 count,
//                 ..
//             } = &program.0[0]
//             else {
//                 panic!("Expected Generate statement");
//             };
//             assert_eq!(template_name, &Some("User".to_string()));
//             assert!(body.is_empty());
//             match &count.kind {
//                 ExpressionKind::IntLiteral(v) => assert_eq!(*v, 10),
//                 _ => panic!("Expected IntLiteral"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_missing_paren_generate() {
//     let program = "@generate _ [10] { name = string; price = float;";
//     let mut parser = Parser::new(program, Path::new(""));
//     expect_missing_paren(&mut parser);
// }

// #[test]
// fn parse_weighted_variant_enum() {
//     let program = "#[public] enum Role { User => 50; Admin => 10; Developer => 30; }";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Enum {
//                 name,
//                 variants,
//                 attributes,
//                 ..
//             } = &program.0[0]
//             else {
//                 panic!("Expected Enum statement");
//             };
//             assert_eq!(name, "Role");
//             assert_eq!(attributes.len(), 1);
//             let Attribute::Flag(name, _) = &attributes[0] else {
//                 panic!("expected flag attribute");
//             };
//             assert_eq!(name, "public");
//             assert_eq!(variants.len(), 3);

//             assert_eq!(variants[0].name, "User");
//             match &variants[0].weight {
//                 Some(expr) => match &expr.kind {
//                     ExpressionKind::IntLiteral(v) => assert_eq!(*v, 50),
//                     _ => panic!("Expected IntLiteral"),
//                 },
//                 None => panic!("Expected weight"),
//             }

//             assert_eq!(variants[1].name, "Admin");
//             match &variants[1].weight {
//                 Some(expr) => match &expr.kind {
//                     ExpressionKind::IntLiteral(v) => assert_eq!(*v, 10),
//                     _ => panic!("Expected IntLiteral"),
//                 },
//                 None => panic!("Expected weight"),
//             }

//             assert_eq!(variants[2].name, "Developer");
//             match &variants[2].weight {
//                 Some(expr) => match &expr.kind {
//                     ExpressionKind::IntLiteral(v) => assert_eq!(*v, 30),
//                     _ => panic!("Expected IntLiteral"),
//                 },
//                 None => panic!("Expected weight"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_empty_enum() {
//     let program = "enum Role {}";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Enum {
//                 name,
//                 variants,
//                 attributes,
//                 ..
//             } = &program.0[0]
//             else {
//                 panic!("Expected Enum statement");
//             };
//             assert_eq!(name, "Role");
//             assert!(variants.is_empty());
//             assert!(attributes.is_empty());
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_single_variant_enum() {
//     let program = "enum Role { User; }";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Enum {
//                 name,
//                 variants,
//                 attributes,
//                 ..
//             } = &program.0[0]
//             else {
//                 panic!("Expected Enum statement");
//             };
//             assert_eq!(name, "Role");
//             assert!(attributes.is_empty());
//             assert_eq!(variants.len(), 1);
//             assert_eq!(variants[0].name, "User");
//             assert!(variants[0].weight.is_none());
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_list_type() {
//     let program = "[int][range=1..=5];";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Expression(expr_stmt) = &program.0[0] else {
//                 panic!("Expected Expression statement");
//             };
//             match &expr_stmt.expression.kind {
//                 ExpressionKind::Type(dt) => {
//                     match &dt.kind {
//                         DataTypeKind::List(inner) => {
//                             assert!(matches!(inner.kind, DataTypeKind::Int));
//                         }
//                         _ => panic!("Expected List type"),
//                     }
//                     let constraints = dt.constraints.as_ref().expect("Expected constraints");
//                     assert_eq!(constraints.len(), 1);
//                     assert!(matches!(constraints[0].kind, ConstraintKind::Range));
//                     match &constraints[0].expression.kind {
//                         ExpressionKind::Infix {
//                             left,
//                             operator,
//                             right,
//                         } => {
//                             assert!(matches!(operator, InfixOperator::InclusiveRange));
//                             match &left.kind {
//                                 ExpressionKind::IntLiteral(v) => assert_eq!(*v, 1),
//                                 _ => panic!("Expected IntLiteral"),
//                             }
//                             match &right.kind {
//                                 ExpressionKind::IntLiteral(v) => assert_eq!(*v, 5),
//                                 _ => panic!("Expected IntLiteral"),
//                             }
//                         }
//                         _ => panic!("Expected Infix expression"),
//                     }
//                 }
//                 _ => panic!("Expected Type expression"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_list_expression() {
//     let program = "[1 => 5; \"John\"; true => 25];";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Expression(expr_stmt) = &program.0[0] else {
//                 panic!("Expected Expression statement");
//             };
//             match &expr_stmt.expression.kind {
//                 ExpressionKind::List(elements) => {
//                     assert_eq!(elements.len(), 3);

//                     match elements[0].value.kind {
//                         ExpressionKind::IntLiteral(v) => assert_eq!(v, 1),
//                         _ => panic!("expected IntLiteral"),
//                     }

//                     match elements[0]
//                         .weight
//                         .as_ref()
//                         .expect("expected expression")
//                         .kind
//                     {
//                         ExpressionKind::IntLiteral(v) => assert_eq!(v, 5),
//                         _ => panic!("expected IntLiteral"),
//                     }

//                     match &elements[1].value.kind {
//                         ExpressionKind::StringLiteral(v) => assert_eq!(v, "John"),
//                         kind => panic!("expected StringLiteral, got {:?}", kind),
//                     }

//                     match elements[2].value.kind {
//                         ExpressionKind::BooleanLiteral(v) => assert!(v),
//                         _ => panic!("expected BoolLiteral"),
//                     }

//                     match elements[2]
//                         .weight
//                         .as_ref()
//                         .expect("expected expression")
//                         .kind
//                     {
//                         ExpressionKind::IntLiteral(v) => assert_eq!(v, 25),
//                         _ => panic!("expected IntLiteral"),
//                     }
//                 }
//                 _ => panic!("Expected List expression"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_extended_type() {
//     let program = r#"
//             #[public]
//             type positive_int = int[range=0..=1024];
//             type even_positive_int = extend positive_int with [multiple_of=2];
//         "#;
//     let mut parser = Parser::new(program, Path::new(""));

//     match parser.parse() {
//         Ok(program) => {
//             assert_eq!(program.0.len(), 2);

//             let Statement::TypeDecl {
//                 name: name1,
//                 data_type: dt1,
//                 attributes: attr1,
//                 ..
//             } = &program.0[0]
//             else {
//                 panic!("Expected TypeDecl statement");
//             };
//             assert_eq!(name1, "positive_int");
//             assert_eq!(attr1.len(), 1);
//             let Attribute::Flag(name, _) = &attr1[0] else {
//                 panic!("expected flag attribute");
//             };
//             assert_eq!(name, "public");

//             match &dt1.kind {
//                 ExpressionKind::Type(dt) => {
//                     assert!(matches!(dt.kind, DataTypeKind::Int));
//                     let constraints = dt.constraints.as_ref().expect("Expected constraints");
//                     assert_eq!(constraints.len(), 1);
//                     assert!(matches!(constraints[0].kind, ConstraintKind::Range));
//                     match &constraints[0].expression.kind {
//                         ExpressionKind::Infix {
//                             left,
//                             operator,
//                             right,
//                         } => {
//                             assert!(matches!(operator, InfixOperator::InclusiveRange));
//                             match &left.kind {
//                                 ExpressionKind::IntLiteral(v) => assert_eq!(*v, 0),
//                                 _ => panic!("Expected IntLiteral"),
//                             }
//                             match &right.kind {
//                                 ExpressionKind::IntLiteral(v) => assert_eq!(*v, 1024),
//                                 _ => panic!("Expected IntLiteral"),
//                             }
//                         }
//                         _ => panic!("Expected Infix expression"),
//                     }
//                 }
//                 _ => panic!("Expected Type expression"),
//             }

//             let Statement::TypeDecl {
//                 name: name2,
//                 data_type: dt2,
//                 attributes: attr2,
//                 ..
//             } = &program.0[1]
//             else {
//                 panic!("Expected TypeDecl statement");
//             };
//             assert_eq!(name2, "even_positive_int");
//             assert!(attr2.is_empty());
//             match &dt2.kind {
//                 ExpressionKind::Type(dt) => {
//                     match &dt.kind {
//                         DataTypeKind::Custom(s) => assert_eq!(s, "positive_int"),
//                         _ => panic!("Expected Custom type"),
//                     }
//                     let constraints = dt.constraints.as_ref().expect("Expected constraints");
//                     assert_eq!(constraints.len(), 1);
//                     assert!(matches!(constraints[0].kind, ConstraintKind::MultipleOf));
//                     match &constraints[0].expression.kind {
//                         ExpressionKind::IntLiteral(v) => assert_eq!(*v, 2),
//                         _ => panic!("Expected IntLiteral"),
//                     }
//                 }
//                 _ => panic!("Expected Type expression"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_enum() {
//     let program = "enum Role { User; Admin; Moderator; }";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Enum {
//                 name,
//                 variants,
//                 attributes,
//                 ..
//             } = &program.0[0]
//             else {
//                 panic!("Expected Enum statement");
//             };
//             assert_eq!(name, "Role");
//             assert!(attributes.is_empty());
//             assert_eq!(variants.len(), 3);
//             assert_eq!(variants[0].name, "User");
//             assert!(variants[0].weight.is_none());
//             assert_eq!(variants[1].name, "Admin");
//             assert!(variants[1].weight.is_none());
//             assert_eq!(variants[2].name, "Moderator");
//             assert!(variants[2].weight.is_none());
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_missing_paren_enum() {
//     let program = "enum Role { User; Admin; Moderator;";
//     let mut parser = Parser::new(program, Path::new(""));
//     expect_missing_paren(&mut parser);
// }

// #[test]
// fn parse_infix_expression() {
//     let program = "2 + 10 * 20;";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Expression(expr_stmt) = &program.0[0] else {
//                 panic!("Expected Expression statement");
//             };
//             match &expr_stmt.expression.kind {
//                 ExpressionKind::Infix {
//                     left,
//                     operator,
//                     right,
//                 } => {
//                     assert!(matches!(operator, InfixOperator::Plus));
//                     match &left.kind {
//                         ExpressionKind::IntLiteral(v) => assert_eq!(*v, 2),
//                         _ => panic!("Expected IntLiteral"),
//                     }
//                     match &right.kind {
//                         ExpressionKind::Infix {
//                             left: l2,
//                             operator: op2,
//                             right: r2,
//                         } => {
//                             assert!(matches!(op2, InfixOperator::Multiply));
//                             match &l2.kind {
//                                 ExpressionKind::IntLiteral(v) => assert_eq!(*v, 10),
//                                 _ => panic!("Expected IntLiteral"),
//                             }
//                             match &r2.kind {
//                                 ExpressionKind::IntLiteral(v) => assert_eq!(*v, 20),
//                                 _ => panic!("Expected IntLiteral"),
//                             }
//                         }
//                         _ => panic!("Expected Infix expression"),
//                     }
//                 }
//                 _ => panic!("Expected Infix expression"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_grouped_expression() {
//     let program = "(2 + 3) * 5;";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Expression(expr_stmt) = &program.0[0] else {
//                 panic!("Expected Expression statement");
//             };
//             match &expr_stmt.expression.kind {
//                 ExpressionKind::Infix {
//                     left,
//                     operator,
//                     right,
//                 } => {
//                     assert!(matches!(operator, InfixOperator::Multiply));
//                     match &left.kind {
//                         ExpressionKind::Infix {
//                             left: l2,
//                             operator: op2,
//                             right: r2,
//                         } => {
//                             assert!(matches!(op2, InfixOperator::Plus));
//                             match &l2.kind {
//                                 ExpressionKind::IntLiteral(v) => assert_eq!(*v, 2),
//                                 _ => panic!("Expected IntLiteral"),
//                             }
//                             match &r2.kind {
//                                 ExpressionKind::IntLiteral(v) => assert_eq!(*v, 3),
//                                 _ => panic!("Expected IntLiteral"),
//                             }
//                         }
//                         _ => panic!("Expected Infix expression"),
//                     }
//                     match &right.kind {
//                         ExpressionKind::IntLiteral(v) => assert_eq!(*v, 5),
//                         _ => panic!("Expected IntLiteral"),
//                     }
//                 }
//                 _ => panic!("Expected Infix expression"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_missing_paren_expression() {
//     let program = "(2 << 3 & 5 >> 1;";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(_) => panic!("Program should have returned err."),
//         Err(err) => match err {
//             ParserError::Expected {
//                 expected,
//                 got,
//                 span: _,
//             } => {
//                 assert_eq!(expected, ")".to_string());
//                 assert_eq!(got, ";".to_string())
//             }
//             _ => panic!("Program should have returned expected error"),
//         },
//     }
// }

// #[test]
// fn parse_prefix_expression() {
//     let program = "-5; !true;";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             assert_eq!(program.0.len(), 2);

//             let Statement::Expression(expr_stmt1) = &program.0[0] else {
//                 panic!("Expected Expression statement");
//             };
//             match &expr_stmt1.expression.kind {
//                 ExpressionKind::Prefix {
//                     operator,
//                     expression,
//                 } => {
//                     assert!(matches!(operator, PrefixOperator::Negative));
//                     match &expression.kind {
//                         ExpressionKind::IntLiteral(v) => assert_eq!(*v, 5),
//                         _ => panic!("Expected IntLiteral"),
//                     }
//                 }
//                 _ => panic!("Expected Prefix expression"),
//             }

//             let Statement::Expression(expr_stmt2) = &program.0[1] else {
//                 panic!("Expected Expression statement");
//             };
//             match &expr_stmt2.expression.kind {
//                 ExpressionKind::Prefix {
//                     operator,
//                     expression,
//                 } => {
//                     assert!(matches!(operator, PrefixOperator::LogicalNegate));
//                     match expression.kind {
//                         ExpressionKind::BooleanLiteral(v) => assert!(v),
//                         _ => panic!("Expected BooleanLiteral"),
//                     }
//                 }
//                 _ => panic!("Expected Prefix expression"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_directive() {
//     let program = "@output csv;";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::OutputDirective {
//                 argument, options, ..
//             } = &program.0[0]
//             else {
//                 panic!("Expected OutputDirective statement");
//             };
//             assert_eq!(argument, "csv");
//             assert!(options.is_empty());
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_output_path() {
//     let program = "@output_path \"./example.csv\";";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::OutputPathDirective { argument, .. } = &program.0[0] else {
//                 panic!("Expected OutputPathDirective statement");
//             };
//             assert_eq!(argument, &PathBuf::from("./example.csv"));
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_pattern_dollar_case() {
//     let program = "string_pattern \"dollar$$$$$$$$$$$$$$$$${aa}\";";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Expression(expr_stmt) = &program.0[0] else {
//                 panic!("Expected Expression statement");
//             };
//             match &expr_stmt.expression.kind {
//                 ExpressionKind::StringPattern(elements) => {
//                     assert_eq!(elements.len(), 2);
//                     match &elements[0] {
//                         PatternElement::Literal(s, _) => assert_eq!(s, "dollar$$$$$$$$$$$$$$$$"),
//                         _ => panic!("Expected Literal"),
//                     }
//                     match &elements[1] {
//                         PatternElement::RepeatChar {
//                             ch,
//                             count,
//                             count_expression,
//                             ..
//                         } => {
//                             assert!(matches!(ch, PatternChar::Lowercase));
//                             assert_eq!(*count, 2);
//                             assert!(count_expression.is_none());
//                         }
//                         _ => panic!("Expected RepeatChar"),
//                     }
//                 }
//                 _ => panic!("Expected StringPattern expression"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_string_pattern() {
//     let program = "string_pattern \"testa$}${aaa[10]}john${A[25]##[13]}\";";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::Expression(ExpressionStatemnt { expression, .. }) = &program.0[0] else {
//                 panic!("Expected Expression statement");
//             };
//             match &expression.kind {
//                 ExpressionKind::StringPattern(elements) => {
//                     assert_eq!(elements.len(), 5);

//                     match &elements[0] {
//                         PatternElement::Literal(s, _) => assert_eq!(s, "testa$}"),
//                         _ => panic!("Expected Literal"),
//                     }

//                     match &elements[1] {
//                         PatternElement::RepeatChar {
//                             ch,
//                             count,
//                             count_expression,
//                             ..
//                         } => {
//                             assert!(matches!(ch, PatternChar::Lowercase));
//                             assert_eq!(*count, 3);
//                             match count_expression {
//                                 Some(expr) => match &expr.kind {
//                                     ExpressionKind::IntLiteral(v) => assert_eq!(*v, 10),
//                                     _ => panic!("Expected IntLiteral"),
//                                 },
//                                 None => panic!("Expected count expression"),
//                             }
//                         }
//                         _ => panic!("Expected RepeatChar"),
//                     }

//                     match &elements[2] {
//                         PatternElement::Literal(s, _) => assert_eq!(s, "john"),
//                         _ => panic!("Expected Literal"),
//                     }

//                     match &elements[3] {
//                         PatternElement::RepeatChar {
//                             ch,
//                             count,
//                             count_expression,
//                             ..
//                         } => {
//                             assert!(matches!(ch, PatternChar::Uppercase));
//                             assert_eq!(*count, 1);
//                             match count_expression {
//                                 Some(expr) => match &expr.kind {
//                                     ExpressionKind::IntLiteral(v) => assert_eq!(*v, 25),
//                                     _ => panic!("Expected IntLiteral"),
//                                 },
//                                 None => panic!("Expected count expression"),
//                             }
//                         }
//                         _ => panic!("Expected RepeatChar"),
//                     }

//                     match &elements[4] {
//                         PatternElement::RepeatChar {
//                             ch,
//                             count,
//                             count_expression,
//                             ..
//                         } => {
//                             assert!(matches!(ch, PatternChar::Digit));
//                             assert_eq!(*count, 2);
//                             match count_expression {
//                                 Some(expr) => match &expr.kind {
//                                     ExpressionKind::IntLiteral(v) => assert_eq!(*v, 13),
//                                     _ => panic!("Expected IntLiteral"),
//                                 },
//                                 None => panic!("Expected count expression"),
//                             }
//                         }
//                         _ => panic!("Expected RepeatChar"),
//                     }
//                 }
//                 _ => panic!("Expected StringPattern expression"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// #[test]
// fn parse_directive_options() {
//     let program = "@output csv { delimiter = \";\"; }";
//     let mut parser = Parser::new(program, Path::new(""));
//     match parser.parse() {
//         Ok(program) => {
//             let Statement::OutputDirective {
//                 argument, options, ..
//             } = &program.0[0]
//             else {
//                 panic!("Expected OutputDirective statement");
//             };
//             assert_eq!(argument, "csv");
//             assert_eq!(options.len(), 1);
//             assert_eq!(options[0].name, "delimiter");
//             match &options[0].value.kind {
//                 ExpressionKind::StringLiteral(s) => assert_eq!(s, ";"),
//                 _ => panic!("Expected StringLiteral"),
//             }
//         }
//         Err(err) => handle_error(err),
//     }
// }

// fn expect_missing_paren(parser: &mut Parser) {
//     match parser.parse() {
//         Ok(_) => panic!("Program should have returned err."),
//         Err(err) => match err {
//             ParserError::UnexpectedEof { .. } => {}
//             err => panic!(
//                 "Program should have returned unexpected EOF instead of {:?}",
//                 err
//             ),
//         },
//     }
// }

// fn handle_error(error: ParserError) {
//     match error {
//         ParserError::Expected {
//             span,
//             expected,
//             got,
//         } => {
//             panic!(
//                 "expected '{}' got '{}' in {}",
//                 expected,
//                 got,
//                 span.to_string()
//             )
//         }
//         ParserError::InvalidDirective { span } => panic!("{}", span.to_string()),
//         ParserError::UnexpectedEof { span } => {
//             panic!("{}", span.to_string())
//         }
//         ParserError::Syntax { span, message } => {
//             panic!("{}: {}", span.to_string(), message)
//         }
//         ParserError::UndefinedConstraint { span } => panic!("{}", span.to_string()),
//         ParserError::InvalidStringPattern { span, pattern } => {
//             panic!("{}: {}", span.to_string(), pattern)
//         }
//         ParserError::InvalidAttribute { span, token } => panic!("{}: {}", span.to_string(), token),
//         ParserError::LexerError(lexer_error) => panic!("{}", lexer_error.to_string()),
//     }
// }
