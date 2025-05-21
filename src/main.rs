use testa::parser::{parser::Parser, parser_error::ParserError};



fn main() -> Result<(), ParserError> {
    let mut parser = Parser::new(
r#"
            generate _ [10] {
                name = int;
                age = 12.3 + 10;
            }
        "#
    );

    parser.parse()?;
    Ok(())
}
