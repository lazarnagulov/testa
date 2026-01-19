use testa_core::analyser::reference_tracker::Reference;
use testa_core::analyser::symbol_table::symbol::{Symbol, SymbolKind};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{Hover, HoverContents, HoverParams, MarkupContent, MarkupKind};

use crate::lsp::backend::Backend;
use crate::lsp::workspace::document::Analysis;

pub(crate) async fn handle_hover(backend: &Backend, params: HoverParams) -> Result<Option<Hover>> {
    let uri = params.text_document_position_params.text_document.uri;
    let document = backend.get_document(&uri).await?;

    let lsp_position = params.text_document_position_params.position;
    let line = lsp_position.line + 1;
    let column = lsp_position.character + 1;

    let analysis = document.get_analysis()?;
    let Some((symbol, reference)) = resolve_symbol_at_position(analysis, line, column) else {
        return Ok(None);
    };
    let hover_content = generate_hover_content(&reference.name, &symbol.kind);

    Ok(Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: hover_content,
        }),
        range: Some(reference.span.to_lsp_range()),
    }))
}

fn resolve_symbol_at_position(analysis: &Analysis, line: u32, column: u32) -> Option<(&Symbol, &Reference)>{
    let reference= analysis.find_reference_at(line, column)?;
    let symbol_table = analysis.symbol_table.as_ref()?;
    let symbol = symbol_table.lookup(&reference.name)?;
    Some((symbol, reference))
}

fn generate_hover_content(name: &str, kind: &SymbolKind) -> String {
    match kind {
        SymbolKind::Template {
            fields,
            parent,
            attributes,
        } => {
            let mut content = String::new();

            content.push_str(&format!("```testa\ntemplate {}", name));
            if let Some(parent_name) = parent {
                content.push_str(&format!(" : {}", parent_name));
            }
            content.push_str("\n```\n\n");

            if !attributes.is_empty() {
                content.push_str("**Attributes:**\n");
                for attr in attributes {
                    content.push_str(&format!("- `{}`\n", attr.name()));
                }
                content.push('\n');
            }

            content.push_str(&format!("**Fields:** {}\n", fields.len()));
            if !fields.is_empty() {
                for field in fields {
                    content.push_str(&format!("- `{}`\n", field));
                }
            }

            content
        }
        SymbolKind::Enum { variants, attributes } => {
            let mut content = String::new();
            content.push_str(&format!("```testa\nenum {}\n```\n\n", name));
            
            if !attributes.is_empty() {
                content.push_str("**Attributes:**\n");
                for attr in attributes {
                    content.push_str(&format!("- `{}`\n", attr.name()));
                }
                content.push('\n');
            }
            
            content.push_str(&format!("**Variants:** {}\n", variants.len()));
            for variant in variants {
                if let Some(weight) = &variant.weight {
                    content.push_str(&format!("- `{}` (weight: {})\n", variant.name, weight.kind));
                } else {
                    content.push_str(&format!("- `{}`\n", variant.name));
                }
            }
            
            content
        }
        SymbolKind::Resource { values } => {
            format!(
                "```testa\nresource {}\n```\n\n**Values:** {}",
                name,
                values.len()
            )
        }
        SymbolKind::TypeAlias { name, data_type, attributes } => {
            let mut content = String::new();
            
            content.push_str(&format!("```testa\ntype {} = ", name));
            
            content.push_str(&data_type.kind.to_string());
            content.push_str("\n```\n\n");
            
            if !attributes.is_empty() {
                content.push_str("**Attributes:**\n");
                for attr in attributes {
                    content.push_str(&format!("- `{}`\n", attr.name()));
                }
            }
            
            content
        }
        SymbolKind::Variant { enum_name } => {
            format!(
                "```testa\n{}\n```\n\nVariant of enum `{}`",
                name, enum_name
            )
        }
        SymbolKind::Field { template_name, is_override, expression } => {
            let mut content = String::new();
            
            if *is_override {
                content.push_str(&format!("```testa\n{} = {}\n```\n\n", name, expression.kind));
                content.push_str(&format!("**Override** field of template `{}`", template_name));
            } else {
                content.push_str(&format!("```testa\n{} = {}\n```\n\n", name, expression.kind));
                content.push_str(&format!("Field of template `{}`", template_name));
            }
            
            content
        }
    }
}