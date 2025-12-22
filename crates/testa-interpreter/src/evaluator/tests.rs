use crate::{evaluator, evaluator::context::Context, object::Object};
use testa_core::parser::Parser;

#[test]
fn evalute_simple_expression() {
    expect_object("2 + 3 * 5;", Object::Int(17));
    expect_object("(2 + 3) * 5;", Object::Int(25));
    expect_object("(2 + 3) ^ 5;", Object::Int(0));
    expect_object("2 != 3;", Object::Boolean(true));
    expect_object("2 > 3;", Object::Boolean(false));
    expect_object("2.5 + 2.5;", Object::Float(5.0));
    expect_object("2.5 == 2.5;", Object::Boolean(true));
    expect_object("~10 + 11;", Object::Int(0));
    expect_object("-27 + (-3);", Object::Int(-30));
    expect_object("1024 >> 10;", Object::Int(1));
    expect_object("2 << 31;", Object::Int(4294967296));
    expect_object(
        "\"-27.252\" + \"str\";",
        Object::String("-27.252str".to_string()),
    );
    expect_object("\"same\" == \"same\";", Object::Boolean(true));
}

#[test]
fn evaluate_enum() {
    let program = Parser::new(
        r#"
            enum Role { User; Admin; Moderator; }
            enum Seniority { Junior; Medior; Senior; }          
        "#,
    )
    .parse()
    .unwrap();
    let mut context = Context::default();
    let result = evaluator::evaluate(program, &mut context).unwrap();
    assert_eq!(result, Object::NoReturn);
    assert!(context.get_enum("Role").is_some());
    assert!(context.get_enum("Seniority").is_some());
}

#[test]
fn evaluate_template() {
    let program = Parser::new(
        r#"
            template User {
                name = string;
                age = int;    
            }

            template Product {
                id = int;
                name = string;
                price = float;
            }           
        "#,
    )
    .parse()
    .unwrap();
    let mut context = Context::default();
    let result = evaluator::evaluate(program, &mut context).unwrap();
    assert_eq!(result, Object::NoReturn);
    assert!(context.get_template("Product").is_some());
    assert!(context.get_template("User").is_some());
}

fn expect_object(source: &str, object: Object) {
    let program = Parser::new(source).parse().unwrap();
    let mut context = Context::default();
    let result = evaluator::evaluate(program, &mut context).unwrap();
    assert_eq!(result, object);
}
