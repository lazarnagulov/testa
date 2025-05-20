use testa::parser::{parser::Parser, parser_error::ParserError};



fn main() -> Result<(), ParserError> {
    let mut parser = Parser::new(
r#"
            generate _ [10] {
                name = int;
                age = float;
            }
        "#
    );

    parser.parse()?;
    Ok(())
}
