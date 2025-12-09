use crate::{
    analyser::symbol_table::{SymbolTable, symbol::SymbolKind}, utils::Span
};

#[test]
fn test_basic_symbol_insertion() {
    let mut table = SymbolTable::new();

    let result = table.insert(
        "User".to_string(),
        SymbolKind::Template {
            parent: None,
            fields: vec!["id".to_string(), "name".to_string()],
            attributes: vec![],
        },
        Span::default(),
    );

    assert!(result.is_ok());
    assert!(table.lookup("User").is_some());
}

#[test]
fn test_duplicate_detection() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "User".to_string(),
            SymbolKind::Template {
                parent: None,
                fields: vec![],
                attributes: vec![],
            },
            Span::default(),
        )
        .unwrap();

    let result = table.insert(
        "User".to_string(),
        SymbolKind::Template {
            parent: None,
            fields: vec![],
            attributes: vec![],
        },
        Span::default(),
    );

    assert!(result.is_err());
}

#[test]
fn test_template_parents() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Base".to_string(),
            SymbolKind::Template {
                parent: None,
                fields: vec![],
                attributes: vec![],
            },
            Span::default(),
        )
        .unwrap();

    table
        .insert(
            "Middle".to_string(),
            SymbolKind::Template {
                parent: Some("Base".to_string()),
                fields: vec![],
                attributes: vec![],
            },
            Span::default(),
        )
        .unwrap();

    table
        .insert(
            "Derived".to_string(),
            SymbolKind::Template {
                parent: Some("Middle".to_string()),
                fields: vec![],
                attributes: vec![],
            },
            Span::default(),
        )
        .unwrap();

    let parents = table.get_template_parents("Derived");
    assert_eq!(parents, vec!["Middle".to_string(), "Base".to_string()]);
}
