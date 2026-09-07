use super::*;
use crate::analyser::symbol_table::symbol::SymbolKind;
use crate::ast::{DataType, DataTypeKind, Expression, ExpressionKind};
use crate::utils::Span;

fn dummy_span() -> Span {
    Span::default()
}

fn dummy_expression() -> Expression {
    Expression {
        kind: ExpressionKind::IntLiteral(1),
        span: dummy_span(),
    }
}

#[test]
fn test_valid_identifier_reference() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "known_var".to_string(),
            SymbolKind::Resource { values: vec![] },
            dummy_span(),
        )
        .unwrap();

    let mut checker = ReferenceChecker::new(&table);
    let expr = Expression {
        kind: ExpressionKind::Identifier("known_var".to_string()),
        span: dummy_span(),
    };

    checker.visit_expression(&expr);

    let (_, errors) = checker.finish();
    assert!(
        errors.is_empty(),
        "Expected no errors for a known identifier"
    );
}

#[test]
fn test_unknown_identifier_reference() {
    let table = SymbolTable::new();
    let mut checker = ReferenceChecker::new(&table);

    let expr = Expression {
        kind: ExpressionKind::Identifier("unknown_var".to_string()),
        span: dummy_span(),
    };

    checker.visit_expression(&expr);

    let (_, errors) = checker.finish();
    assert_eq!(errors.len(), 1);
    assert!(matches!(errors[0], SemanticError::UnknownIdentifier { .. }));
}

#[test]
fn test_unknown_custom_type_in_list() {
    let table = SymbolTable::new();
    let mut checker = ReferenceChecker::new(&table);

    let list_type = DataType {
        kind: DataTypeKind::Custom("UnknownType".to_string()),
        constraints: None,
        span: dummy_span(),
    };

    let expr = Expression {
        kind: ExpressionKind::Type(DataType {
            kind: DataTypeKind::List(Box::new(list_type)),
            constraints: None,
            span: dummy_span(),
        }),
        span: dummy_span(),
    };

    checker.visit_expression(&expr);

    let (_, errors) = checker.finish();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(errors[0], SemanticError::UnknownIdentifier { ref name, .. } if name == "UnknownType")
    );
}

#[test]
fn test_valid_template_parent() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "BaseTemplate".to_string(),
            SymbolKind::Template {
                parent: None,
                fields: vec![],
                attributes: vec![],
            },
            dummy_span(),
        )
        .unwrap();

    let mut checker = ReferenceChecker::new(&table);
    let parent_name = Some("BaseTemplate".to_string());

    checker.visit_template(&parent_name, &[], "ChildTemplate", &[], dummy_span());

    let (_, errors) = checker.finish();
    assert!(
        errors.is_empty(),
        "Expected no errors when extending a valid template"
    );
}

#[test]
fn test_unknown_template_parent() {
    let table = SymbolTable::new();
    let mut checker = ReferenceChecker::new(&table);
    let parent_name = Some("MissingTemplate".to_string());

    checker.visit_template(&parent_name, &[], "ChildTemplate", &[], dummy_span());

    let (_, errors) = checker.finish();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(errors[0], SemanticError::UnknownParentTemplate { ref name, .. } if name == "MissingTemplate")
    );
}

#[test]
fn test_invalid_parent_type_mismatch() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "MyStruct".to_string(),
            SymbolKind::Struct {
                name: "MyStruct".to_string(),
                fields: vec![],
            },
            dummy_span(),
        )
        .unwrap();

    let mut checker = ReferenceChecker::new(&table);
    let parent_name = Some("MyStruct".to_string());

    checker.visit_template(&parent_name, &[], "ChildTemplate", &[], dummy_span());

    let (_, errors) = checker.finish();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(errors[0], SemanticError::TypeMismatch { ref expected, .. } if expected == "template")
    );
}

#[test]
fn test_valid_generate_statement() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "MyTemplate".to_string(),
            SymbolKind::Template {
                parent: None,
                fields: vec![],
                attributes: vec![],
            },
            dummy_span(),
        )
        .unwrap();

    let mut checker = ReferenceChecker::new(&table);
    let template_name = Some("MyTemplate".to_string());
    let count_expr = dummy_expression();

    checker.visit_generate(&template_name, &[], &count_expr, dummy_span());

    let (_, errors) = checker.finish();

    assert!(
        errors.is_empty(),
        "Expected no errors generating a valid template"
    );
}

#[test]
fn test_unknown_generate_template() {
    let table = SymbolTable::new();
    let mut checker = ReferenceChecker::new(&table);
    let template_name = Some("UnknownTemplate".to_string());
    let count_expr = dummy_expression();

    checker.visit_generate(&template_name, &[], &count_expr, dummy_span());

    let (_, errors) = checker.finish();

    assert_eq!(errors.len(), 1);
    assert!(
        matches!(errors[0], SemanticError::UnknownTemplate { ref name, .. } if name == "UnknownTemplate")
    );
}

#[test]
fn test_invalid_generate_type_mismatch() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "MyEnum".to_string(),
            SymbolKind::Enum {
                variants: vec![],
                attributes: vec![],
            },
            dummy_span(),
        )
        .unwrap();

    let mut checker = ReferenceChecker::new(&table);
    let template_name = Some("MyEnum".to_string());
    let count_expr = dummy_expression();

    checker.visit_generate(&template_name, &[], &count_expr, dummy_span());

    let (_, errors) = checker.finish();

    assert_eq!(errors.len(), 1);
    assert!(
        matches!(errors[0], SemanticError::TypeMismatch { ref expected, .. } if expected == "template")
    );
}

#[test]
fn test_valid_reference_from_imports() {
    let local_table = SymbolTable::new();

    let mut imported_table = SymbolTable::new();
    imported_table
        .insert(
            "ImportedTemplate".to_string(),
            SymbolKind::Template {
                parent: None,
                fields: vec![],
                attributes: vec![],
            },
            dummy_span(),
        )
        .unwrap();

    let imports = vec![&imported_table];
    let mut checker = ReferenceChecker::with_imports(&local_table, &imports);

    let template_name = Some("ImportedTemplate".to_string());
    let count_expr = dummy_expression();

    checker.visit_generate(&template_name, &[], &count_expr, dummy_span());

    let (_, errors) = checker.finish();

    assert!(
        errors.is_empty(),
        "Expected to successfully resolve imported template"
    );
}

#[test]
fn test_valid_ref_syntax() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "User".to_string(),
            SymbolKind::Template {
                parent: None,
                fields: vec!["id".to_string(), "name".to_string()],
                attributes: vec![],
            },
            dummy_span(),
        )
        .unwrap();

    let mut checker = ReferenceChecker::new(&table);
    let expr = Expression {
        kind: ExpressionKind::Reference {
            template: "User".to_string(),
            field: "id".to_string(),
        },
        span: dummy_span(),
    };

    checker.visit_expression(&expr);
    let (_, errors) = checker.finish();
    assert!(
        errors.is_empty(),
        "Expected no errors for valid ref User.id"
    );
}

#[test]
fn test_ref_unknown_template() {
    let table = SymbolTable::new();
    let mut checker = ReferenceChecker::new(&table);

    let expr = Expression {
        kind: ExpressionKind::Reference {
            template: "NonExistent".to_string(),
            field: "id".to_string(),
        },
        span: dummy_span(),
    };

    checker.visit_expression(&expr);
    let (_, errors) = checker.finish();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(&errors[0], SemanticError::UnknownIdentifier { name, .. } if name == "NonExistent")
    );
}

#[test]
fn test_ref_unknown_field() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "User".to_string(),
            SymbolKind::Template {
                parent: None,
                fields: vec!["id".to_string()],
                attributes: vec![],
            },
            dummy_span(),
        )
        .unwrap();

    let mut checker = ReferenceChecker::new(&table);
    let expr = Expression {
        kind: ExpressionKind::Reference {
            template: "User".to_string(),
            field: "nonexistent".to_string(),
        },
        span: dummy_span(),
    };

    checker.visit_expression(&expr);
    let (_, errors) = checker.finish();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(&errors[0], SemanticError::UnknownIdentifier { name, .. } if name == "nonexistent")
    );
}

#[test]
fn test_ref_to_non_template() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "SomeEnum".to_string(),
            SymbolKind::Enum {
                variants: vec![],
                attributes: vec![],
            },
            dummy_span(),
        )
        .unwrap();

    let mut checker = ReferenceChecker::new(&table);
    let expr = Expression {
        kind: ExpressionKind::Reference {
            template: "SomeEnum".to_string(),
            field: "variant".to_string(),
        },
        span: dummy_span(),
    };

    checker.visit_expression(&expr);
    let (_, errors) = checker.finish();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(&errors[0], SemanticError::TypeMismatch { expected, .. } if expected == "template")
    );
}

#[test]
fn test_self_reference_is_valid() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "A".to_string(),
            SymbolKind::Template {
                parent: None,
                fields: vec!["id".to_string(), "reference".to_string()],
                attributes: vec![],
            },
            dummy_span(),
        )
        .unwrap();

    let mut checker = ReferenceChecker::new(&table);
    let expr = Expression {
        kind: ExpressionKind::Reference {
            template: "A".to_string(),
            field: "id".to_string(),
        },
        span: dummy_span(),
    };

    checker.visit_expression(&expr);
    let (_, errors) = checker.finish();
    assert!(
        errors.is_empty(),
        "Self-reference should be valid, handled at runtime by pool fallback"
    );
}

#[test]
fn test_ref_resolved_from_imported_table() {
    let local_table = SymbolTable::new();
    let mut imported_table = SymbolTable::new();
    imported_table
        .insert(
            "User".to_string(),
            SymbolKind::Template {
                parent: None,
                fields: vec!["id".to_string()],
                attributes: vec![],
            },
            dummy_span(),
        )
        .unwrap();

    let imports = vec![&imported_table];
    let mut checker = ReferenceChecker::with_imports(&local_table, &imports);
    let expr = Expression {
        kind: ExpressionKind::Reference {
            template: "User".to_string(),
            field: "id".to_string(),
        },
        span: dummy_span(),
    };

    checker.visit_expression(&expr);
    let (_, errors) = checker.finish();
    assert!(
        errors.is_empty(),
        "ref to imported template field should be valid"
    );
}

#[test]
fn test_ref_unknown_field_in_imported_template() {
    let local_table = SymbolTable::new();
    let mut imported_table = SymbolTable::new();
    imported_table
        .insert(
            "User".to_string(),
            SymbolKind::Template {
                parent: None,
                fields: vec!["id".to_string()],
                attributes: vec![],
            },
            dummy_span(),
        )
        .unwrap();

    let imports = vec![&imported_table];
    let mut checker = ReferenceChecker::with_imports(&local_table, &imports);
    let expr = Expression {
        kind: ExpressionKind::Reference {
            template: "User".to_string(),
            field: "nonexistent".to_string(),
        },
        span: dummy_span(),
    };

    checker.visit_expression(&expr);
    let (_, errors) = checker.finish();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(&errors[0], SemanticError::UnknownIdentifier { name, .. } if name == "nonexistent")
    );
}
