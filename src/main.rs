use testa::{evaluator::{context::Context, evaluator::evaluate}, parser::{parser::Parser, parser_error::ParserError}};



fn main() -> Result<(), ParserError> {
    let mut parser = Parser::new(
r#"
            generate _ [10] {
                name = string;
                age = int;
            }
        "#
    );
    let mut context = Context::default();
    let program = parser.parse()?;
    match evaluate(program, &mut context) {
        Ok(val) => println!("{:?}", *val),
        Err(_) => todo!(),
    }
    Ok(())
}
