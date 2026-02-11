use testa_core::ast::{CompletionContext, detect_completion_context};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse, Documentation, InsertTextFormat};

use crate::lsp::backend::Backend;

pub(crate) async fn handle_completion(
    backend: &Backend,
    params: CompletionParams,
) -> Result<Option<CompletionResponse>> {
    let uri = params.text_document_position.text_document.uri;
    let document = backend.get_document(&uri).await?;
    let lsp_position = params.text_document_position.position;
    
    let line = lsp_position.line;
    let column = lsp_position.character;

    let context = detect_completion_context(&document.text, line, column);
    let items = match context {
        CompletionContext::TopLevel => generate_top_level_completions(),
        CompletionContext::TemplateBody => generate_field_completions(),
        CompletionContext::Directive => generate_directive_completions(),
        CompletionContext::Expression => generate_expression_completions(),
        _ => generate_generic_completions()
    };

    Ok(Some(CompletionResponse::Array(items)))
}

fn generate_top_level_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "type".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Type alias".to_string()),
            insert_text: Some("type ${1:Name} = ${2:int};".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "Define a type alias with optional constraints".to_string()
            )),
            ..Default::default()
        },
        CompletionItem {
            label: "enum".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Enum definition".to_string()),
            insert_text: Some("enum ${1:Name} {\n\t${2:Variant} => ${3:Weight};\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "Define an enumeration with variants".to_string()
            )),
            ..Default::default()
        },
        CompletionItem {
            label: "template".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Template definition".to_string()),
            insert_text: Some("template ${1:Name} {\n\t${2:field} = ${3:type};\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "Define a data template for generation".to_string()
            )),
            ..Default::default()
        },
        CompletionItem {
            label: "struct".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Struct definition".to_string()),
            insert_text: Some("struct ${1:Name} {\n\t${2:field} = ${3:type};\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "Define a struct".to_string()
            )),
            ..Default::default()
        },
    ]
}

fn generate_field_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "field".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some("Field declaration".to_string()),
            insert_text: Some("${1:name} = ${2:type};".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        }   
    ]
}

fn generate_builtin_type_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "int".to_string(),
            kind: Some(CompletionItemKind::CLASS),
            detail: Some("Integer type".to_string()),
            documentation: Some(Documentation::String(
                "Built-in integer type. Can have constraints like [range=1..10]".to_string()
            )),
            ..Default::default()
        },
        CompletionItem {
            label: "string".to_string(),
            kind: Some(CompletionItemKind::CLASS),
            detail: Some("String type".to_string()),
            documentation: Some(Documentation::String(
                "Built-in string type. Can have constraints like [length=5..20]".to_string()
            )),
            ..Default::default()
        },
        CompletionItem {
            label: "float".to_string(),
            kind: Some(CompletionItemKind::CLASS),
            detail: Some("Float type".to_string()),
            documentation: Some(Documentation::String(
                "Built-in floating point number type".to_string()
            )),
            ..Default::default()
        },
        CompletionItem {
            label: "bool".to_string(),
            kind: Some(CompletionItemKind::CLASS),
            detail: Some("Boolean type".to_string()),
            documentation: Some(Documentation::String(
                "Built-in boolean type (true/false)".to_string()
            )),
            ..Default::default()
        },
        CompletionItem {
            label: "string_pattern".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some("Pattern-based string".to_string()),
            insert_text: Some("string_pattern \"${1:pattern}\"".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            documentation: Some(Documentation::String(
                "Generate strings based on a pattern. Example: \"ID_${#[3]}\"".to_string()
            )),
            ..Default::default()
        },
    ]
}

fn generate_expression_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "true".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Boolean literal".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "false".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Boolean literal".to_string()),
            ..Default::default()
        },
    ]
}

fn generate_directive_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "output".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Output format".to_string()),
            insert_text: Some("output ${1:csv};".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "output_path".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Output file path".to_string()),
            insert_text: Some("output_path \"${1:output.csv}\";".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "generate".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Generate data".to_string()),
            insert_text: Some("generate ${1:Template}[${2:10}];".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
    ]
}


fn generate_generic_completions() -> Vec<CompletionItem> {
    let mut items = generate_top_level_completions();
    items.extend(generate_builtin_type_completions());
    items
}