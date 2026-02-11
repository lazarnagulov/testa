use testa_core::{
    analyser::symbol_table::symbol_table_builder::SymbolTableBuilder,
    ast::{DataType, DataTypeKind, Expression, ExpressionKind, Program, Statement, Variant},
    utils::Span,
};

use crate::{
    evaluator::{Evaluator, context::Context},
    object::Object,
};

const SEED: Option<u64> = Some(42);

#[test]
fn test_evaluate_enum() {
    let symbol_table = SymbolTableBuilder::new()
        .build(&Program(vec![Statement::Enum {
            name: "test".to_string(),
            variants: vec![Variant::new("variant1".to_string(), None, Span::default())],
            attributes: Vec::new(),
            span: Span::default(),
            name_span: Some(Span::default()),
        }]))
        .expect("Symbol table builder failed");

    let context = Context::new(symbol_table);
    let mut evaluator = Evaluator::new(context, SEED);
    let object = evaluator
        .evaluate_expression(&Expression::new(
            ExpressionKind::Identifier("test".to_string()),
            Span::default(),
        ))
        .expect("Evaluation failed");

    assert_eq!(object, Object::String("variant1".to_string()));
}

#[test]
fn test_evaluate_data_type() {
    let symbol_table = SymbolTableBuilder::new()
        .build(&Program(vec![Statement::TypeDecl {
            name: "test".to_string(),
            data_type: Expression::new(
                ExpressionKind::Type(DataType::new(DataTypeKind::Int, None, Span::default())),
                Span::default(),
            ),
            attributes: Vec::new(),
            span: Span::default(),
            name_span: Some(Span::default()),
        }]))
        .expect("Symbol table builder failed");

    let context = Context::new(symbol_table);
    let mut evaluator = Evaluator::new(context, SEED);
    let object = evaluator
        .evaluate_expression(&Expression::new(
            ExpressionKind::Identifier("test".to_string()),
            Span::default(),
        ))
        .expect("Evaluation failed");

    assert_eq!(object, Object::Int(572990626));
}
