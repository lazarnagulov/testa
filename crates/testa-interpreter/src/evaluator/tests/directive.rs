use testa_core::{
    analyser::symbol_table::SymbolTable,
    ast::{Program, Statement},
    utils::Span,
};

use crate::evaluator::{
    Evaluator,
    context::{Context, OutputFormat},
};

#[test]
fn test_output_directive() {
    let context = Context::new(SymbolTable::new());

    let mut evaluator = Evaluator::new(context, None);
    let program = Program(vec![Statement::OutputDirective {
        argument: "csv".into(),
        options: vec![],
        span: Span::default(),
    }]);
    evaluator
        .evaluate_directives(&program)
        .expect("Failed to evaluate output directive");
    assert_eq!(evaluator.context.output_format, OutputFormat::Csv);
}

#[test]
fn test_invalid_output_directive() {
    let context = Context::new(SymbolTable::new());

    let mut evaluator = Evaluator::new(context, None);
    let program = Program(vec![Statement::OutputDirective {
        argument: "invalid".into(),
        options: vec![],
        span: Span::default(),
    }]);
    assert!(evaluator.evaluate_directives(&program).is_err());
}
