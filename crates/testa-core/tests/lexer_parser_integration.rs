use core::panic;
use std::path::PathBuf;

use testa_core::ast::{
    Attribute, ConstraintKind, DataTypeKind, ExpressionKind, ExpressionStatemnt, InfixOperator,
    PatternChar, PatternElement, PrefixOperator, Program, Statement,
};
use testa_core::lexer::Lexer;
use testa_core::parser::Parser;
use testa_core::parser::error::ParserError;

fn parse_ok(src: &str) -> Program {
    let lexer = Lexer::new(src);
    match Parser::new(lexer, src).parse() {
        Ok(program) => program,
        Err(err) => handle_error(&err[0]),
    }
}

fn expect_unexpected_eof(src: &str) {
    let lexer = Lexer::new(src);
    match Parser::new(lexer, src).parse() {
        Ok(_) => panic!("expected a parse error, but parsing succeeded"),
        Err(err) => match &err[0] {
            ParserError::UnexpectedEof { .. } => {}
            other => panic!("expected UnexpectedEof, got {:?}", other),
        },
    }
}

fn handle_error(error: &ParserError) -> ! {
    match error {
        ParserError::Expected {
            span,
            expected,
            got,
        } => {
            panic!("expected '{}' got '{}' in {}", expected, got, span)
        }
        ParserError::InvalidDirective { span } => panic!("{}", span),
        ParserError::UnexpectedEof { span } => panic!("{}", span),
        ParserError::Syntax { span, message } => panic!("{}: {}", span, message),
        ParserError::UndefinedConstraint { span } => panic!("{}", span),
        ParserError::InvalidStringPattern { span, pattern } => {
            panic!("{}: {}", span, pattern)
        }
        ParserError::InvalidAttribute { span, token } => panic!("{}: {}", span, token),
        ParserError::LexerError(lexer_error) => panic!("{}", lexer_error),
    }
}

#[test]
fn parse_empty_template() {
    let program = parse_ok("template User {}");
    assert_eq!(program.0.len(), 1);

    let Statement::Template {
        attributes,
        name,
        body,
        parent_name,
        ..
    } = &program.0[0]
    else {
        panic!("expected Template statement");
    };
    assert!(attributes.is_empty());
    assert_eq!(name, "User");
    assert!(body.is_empty());
    assert!(parent_name.is_none());
}

#[test]
fn parse_tagged_template() {
    let program = parse_ok("#[abstract] template User {}");
    assert_eq!(program.0.len(), 1);

    let Statement::Template {
        attributes,
        name,
        body,
        parent_name,
        ..
    } = &program.0[0]
    else {
        panic!("expected Template statement");
    };
    let Attribute::Flag(attr_name, _) = &attributes[0] else {
        panic!("expected Flag attribute");
    };
    assert_eq!(attr_name, "abstract");
    assert_eq!(name, "User");
    assert!(body.is_empty());
    assert!(parent_name.is_none());
}

#[test]
fn parse_tagged_template_field() {
    let program = parse_ok("template User { #[primary_key] #[unique] id = string; }");

    let Statement::Template { body, .. } = &program.0[0] else {
        panic!("expected Template statement");
    };
    assert_eq!(body.len(), 1);
    assert_eq!(body[0].name, "id");
    assert!(!body[0].overridable);

    let [Attribute::Flag(first, _), Attribute::Flag(second, _)] = body[0].attributes.as_slice()
    else {
        panic!("expected two Flag attributes");
    };
    assert_eq!(first, "primary_key");
    assert_eq!(second, "unique");

    let ExpressionKind::Type(dt) = &body[0].value.kind else {
        panic!("expected Type expression");
    };
    assert!(matches!(dt.kind, DataTypeKind::Str));
}

#[test]
fn parse_single_field_template() {
    let program = parse_ok("template User { name = string; }");

    let Statement::Template {
        name,
        body,
        attributes,
        parent_name,
        ..
    } = &program.0[0]
    else {
        panic!("expected Template statement");
    };
    assert_eq!(name, "User");
    assert!(attributes.is_empty());
    assert!(parent_name.is_none());
    assert_eq!(body.len(), 1);

    let field = &body[0];
    assert_eq!(field.name, "name");
    assert!(!field.overridable);
    assert!(field.attributes.is_empty());

    let ExpressionKind::Type(dt) = &field.value.kind else {
        panic!("expected Type expression");
    };
    assert!(matches!(dt.kind, DataTypeKind::Str));
    assert!(dt.constraints.is_none());
}

#[test]
fn parse_template() {
    let program = parse_ok(
        "template Product : Consumable { override name = string; quantity = int; price = float; }",
    );

    let Statement::Template {
        name,
        body,
        parent_name,
        attributes,
        ..
    } = &program.0[0]
    else {
        panic!("expected Template statement");
    };
    assert_eq!(name, "Product");
    assert_eq!(parent_name.as_deref(), Some("Consumable"));
    assert!(attributes.is_empty());
    assert_eq!(body.len(), 3);

    let cases = [
        ("name", true, DataTypeKind::Str),
        ("quantity", false, DataTypeKind::Int),
        ("price", false, DataTypeKind::Float),
    ];
    for (field, (expected_name, expected_override, expected_kind)) in body.iter().zip(&cases) {
        assert_eq!(field.name, *expected_name);
        assert_eq!(field.overridable, *expected_override);
        let ExpressionKind::Type(dt) = &field.value.kind else {
            panic!("expected Type expression for field '{}'", field.name);
        };
        assert!(
            matches!(&dt.kind, k if std::mem::discriminant(k) == std::mem::discriminant(expected_kind))
        );
    }
}

#[test]
fn parse_missing_paren_template() {
    expect_unexpected_eof("template Invalid { name = string;");
}

#[test]
fn parse_reference() {
    let program = parse_ok("template Course { instructor = ref Instructor.id; }");

    let Statement::Template { name, body, .. } = &program.0[0] else {
        panic!("expected Template statement");
    };

    assert_eq!(name, "Course");
    assert_eq!(body.len(), 1);
    assert_eq!(body.first().unwrap().name, "instructor");
    let field = body.first().unwrap();

    let ExpressionKind::Reference { template, field } = &field.value.kind else {
        panic!("expected reference expression");
    };

    assert_eq!(template, "Instructor");
    assert_eq!(field, "id");
}

#[test]
fn parse_generate() {
    let program = parse_ok("@generate User [10];");

    let Statement::Generate {
        template_name,
        body,
        count,
        ..
    } = &program.0[0]
    else {
        panic!("expected Generate statement");
    };
    assert_eq!(template_name.as_deref(), Some("User"));
    assert!(body.is_empty());
    let ExpressionKind::IntLiteral(v) = count.kind else {
        panic!("expected IntLiteral");
    };
    assert_eq!(v, 10);
}

#[test]
fn parse_anonymus_generate() {
    let program = parse_ok("@generate _ [10] { name = string; price = float; }");

    let Statement::Generate {
        template_name,
        body,
        count,
        ..
    } = &program.0[0]
    else {
        panic!("expected Generate statement");
    };
    assert!(template_name.is_none());
    assert_eq!(body.len(), 2);

    let ExpressionKind::Type(dt0) = &body[0].value.kind else {
        panic!("expected Type expression");
    };
    assert_eq!(body[0].name, "name");
    assert!(matches!(dt0.kind, DataTypeKind::Str));

    let ExpressionKind::Type(dt1) = &body[1].value.kind else {
        panic!("expected Type expression");
    };
    assert_eq!(body[1].name, "price");
    assert!(matches!(dt1.kind, DataTypeKind::Float));

    let ExpressionKind::IntLiteral(v) = count.kind else {
        panic!("expected IntLiteral");
    };
    assert_eq!(v, 10);
}

#[test]
fn parse_missing_paren_generate() {
    expect_unexpected_eof("@generate _ [10] { name = string; price = float;");
}

#[test]
fn parse_empty_enum() {
    let program = parse_ok("enum Role {}");

    let Statement::Enum {
        name,
        variants,
        attributes,
        ..
    } = &program.0[0]
    else {
        panic!("expected Enum statement");
    };
    assert_eq!(name, "Role");
    assert!(variants.is_empty());
    assert!(attributes.is_empty());
}

#[test]
fn parse_single_variant_enum() {
    let program = parse_ok("enum Role { User; }");

    let Statement::Enum {
        name,
        variants,
        attributes,
        ..
    } = &program.0[0]
    else {
        panic!("expected Enum statement");
    };
    assert_eq!(name, "Role");
    assert!(attributes.is_empty());
    assert_eq!(variants.len(), 1);
    assert_eq!(variants[0].name, "User");
    assert!(variants[0].weight.is_none());
}

#[test]
fn parse_enum() {
    let program = parse_ok("enum Role { User; Admin; Moderator; }");

    let Statement::Enum {
        name,
        variants,
        attributes,
        ..
    } = &program.0[0]
    else {
        panic!("expected Enum statement");
    };
    assert_eq!(name, "Role");
    assert!(attributes.is_empty());

    let expected = ["User", "Admin", "Moderator"];
    assert_eq!(variants.len(), expected.len());
    for (variant, expected_name) in variants.iter().zip(&expected) {
        assert_eq!(variant.name, *expected_name);
        assert!(variant.weight.is_none());
    }
}

#[test]
fn parse_weighted_variant_enum() {
    let program = parse_ok("#[public] enum Role { User => 50; Admin => 10; Developer => 30; }");

    let Statement::Enum {
        name,
        variants,
        attributes,
        ..
    } = &program.0[0]
    else {
        panic!("expected Enum statement");
    };
    assert_eq!(name, "Role");
    assert_eq!(attributes.len(), 1);
    let Attribute::Flag(attr_name, _) = &attributes[0] else {
        panic!("expected Flag attribute");
    };
    assert_eq!(attr_name, "public");

    let expected = [("User", 50), ("Admin", 10), ("Developer", 30)];
    assert_eq!(variants.len(), expected.len());
    for (variant, (expected_name, expected_weight)) in variants.iter().zip(&expected) {
        assert_eq!(variant.name, *expected_name);
        let Some(weight_expr) = &variant.weight else {
            panic!("expected weight for variant '{}'", variant.name);
        };
        let ExpressionKind::IntLiteral(w) = weight_expr.kind else {
            panic!("expected IntLiteral weight");
        };
        assert_eq!(w, *expected_weight);
    }
}

#[test]
fn parse_missing_paren_enum() {
    expect_unexpected_eof("enum Role { User; Admin; Moderator;");
}

#[test]
fn parse_extended_type() {
    let program = parse_ok(
        r#"
            #[public]
            type positive_int = int[range=0..=1024];
            type even_positive_int = extend positive_int with [multiple_of=2];
        "#,
    );
    assert_eq!(program.0.len(), 2);

    let Statement::TypeDecl {
        name: name1,
        data_type: dt1,
        attributes: attr1,
        ..
    } = &program.0[0]
    else {
        panic!("expected TypeDecl statement");
    };
    assert_eq!(name1, "positive_int");
    assert_eq!(attr1.len(), 1);
    let Attribute::Flag(attr_name, _) = &attr1[0] else {
        panic!("expected Flag attribute");
    };
    assert_eq!(attr_name, "public");

    let ExpressionKind::Type(dt) = &dt1.kind else {
        panic!("expected Type expression");
    };
    assert!(matches!(dt.kind, DataTypeKind::Int));
    let constraints = dt.constraints.as_ref().expect("expected constraints");
    assert_eq!(constraints.len(), 1);
    assert!(matches!(constraints[0].kind, ConstraintKind::Range));
    let ExpressionKind::Infix {
        left,
        operator,
        right,
    } = &constraints[0].expression.kind
    else {
        panic!("expected Infix expression");
    };
    assert!(matches!(operator, InfixOperator::InclusiveRange));
    assert!(matches!(left.kind, ExpressionKind::IntLiteral(0)));
    assert!(matches!(right.kind, ExpressionKind::IntLiteral(1024)));

    let Statement::TypeDecl {
        name: name2,
        data_type: dt2,
        attributes: attr2,
        ..
    } = &program.0[1]
    else {
        panic!("expected TypeDecl statement");
    };
    assert_eq!(name2, "even_positive_int");
    assert!(attr2.is_empty());

    let ExpressionKind::Type(dt) = &dt2.kind else {
        panic!("expected Type expression");
    };
    let DataTypeKind::Custom(base_name) = &dt.kind else {
        panic!("expected Custom type");
    };
    assert_eq!(base_name, "positive_int");
    let constraints = dt.constraints.as_ref().expect("expected constraints");
    assert_eq!(constraints.len(), 1);
    assert!(matches!(constraints[0].kind, ConstraintKind::MultipleOf));
    assert!(matches!(
        constraints[0].expression.kind,
        ExpressionKind::IntLiteral(2)
    ));
}

#[test]
fn parse_list_type() {
    let program = parse_ok("[int][range=1..=5];");

    let Statement::Expression(expr_stmt) = &program.0[0] else {
        panic!("expected Expression statement");
    };
    let ExpressionKind::Type(dt) = &expr_stmt.expression.kind else {
        panic!("expected Type expression");
    };
    let DataTypeKind::List(inner) = &dt.kind else {
        panic!("expected List type");
    };
    assert!(matches!(inner.kind, DataTypeKind::Int));

    let constraints = dt.constraints.as_ref().expect("expected constraints");
    assert_eq!(constraints.len(), 1);
    assert!(matches!(constraints[0].kind, ConstraintKind::Range));

    let ExpressionKind::Infix {
        left,
        operator,
        right,
    } = &constraints[0].expression.kind
    else {
        panic!("expected Infix expression");
    };
    assert!(matches!(operator, InfixOperator::InclusiveRange));
    assert!(matches!(left.kind, ExpressionKind::IntLiteral(1)));
    assert!(matches!(right.kind, ExpressionKind::IntLiteral(5)));
}

#[test]
fn parse_list_expression() {
    let program = parse_ok("[1 => 5; \"John\"; true => 25];");

    let Statement::Expression(expr_stmt) = &program.0[0] else {
        panic!("expected Expression statement");
    };
    let ExpressionKind::List(elements) = &expr_stmt.expression.kind else {
        panic!("expected List expression");
    };
    assert_eq!(elements.len(), 3);

    assert!(matches!(
        elements[0].value.kind,
        ExpressionKind::IntLiteral(1)
    ));
    assert!(matches!(
        elements[0].weight.as_ref().expect("expected weight").kind,
        ExpressionKind::IntLiteral(5)
    ));

    let ExpressionKind::StringLiteral(s) = &elements[1].value.kind else {
        panic!("expected StringLiteral");
    };
    assert_eq!(s, "John");
    assert!(elements[1].weight.is_none());

    assert!(matches!(
        elements[2].value.kind,
        ExpressionKind::BooleanLiteral(true)
    ));
    assert!(matches!(
        elements[2].weight.as_ref().expect("expected weight").kind,
        ExpressionKind::IntLiteral(25)
    ));
}

#[test]
fn parse_infix_expression() {
    let program = parse_ok("2 + 10 * 20;");

    let Statement::Expression(expr_stmt) = &program.0[0] else {
        panic!("expected Expression statement");
    };
    let ExpressionKind::Infix {
        left,
        operator,
        right,
    } = &expr_stmt.expression.kind
    else {
        panic!("expected Infix expression");
    };
    assert!(matches!(operator, InfixOperator::Plus));
    assert!(matches!(left.kind, ExpressionKind::IntLiteral(2)));

    let ExpressionKind::Infix {
        left: l2,
        operator: op2,
        right: r2,
    } = &right.kind
    else {
        panic!("expected nested Infix expression");
    };
    assert!(matches!(op2, InfixOperator::Multiply));
    assert!(matches!(l2.kind, ExpressionKind::IntLiteral(10)));
    assert!(matches!(r2.kind, ExpressionKind::IntLiteral(20)));
}

#[test]
fn parse_grouped_expression() {
    let program = parse_ok("(2 + 3) * 5;");

    let Statement::Expression(expr_stmt) = &program.0[0] else {
        panic!("expected Expression statement");
    };
    let ExpressionKind::Infix {
        left,
        operator,
        right,
    } = &expr_stmt.expression.kind
    else {
        panic!("expected Infix expression");
    };
    assert!(matches!(operator, InfixOperator::Multiply));
    assert!(matches!(right.kind, ExpressionKind::IntLiteral(5)));

    let ExpressionKind::Infix {
        left: l2,
        operator: op2,
        right: r2,
    } = &left.kind
    else {
        panic!("expected nested Infix expression");
    };
    assert!(matches!(op2, InfixOperator::Plus));
    assert!(matches!(l2.kind, ExpressionKind::IntLiteral(2)));
    assert!(matches!(r2.kind, ExpressionKind::IntLiteral(3)));
}

#[test]
fn parse_missing_paren_expression() {
    let lexer = Lexer::new("(2 << 3 & 5 >> 1;");
    match Parser::new(lexer, "(2 << 3 & 5 >> 1;").parse() {
        Ok(_) => panic!("expected a parse error, but parsing succeeded"),
        Err(err) => {
            let ParserError::Expected { expected, got, .. } = &err[0] else {
                panic!("expected Expected error");
            };
            assert_eq!(*expected, ")");
            assert_eq!(*got, ";");
        }
    }
}

#[test]
fn parse_prefix_expression() {
    let program = parse_ok("-5; !true;");
    assert_eq!(program.0.len(), 2);

    let Statement::Expression(expr1) = &program.0[0] else {
        panic!("expected Expression statement");
    };
    let ExpressionKind::Prefix {
        operator: op1,
        expression: inner1,
    } = &expr1.expression.kind
    else {
        panic!("expected Prefix expression");
    };
    assert!(matches!(op1, PrefixOperator::Negative));
    assert!(matches!(inner1.kind, ExpressionKind::IntLiteral(5)));

    let Statement::Expression(expr2) = &program.0[1] else {
        panic!("expected Expression statement");
    };
    let ExpressionKind::Prefix {
        operator: op2,
        expression: inner2,
    } = &expr2.expression.kind
    else {
        panic!("expected Prefix expression");
    };
    assert!(matches!(op2, PrefixOperator::LogicalNegate));
    assert!(matches!(inner2.kind, ExpressionKind::BooleanLiteral(true)));
}

#[test]
fn parse_directive() {
    let program = parse_ok("@output csv;");

    let Statement::OutputDirective {
        argument, options, ..
    } = &program.0[0]
    else {
        panic!("expected OutputDirective statement");
    };
    assert_eq!(argument, "csv");
    assert!(options.is_empty());
}

#[test]
fn parse_directive_options() {
    let program = parse_ok("@output csv { delimiter = \";\"; }");

    let Statement::OutputDirective {
        argument, options, ..
    } = &program.0[0]
    else {
        panic!("expected OutputDirective statement");
    };
    assert_eq!(argument, "csv");
    assert_eq!(options.len(), 1);
    assert_eq!(options[0].name, "delimiter");
    let ExpressionKind::StringLiteral(s) = &options[0].value.kind else {
        panic!("expected StringLiteral");
    };
    assert_eq!(s, ";");
}

#[test]
fn parse_output_path() {
    let program = parse_ok("@output_path \"./example.csv\";");

    let Statement::OutputPathDirective { argument, .. } = &program.0[0] else {
        panic!("expected OutputPathDirective statement");
    };
    assert_eq!(argument, &PathBuf::from("./example.csv"));
}

#[test]
fn parse_pattern_dollar_case() {
    let program = parse_ok("string_pattern \"dollar$$$$$$$$$$$$$$$$${aa}\";");

    let Statement::Expression(expr_stmt) = &program.0[0] else {
        panic!("expected Expression statement");
    };
    let ExpressionKind::StringPattern(elements) = &expr_stmt.expression.kind else {
        panic!("expected StringPattern expression");
    };
    assert_eq!(elements.len(), 2);

    let PatternElement::Literal(s, _) = &elements[0] else {
        panic!("expected Literal");
    };
    assert_eq!(s, "dollar$$$$$$$$$$$$$$$$");

    let PatternElement::RepeatChar {
        ch,
        count,
        count_expression,
        ..
    } = &elements[1]
    else {
        panic!("expected RepeatChar");
    };
    assert!(matches!(ch, PatternChar::Lowercase));
    assert_eq!(*count, 2);
    assert!(count_expression.is_none());
}

#[test]
fn parse_string_pattern() {
    let program = parse_ok("string_pattern \"testa$}${aaa[10]}john${A[25]##[13]}\";");

    let Statement::Expression(ExpressionStatemnt { expression, .. }) = &program.0[0] else {
        panic!("expected Expression statement");
    };
    let ExpressionKind::StringPattern(elements) = &expression.kind else {
        panic!("expected StringPattern expression");
    };
    assert_eq!(elements.len(), 5);

    let PatternElement::Literal(s0, _) = &elements[0] else {
        panic!("expected Literal at [0]");
    };
    assert_eq!(s0, "testa$}");

    let PatternElement::RepeatChar {
        ch: ch1,
        count: c1,
        count_expression: ce1,
        ..
    } = &elements[1]
    else {
        panic!("expected RepeatChar at [1]");
    };
    assert!(matches!(ch1, PatternChar::Lowercase));
    assert_eq!(*c1, 3);
    assert!(matches!(
        ce1.as_ref().expect("expected count expression").kind,
        ExpressionKind::IntLiteral(10)
    ));

    let PatternElement::Literal(s2, _) = &elements[2] else {
        panic!("expected Literal at [2]");
    };
    assert_eq!(s2, "john");

    let PatternElement::RepeatChar {
        ch: ch3,
        count: c3,
        count_expression: ce3,
        ..
    } = &elements[3]
    else {
        panic!("expected RepeatChar at [3]");
    };
    assert!(matches!(ch3, PatternChar::Uppercase));
    assert_eq!(*c3, 1);
    assert!(matches!(
        ce3.as_ref().expect("expected count expression").kind,
        ExpressionKind::IntLiteral(25)
    ));

    let PatternElement::RepeatChar {
        ch: ch4,
        count: c4,
        count_expression: ce4,
        ..
    } = &elements[4]
    else {
        panic!("expected RepeatChar at [4]");
    };
    assert!(matches!(ch4, PatternChar::Digit));
    assert_eq!(*c4, 2);
    assert!(matches!(
        ce4.as_ref().expect("expected count expression").kind,
        ExpressionKind::IntLiteral(13)
    ));
}
