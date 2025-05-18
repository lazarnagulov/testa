#[cfg(test)]
mod lexer_tests {
    use std::vec;

    use crate::lexer::{lexer::Lexer, token::TokenKind::{self, *}};

    fn expect_token(lexer: &mut Lexer, expected: Vec<TokenKind>) {
        let token_kinds = lexer.into_iter().map(|token| token.kind).collect::<Vec<_>>();
        assert_eq!(token_kinds, expected);
    }

    fn expect_token_size(lexer: &mut Lexer, input: &str, expected: Vec<&str>) {
        let token_kinds = lexer.into_iter().map(|token| &input[token.start..token.start + token.size]).collect::<Vec<_>>();
        assert_eq!(token_kinds, expected);
    }

    #[test]
    fn lex_single_char_tokens() {
        let program = "(){}:[],.;=!+-/*&|^<>";
        let mut lexer = Lexer::new(program);   
        expect_token(&mut lexer, vec![
            LParen, RParen, LBrace, RBrace, Colon, LBracket, RBracket,
            Comma, Period, Semicolon, SingleEqual, ExclamationMark,
            Plus, Minus, Slash, Asterisk, BitAnd, BitOr, BitXor, LessThan, GreaterThan    
        ]);
    }

    #[test]
    fn lex_two_char_tokens() {
        let program = "==!=<=>=";
        let mut lexer = Lexer::new(program);
        expect_token(&mut lexer, vec![
            DoubleEqual, NotEqual, LessThanOrEqual, GreaterThanOrEqual
        ]);
    }

    #[test]
    fn lex_string_tokens() {
        let program = "generate @output $uuid john \"Peter\" 123";
        let mut lexer = Lexer::new(program);
        expect_token(&mut lexer, vec![
            Generate, Output, Uuid, Identifier, StringLiteral, IntLiteral
        ]);
    }

    #[test]
    #[should_panic]
    fn lex_invalid_multiline_string() {
        let program = r#"
            "john
            peter
            "
        "#;
        let mut lexer = Lexer::new(program);
        expect_token(&mut lexer, vec![]);
    }

    #[test]
    fn lex_literal_size() {
        let program = "john \"Peter\" 123";
        let mut lexer = Lexer::new(program);
        expect_token_size(&mut lexer, program, vec![
            "john", "\"Peter\"", "123"
        ]); 
    }

    #[test]
    #[should_panic]
    fn lex_invalid_string() {
        let program = "\"john ";
        let mut lexer = Lexer::new(program);
        expect_token(&mut lexer, vec![]);
    }

    #[test]
    #[should_panic]
    fn lex_invalid_directive() {
        let program = "@invalid_directive";
        let mut lexer = Lexer::new(program);
        expect_token(&mut lexer, vec![]);
    }

    #[test]
    #[should_panic]
    fn lex_invalid_buitin() {
        let program = "$invalid_buitin";
        let mut lexer = Lexer::new(program);
        expect_token(&mut lexer, vec![]);
    }

}