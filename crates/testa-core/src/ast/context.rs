#[derive(Debug, Clone, PartialEq)]
pub enum CompletionContext {
    TopLevel,
    TemplateBody,
    FieldValue,
    Attribute,
    Directive,
    TypePosition,
    Expression,
    Unknown,
}

pub fn detect_completion_context(text: &str, line: u32, column: u32) -> CompletionContext {
    let lines: Vec<&str> = text.lines().collect();

    if line as usize >= lines.len() {
        return CompletionContext::Unknown;
    }

    let current_line = lines[line as usize];
    let before_cursor = &current_line[..column.min(current_line.len() as u32) as usize];

    if before_cursor.trim_start().starts_with('@') {
        return CompletionContext::Directive;
    }

    if before_cursor.contains("#[") && !before_cursor.contains(']') {
        return CompletionContext::Attribute;
    }

    let context = find_enclosing_context(text, line);
    match context {
        Some(EnclosingContext::Template) | Some(EnclosingContext::Struct) => {
            if before_cursor.contains('=') && !before_cursor.trim_end().ends_with('=') {
                return CompletionContext::FieldValue;
            }
            return CompletionContext::TemplateBody;
        }
        Some(EnclosingContext::Enum) => {
            return CompletionContext::Expression;
        }
        None => {}
    }

    CompletionContext::TopLevel
}

#[derive(Debug, Clone, PartialEq)]
enum EnclosingContext {
    Template,
    Struct,
    Enum,
}

fn find_enclosing_context(text: &str, target_line: u32) -> Option<EnclosingContext> {
    let lines: Vec<&str> = text.lines().collect();
    let mut depth = 0;
    let mut current_context = None;

    for (idx, line) in lines.iter().enumerate() {
        if idx > target_line as usize {
            break;
        }

        let trimmed = line.trim();
        if depth == 0 {
            if trimmed.starts_with("template ") {
                current_context = Some(EnclosingContext::Template);
            } else if trimmed.starts_with("enum ") {
                current_context = Some(EnclosingContext::Enum);
            } else if trimmed.starts_with("struct ") {
                current_context = Some(EnclosingContext::Struct);
            }
        }

        depth += line.matches('{').count() as i32;
        depth -= line.matches('}').count() as i32;

        if depth == 0 {
            current_context = None;
        }
    }

    current_context
}
