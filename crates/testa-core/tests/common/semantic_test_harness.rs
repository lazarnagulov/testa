use testa_core::{
    analyser::{SemanticAnalyser, result::AnalysisResult, symbol_table::SymbolTable},
    lexer::Lexer,
    parser::Parser,
};

pub struct SemanticTestHarness<'a> {
    src: &'a str,
    imports: &'a [&'a SymbolTable],
}

impl<'a> SemanticTestHarness<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, imports: &[] }
    }

    pub fn with_imports(mut self, imports: &'a [&'a SymbolTable]) -> Self {
        self.imports = imports;
        self
    }

    pub fn run(&self) -> AnalysisResult {
        let lexer = Lexer::new(self.src);
        let mut parser = Parser::new(lexer, self.src);

        let program = parser.parse().unwrap_or_else(|errs| {
            panic!("Integration test failed at PARSING stage: {:?}", errs);
        });

        let mut analyser = SemanticAnalyser::new(&program);
        analyser.analyse_with_imports(self.imports)
    }

    pub fn assert_ok(&self) {
        let result = self.run();
        if result.has_errors() {
            let errors: Vec<_> = result.errors().collect();
            panic!(
                "Expected valid semantics, but found {} errors: {:#?}",
                errors.len(),
                errors
            );
        }
    }

    pub fn assert_err(&self, expected_msg: &str) {
        let result = self.run();
        let found = result.errors().any(|d| {
            d.message
                .to_lowercase()
                .contains(&expected_msg.to_lowercase())
        });
        assert!(
            found,
            "Expected error message containing '{}', but it was not found in: {:#?}",
            expected_msg, result.diagnostics
        );
    }
}
