use testa_core::{analyser::SemanticAnalyser, lexer::Lexer, parser::Parser};

use crate::common::semantic_test_harness::SemanticTestHarness;
mod common;

#[test]
fn test_integration_inheritance_cycle() {
    let src = r#"
        template Alpha : Gamma {}
        template Beta : Alpha {}
        template Gamma : Beta {}
    "#;

    SemanticTestHarness::new(src)
        .assert_err("Template inheritance cycle detected: Alpha -> Gamma -> Beta -> Alpha");
}

#[test]
fn test_deep_valid_inheritance() {
    let src = r#"
        template Grandparent { a = int; }
        template Parent : Grandparent { b = string; }
        template Child : Parent { c = int; }
    "#;
    SemanticTestHarness::new(src).assert_ok();
}

#[test]
fn test_unknown_parent_template() {
    let src = "template Child : NonExistentParent {}";
    SemanticTestHarness::new(src).assert_err("Unknown parent template 'NonExistentParent'");
}

#[test]
fn test_duplicate_template_definition() {
    let src = r#"
        template User { id = int; }
        template User { email = string; }
    "#;

    SemanticTestHarness::new(src).assert_err("Symbol 'User' already declared in this scope");
}

#[test]
fn test_empty_enum_error() {
    let src = "enum Status {}";
    SemanticTestHarness::new(src).assert_err("Enum 'Status' must have at least one variant");
}

#[test]
fn test_duplicate_enum_variant() {
    let src = "enum Status { Active; Pending; Active; }";
    SemanticTestHarness::new(src).assert_err("already declared in this scope");
}

#[test]
fn test_unknown_enum_reference() {
    let src = "template T { s = MissingEnum; }";
    SemanticTestHarness::new(src).assert_err("Unknown identifier 'MissingEnum'");
}

#[test]
fn test_integration_unknown_type() {
    let src = "template Profile { avatar = CustomImage; }";

    SemanticTestHarness::new(src).assert_err("Unknown identifier 'CustomImage'");
}

#[test]
fn test_type_mismatch_in_generate() {
    let src = r#"
        template Mock {}
        @generate Mock ["five"];
    "#;
    SemanticTestHarness::new(src).assert_err("Type mismatch: expected 'int', found 'string'");
}

#[test]
fn test_invalid_binary_operation() {
    let src = r#"
        template T {
            val = "string" + 5;
        }
    "#;
    SemanticTestHarness::new(src).assert_err("cannot be applied to 'string' and 'int'");
}

#[test]
fn test_invalid_unary_operation() {
    let src = "template T { val = !5; }";
    SemanticTestHarness::new(src).assert_err("cannot be applied to 'int'");
}

#[test]
fn test_invalid_constraint_for_type() {
    let src = "template T { name = string[min=10]; }";
    SemanticTestHarness::new(src).assert_err("Invalid constraint 'min' for type 'string'");
}

#[test]
fn test_invalid_constraint_value_type() {
    let src = "template T { age = int[min=\"young\"]; }";
    SemanticTestHarness::new(src)
        .assert_err("Invalid value for constraint 'min': expected numeric value, found string");
}

#[test]
fn test_integration_invalid_generate_template() {
    let src = "@generate MissingTemplate [5];";
    SemanticTestHarness::new(src).assert_err("Unknown template 'MissingTemplate'");
}

#[test]
fn test_integration_with_imports() {
    let lib_src = "template BaseTemplate { id = int; }";
    let lib_lexer = Lexer::new(lib_src);
    let lib_program = Parser::new(lib_lexer, lib_src).parse().unwrap();
    let mut lib_analyser = SemanticAnalyser::new(&lib_program);
    let lib_result = lib_analyser.analyse();

    let main_src = "template User : BaseTemplate { name = string; }";
    SemanticTestHarness::new(main_src)
        .with_imports(&[&lib_result.symbol_table])
        .assert_ok();
}

#[test]
fn test_valid_ref_syntax() {
    let src = r#"
        template User { id = int; name = string; }
        template Post { author_id = ref User.id; }
    "#;
    SemanticTestHarness::new(src).assert_ok();
}

#[test]
fn test_ref_unknown_template() {
    let src = r#"
        template Post { author_id = ref NonExistent.id; }
    "#;
    SemanticTestHarness::new(src).assert_err("Unknown identifier 'NonExistent'");
}

#[test]
fn test_ref_unknown_field() {
    let src = r#"
        template User { id = int; }
        template Post { author_id = ref User.nonexistent; }
    "#;
    SemanticTestHarness::new(src).assert_err("Unknown identifier 'nonexistent'");
}

#[test]
fn test_ref_to_non_template() {
    let src = r#"
        enum Status { Active; Inactive; }
        template Post { status_ref = ref Status.Active; }
    "#;
    SemanticTestHarness::new(src).assert_err("Type mismatch");
}

#[test]
fn test_self_reference_is_valid() {
    let src = r#"
        template Node {
            id = int;
            parent_id = ref Node.id;
        }
    "#;
    SemanticTestHarness::new(src).assert_ok();
}

#[test]
fn test_ref_from_imported_template() {
    let lib_src = "template User { id = int; name = string; }";
    let lib_lexer = Lexer::new(lib_src);
    let lib_program = Parser::new(lib_lexer, lib_src).parse().unwrap();
    let mut lib_analyser = SemanticAnalyser::new(&lib_program);
    let lib_result = lib_analyser.analyse();

    let main_src = r#"
        template Post { author_id = ref User.id; }
    "#;
    SemanticTestHarness::new(main_src)
        .with_imports(&[&lib_result.symbol_table])
        .assert_ok();
}

#[test]
fn test_ref_unknown_field_in_imported_template() {
    let lib_src = "template User { id = int; }";
    let lib_lexer = Lexer::new(lib_src);
    let lib_program = Parser::new(lib_lexer, lib_src).parse().unwrap();
    let mut lib_analyser = SemanticAnalyser::new(&lib_program);
    let lib_result = lib_analyser.analyse();

    let main_src = r#"
        template Post { author_id = ref User.nonexistent; }
    "#;
    SemanticTestHarness::new(main_src)
        .with_imports(&[&lib_result.symbol_table])
        .assert_err("Unknown identifier 'nonexistent'");
}

#[test]
fn test_multiple_refs_to_same_template() {
    let src = r#"
        template User { id = int; name = string; }
        template Post {
            author_id = ref User.id;
            author_name = ref User.name;
        }
    "#;
    SemanticTestHarness::new(src).assert_ok();
}

#[test]
fn test_ref_chain() {
    let src = r#"
        template C { id = int; }
        template B { c_id = ref C.id; }
        template A { b_id = ref B.c_id; }
    "#;
    SemanticTestHarness::new(src).assert_ok();
}