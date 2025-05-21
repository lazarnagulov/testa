use testa::parser::{parser::Parser, parser_error::ParserError};



fn main() -> Result<(), ParserError> {
    let mut parser = Parser::new(
r#"
            @output csv {
                delimiter = ";";
            }
        "#
    );

    parser.parse()?;
    Ok(())
}
