#[cfg(test)]
mod evaluator_tests {
    use crate::{evaluator::{evaluator::evaluate, object::Object}, parser::parser::Parser};


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
        expect_object("\"-27.252\" + \"str\";", Object::String("-27.252str".to_string()));
        expect_object("\"same\" == \"same\";", Object::Boolean(true));
    }

    #[test]
    fn evalute_data_types() {
        let program = Parser::new("int;").parse().unwrap();
        let result = evaluate(program).unwrap();
        println!("{}", result);
    }

    fn expect_object(source: &str, object: Object) {
        let program = Parser::new(source).parse().unwrap();
        let result = evaluate(program).unwrap();
        assert_eq!(*result, object);
    }

}