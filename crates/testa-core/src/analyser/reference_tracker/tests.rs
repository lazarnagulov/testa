use super::*;
use crate::analyser::symbol_table::{SymbolTable, symbol::SymbolKind};
use crate::ast::{DataType, DataTypeKind, Expression, ExpressionKind, Statement};
use crate::utils::Span;

fn dummy_span() -> Span {
    Span::default()
}

fn populate_symbol(table: &mut SymbolTable, name: &str) {
    let dummy_kind = SymbolKind::Resource { values: vec![] };

    let _ = table.insert(name.to_string(), dummy_kind, Span::default());
}

#[test]
fn test_successful_reference_resolution() {
    let mut symbol_table = SymbolTable::new();
    populate_symbol(&mut symbol_table, "KnownStruct");

    let mut tracker = ReferenceTracker::new(&symbol_table);

    let stmt = Statement::Struct {
        name: "KnownStruct".to_string(),
        name_span: dummy_span(),
        body: vec![],
        span: dummy_span(),
    };

    tracker.visit_statement(&stmt);

    let errors = tracker.take_errors();
    assert!(
        errors.is_empty(),
        "Expected no semantic errors for known identifiers"
    );

    let (refs, _) = tracker.track_references(&Program(Vec::new()));

    let struct_refs = refs
        .get("KnownStruct")
        .expect("Expected 'KnownStruct' to be tracked");
    assert_eq!(struct_refs.len(), 1);
    assert_eq!(struct_refs[0].kind, ReferenceKind::StructDecl);
    assert!(struct_refs[0].is_resolved);
}

#[test]
fn test_unresolved_reference_error() {
    let symbol_table = SymbolTable::new();
    let mut tracker = ReferenceTracker::new(&symbol_table);

    let stmt = Statement::Generate {
        template_name: Some("UnknownTemplate".to_string()),
        template_name_span: Some(dummy_span()),
        body: vec![],
        count: Expression {
            kind: ExpressionKind::IntLiteral(1),
            span: dummy_span(),
        },
        span: dummy_span(),
    };

    tracker.visit_statement(&stmt);

    let errors = tracker.take_errors();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::UnknownIdentifier { .. }));

    let refs = tracker.references;
    let template_refs = refs.get("UnknownTemplate").unwrap();
    assert!(!template_refs[0].is_resolved);
    assert_eq!(template_refs[0].kind, ReferenceKind::TemplateGenerate);
}

#[test]
fn test_imported_symbol_resolution() {
    let local_table = SymbolTable::new();

    let mut imported_table = SymbolTable::new();
    populate_symbol(&mut imported_table, "ImportedEnum");

    let imports = vec![&imported_table];
    let mut tracker = ReferenceTracker::with_imports(&local_table, &imports);

    let stmt = Statement::Enum {
        name: "ImportedEnum".to_string(),
        name_span: Some(dummy_span()),
        variants: vec![],
        attributes: vec![],
        span: dummy_span(),
    };

    tracker.visit_statement(&stmt);

    let errors = tracker.take_errors();
    assert!(
        errors.is_empty(),
        "Should resolve 'ImportedEnum' from imported symbol table"
    );

    let refs = tracker.references;
    assert!(refs.get("ImportedEnum").unwrap()[0].is_resolved);
}

#[test]
fn test_expression_custom_type_tracking() {
    let symbol_table = SymbolTable::new();
    let mut tracker = ReferenceTracker::new(&symbol_table);

    let expr = Expression {
        kind: ExpressionKind::Type(DataType {
            kind: DataTypeKind::Custom("MyType".to_string()),
            constraints: None,
            span: dummy_span(),
        }),
        span: dummy_span(),
    };

    tracker.visit_expression(&expr);

    let refs = tracker.references;
    let type_refs = refs.get("MyType").expect("Expected 'MyType' to be tracked");

    assert_eq!(type_refs.len(), 1);
    assert_eq!(type_refs[0].kind, ReferenceKind::Type);
    assert!(!type_refs[0].is_resolved);
}

#[test]
fn test_expression_list_custom_type_tracking() {
    let symbol_table = SymbolTable::new();
    let mut tracker = ReferenceTracker::new(&symbol_table);

    let custom_type = DataType {
        kind: DataTypeKind::Custom("MyItem".to_string()),
        constraints: None,
        span: dummy_span(),
    };

    let expr = Expression {
        kind: ExpressionKind::Type(DataType {
            kind: DataTypeKind::List(Box::new(custom_type)),
            constraints: None,
            span: dummy_span(),
        }),
        span: dummy_span(),
    };

    tracker.visit_expression(&expr);

    let refs = tracker.references;
    let item_refs = refs
        .get("MyItem")
        .expect("Expected 'MyItem' to be tracked from inside the list");

    assert_eq!(item_refs[0].kind, ReferenceKind::Type);
}

#[test]
fn test_template_parent_tracking() {
    let symbol_table = SymbolTable::new();
    let mut tracker = ReferenceTracker::new(&symbol_table);

    let stmt = Statement::Template {
        name: "Child".to_string(),
        name_span: Some(dummy_span()),
        parent_name: Some("Parent".to_string()),
        parent_span: Some(dummy_span()),
        attributes: vec![],
        body: vec![],
        span: dummy_span(),
    };

    tracker.visit_statement(&stmt);

    let refs = tracker.references;

    let child_refs = refs.get("Child").unwrap();
    assert_eq!(child_refs[0].kind, ReferenceKind::TemplateDecl);

    let parent_refs = refs.get("Parent").unwrap();
    assert_eq!(parent_refs[0].kind, ReferenceKind::TemplateParent);
}
