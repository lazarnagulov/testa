#[cfg(test)]
mod lexer_tests {
    use std::vec;

    use crate::lexer::{
        lexer::Lexer,
        token::TokenKind::{self, *},
    };

    #[test]
    fn lex_single_char_tokens() {
        let program = "(){}:[],.;=!+-/*&|^<>~%";
        let mut lexer = Lexer::new(program);
        expect_token(
            &mut lexer,
            vec![
                LParen,
                RParen,
                LBrace,
                RBrace,
                Colon,
                LBracket,
                RBracket,
                Comma,
                SinglePeriod,
                Semicolon,
                SingleEqual,
                ExclamationMark,
                Plus,
                Minus,
                Slash,
                Asterisk,
                BitAnd,
                BitOr,
                BitXor,
                LessThan,
                GreaterThan,
                BitNegate,
                Percent,
            ],
        );
    }

    #[test]
    fn lex_two_char_tokens() {
        let program = "==!=<=>=<<>>&&||=>..";
        let mut lexer = Lexer::new(program);
        expect_token(
            &mut lexer,
            vec![
                DoubleEqual,
                NotEqual,
                LessThanOrEqual,
                GreaterThanOrEqual,
                BitLShift,
                BitRShift,
                And,
                Or,
                Arrow,
                DoublePeriod,
            ],
        );
    }

    #[test]
    fn lex_three_char_tokens() {
        let program = "..=";
        let mut lexer = Lexer::new(program);
        expect_token(&mut lexer, vec![DoublePeriodEqual]);
    }

    #[test]
    fn lex_range() {
        let program = "10..=20";
        let mut lexer = Lexer::new(program);
        expect_token(&mut lexer, vec![IntLiteral, DoublePeriodEqual, IntLiteral]);
    }

    #[test]
    fn lex_string_tokens() {
        let program = "generate @output $uuid john \"Peter\" 123 true false int float string 123.123 type constraint";
        let mut lexer = Lexer::new(program);
        expect_token(
            &mut lexer,
            vec![
                Generate,
                Output,
                Uuid,
                Identifier,
                StringLiteral,
                IntLiteral,
                True,
                False,
                Int,
                Float,
                Str,
                FloatLiteral,
                Type,
                Constraint,
            ],
        );
    }

    #[test]
    #[should_panic]
    fn lex_invalid_float() {
        let program = "123.123.234";
        let lexer = Lexer::new(program);
        expect_panic(lexer);
    }

    #[test]
    #[should_panic]
    fn lex_invalid_int() {
        let program = "123abc";
        let lexer = Lexer::new(program);
        expect_panic(lexer);
    }

    #[test]
    #[should_panic]
    fn lex_invalid_multiline_string() {
        let program = r#"
            "john
            peter
            "
        "#;
        let lexer = Lexer::new(program);
        expect_panic(lexer);
    }

    #[test]
    fn lex_literal_size() {
        let program = "john \"Peter\" 123";
        let mut lexer = Lexer::new(program);
        expect_token_size(&mut lexer, program, vec!["john", "\"Peter\"", "123"]);
    }

    #[test]
    #[should_panic]
    fn lex_invalid_string() {
        let program = "\"john ";
        let lexer = Lexer::new(program);
        expect_panic(lexer);
    }

    #[test]
    #[should_panic]
    fn lex_invalid_directive() {
        let program = "@invalid_directive";
        let lexer = Lexer::new(program);
        expect_panic(lexer);
    }

    #[test]
    #[should_panic]
    fn lex_invalid_buitin() {
        let program = "$invalid_buitin";
        let lexer = Lexer::new(program);
        expect_panic(lexer);
    }

    fn expect_token(lexer: &mut Lexer, expected: Vec<TokenKind>) {
        let token_kinds = lexer
            .into_iter()
            .map(|token| token.kind)
            .collect::<Vec<_>>();
        assert_eq!(token_kinds, expected);
    }

    fn expect_token_size(lexer: &mut Lexer, input: &str, expected: Vec<&str>) {
        let token_kinds = lexer
            .into_iter()
            .map(|token| &input[token.start..token.start + token.size])
            .collect::<Vec<_>>();
        assert_eq!(token_kinds, expected);
    }

    fn expect_panic(lexer: Lexer) {
        lexer.into_iter().map(|token| token.kind).for_each(drop);
    }
}
