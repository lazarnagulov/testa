use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{StringPool, source_map::SourceMap};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StringId(pub u32);

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemId(pub u32);

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FieldId(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    Template(Template),
    // Enum(Enum),
    // TypeAlias(TypeAlias),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: ItemId,
    pub name: StringId,
    pub parent: Option<ItemId>,
    pub fields: Vec<Field>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub id: FieldId,
    pub name: StringId,
    pub ty: Type,
    pub default_value: Option<Expr>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Int,
    String,
    Bool,
    Float,
    Optional(Box<Type>),
    List(Box<Type>),
    UserDefined(ItemId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub kind: AttributeKind,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeKind {
    Required,
    Nullable,
    PrimaryKey,
    Min,
    Max,
    Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Int(i64),
    String(StringId),
    Bool(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub metadata: ModuleMetadata,
    pub string_pool: StringPool,
    pub items: Vec<Item>,
    pub source_map: SourceMap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetadata {
    pub version: u32,
    pub name: String,
    pub source_file: PathBuf,
    pub source_hash: u64,
    pub compiled_at: u64,
}
