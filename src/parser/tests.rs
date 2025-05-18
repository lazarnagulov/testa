#[cfg(test)]
mod parser_tests {
    use core::panic;
    use std::vec;

    use crate::parser::{ast::Statement, parser::Parser};


    #[test]
    fn parse_empty_enum() {
        let program = "enum Role {}";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(program.0, vec![Statement::Enum { name: "Role".to_string(), variants: vec![] }]);
            },
            Err(_) => panic!("Parsing error"),
        }
    }

    #[test]
    fn parse_single_variant_enum() {
        let program = "enum Role { User }";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(program.0, vec![Statement::Enum { 
                    name: "Role".to_string(), 
                    variants: vec!["User".to_string()] 
                }]);
            },
            Err(_) => panic!("Parsing error"),
        }
    }

    #[test]
    fn parse_enum() {
        let program = "enum Role { User, Admin, Moderator }";
        let mut parser = Parser::new(program);
        match parser.parse() {
            Ok(program) => {
                assert_eq!(program.0, vec![Statement::Enum { 
                    name: "Role".to_string(), 
                    variants: vec!["User".to_string(), "Admin".to_string(), "Moderator".to_string()] 
                }]);
            },
            Err(_) => panic!("Parsing error"),
        }
    }

}