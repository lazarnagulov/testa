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
        _ => Vec::new(),
        // CompletionContext::TemplateInheritance => todo!(),
        // CompletionContext::TemplateBody => todo!(),
        // CompletionContext::FieldValue => todo!(),
        // CompletionContext::Attribute => todo!(),
        // CompletionContext::Directive => todo!(),
        // CompletionContext::TypePosition => todo!(),
        // CompletionContext::Expression => todo!(),
        // CompletionContext::Unknown => todo!(),
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

