use testa::parser::parser::Parser;



fn main() {
    let mut parser = Parser::new(
r#"
        @output csv { delimiter = ";"; }
        "#
    );

    match parser.parse() {
        Ok(_) => {},
        Err(error) => match error {
            testa::parser::parser_error::ParserError::Expected { expected, got } => panic!("Expected {} got {}", expected, got),
            testa::parser::parser_error::ParserError::UnexpectedEOF => panic!("Unexpected end of file"),
            testa::parser::parser_error::ParserError::Syntax(message) => panic!("{}", message),
        },
    }
}
