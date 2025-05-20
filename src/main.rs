use testa::parser::{parser::Parser, parser_error::ParserError};



fn main() -> Result<(), ParserError> {
    let mut parser = Parser::new(
r#"
            template User {
                id = int;
                name = string;
                age =  int;
            }
        "#
    );

    parser.parse()?;
    Ok(())
}
