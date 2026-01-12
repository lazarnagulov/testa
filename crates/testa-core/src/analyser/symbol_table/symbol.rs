use std::{
    collections::HashMap,
    fmt::{self, Display},
    hash::Hash,
};

use crate::{
    ast::{Attribute, Expression},
    utils::Span,
};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub span: Span,
    pub scope_id: ScopeId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Template {
        fields: Vec<String>,
        parent: Option<String>,
        attributes: Vec<Attribute>,
    },
    Struct {
        fields: Vec<String>,
    },
    Enum {
        variants: Vec<VariantInfo>,
        attributes: Vec<Attribute>,
    },
    Resource {
        values: Vec<String>,
    },
    TypeAlias {
        name: String,
        data_type: Expression,
        attributes: Vec<Attribute>,
    },
    Variant {
        enum_name: String,
    },
    Field {
        template_name: String,
        is_override: bool,
        expression: Expression,
    },
}

impl Display for SymbolKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolKind::Template {
                fields,
                parent,
                attributes,
            } => {
                write!(f, "template")?;
                if let Some(parent) = parent {
                    write!(f, " extends {}", parent)?;
                }
                write!(f, " with {} fields", fields.len())?;
                if !attributes.is_empty() {
                    write!(f, " [{} attributes]", attributes.len())?;
                }
                Ok(())
            }
            SymbolKind::Enum {
                variants,
                attributes,
            } => {
                write!(f, "enum with {} variants", variants.len())?;
                if !attributes.is_empty() {
                    write!(f, " [{} attributes]", attributes.len())?;
                }
                Ok(())
            }
            SymbolKind::Resource { values } => {
                write!(f, "resource with {} values", values.len())
            }
            SymbolKind::TypeAlias {
                name, attributes, ..
            } => {
                write!(f, "type alias {}", name)?;
                if !attributes.is_empty() {
                    write!(f, " [{} attributes]", attributes.len())?;
                }
                Ok(())
            }
            SymbolKind::Variant { enum_name } => {
                write!(f, "variant of enum {}", enum_name)
            }
            SymbolKind::Field {
                template_name,
                is_override,
                expression: _,
            } => {
                if *is_override {
                    write!(f, "override field of template {}", template_name)
                } else {
                    write!(f, "field of template {}", template_name)
                }
            }
            SymbolKind::Struct { fields } => {
                write!(f, "struct")?;
                write!(f, " with {} fields", fields.len())?;
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub id: ScopeId,
    pub parent: Option<ScopeId>,
    pub symbols: HashMap<String, Symbol>,
    pub kind: ScopeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ScopeId(pub usize);

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeKind {
    Global,
    Template { name: String },
    Enum { name: String },
    Generate { name: String },
    Directive { name: String },
    Block,
}

impl fmt::Display for ScopeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScopeKind::Global => write!(f, "global"),
            ScopeKind::Template { name } => write!(f, "template {}", name),
            ScopeKind::Enum { name } => write!(f, "enum {}", name),
            ScopeKind::Generate { name } => write!(f, "generate {}", name),
            ScopeKind::Directive { name } => write!(f, "directive {}", name),
            ScopeKind::Block => write!(f, "block"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantInfo {
    pub name: String,
    pub weight: Option<Expression>,
}

impl VariantInfo {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            weight: None,
        }
    }

    pub fn with_weight(mut self, weight_expression: Expression) -> Self {
        self.weight = Some(weight_expression);
        self
    }
}
