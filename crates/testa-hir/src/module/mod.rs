pub mod error;
pub mod resolver;
pub mod serialize;

use crate::{StringPool, source_map::SourceMap};
use core::fmt;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use testa_core::{self, analyser::type_checker};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StringId(pub u32);

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemId(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ItemRef {
    Local(ItemId),
    Imported { module: StringId, item: ItemId },
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FieldId(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    Template(Template),
    Enum(Enum),
    TypeAlias(TypeAlias),
}

impl Item {
    pub fn id(&self) -> ItemId {
        match self {
            Item::Template(t) => t.id,
            Item::Enum(e) => e.id,
            Item::TypeAlias(t) => t.id,
        }
    }

    pub fn name(&self) -> StringId {
        match self {
            Item::Template(t) => t.name,
            Item::Enum(e) => e.name,
            Item::TypeAlias(t) => t.name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: ItemId,
    pub name: StringId,
    pub parent: Option<ItemRef>,
    pub fields: Vec<Field>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub id: ItemId,
    pub name: StringId,
    pub variants: Vec<Variant>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub name: StringId,
    pub weight: Option<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAlias {
    pub id: ItemId,
    pub name: StringId,
    pub target_type: Type,
    pub constraints: Vec<Constraint>,
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
    UserDefined(ItemRef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub kind: ConstraintKind,
    pub value: Expr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintKind {
    Range { min: Expr, max: Expr },
    Min,
    Max,
    Length,
    MultipleOf,
    Bias,
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
    Private,
    Public,
    Abstract,
}

pub fn attribute_kind(name: &str) -> AttributeKind {
    match name {
        "nullable" => AttributeKind::Nullable,
        "primary_key" => AttributeKind::PrimaryKey,
        "private" => AttributeKind::Private,
        "public" => AttributeKind::Public,
        "abstract" => AttributeKind::Abstract,
        _ => AttributeKind::Required,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Int(i64),
    Float(f64),
    String(StringId),
    Bool(bool),
    List(Vec<Expr>),
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        inclusive: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub metadata: ModuleMetadata,
    pub string_pool: StringPool,
    pub imports: Vec<StringId>,
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

impl Module {
    pub fn get_item(&self, id: ItemId) -> Option<&Item> {
        self.items.iter().find(|item| match item {
            Item::Template(t) => t.id == id,
            Item::Enum(e) => e.id == id,
            Item::TypeAlias(t) => t.id == id,
        })
    }

    pub fn get_item_by_name(&self, name: &str) -> Option<&Item> {
        self.items.iter().find(|item| {
            let item_name = match item {
                Item::Template(t) => self.string_pool.resolve(t.name),
                Item::Enum(e) => self.string_pool.resolve(e.name),
                Item::TypeAlias(t) => self.string_pool.resolve(t.name),
            };
            item_name == name
        })
    }

    pub fn templates(&self) -> impl Iterator<Item = &Template> {
        self.items.iter().filter_map(|item| {
            if let Item::Template(t) = item {
                Some(t)
            } else {
                None
            }
        })
    }

    pub fn enums(&self) -> impl Iterator<Item = &Enum> {
        self.items.iter().filter_map(|item| {
            if let Item::Enum(e) = item {
                Some(e)
            } else {
                None
            }
        })
    }

    pub fn type_aliases(&self) -> impl Iterator<Item = &TypeAlias> {
        self.items.iter().filter_map(|item| {
            if let Item::TypeAlias(t) = item {
                Some(t)
            } else {
                None
            }
        })
    }

    pub fn is_stale(&self) -> Result<bool, std::io::Error> {
        use std::collections::hash_map::DefaultHasher;
        use std::fs;
        use std::hash::{Hash, Hasher};

        let current_source = fs::read_to_string(&self.metadata.source_file)?;

        let mut hasher = DefaultHasher::new();
        current_source.hash(&mut hasher);
        let current_hash = hasher.finish();

        Ok(current_hash != self.metadata.source_hash)
    }
}

impl Template {
    pub fn get_field(&self, id: FieldId) -> Option<&Field> {
        self.fields.iter().find(|f| f.id == id)
    }

    pub fn get_field_by_name(&self, name: StringId) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }
}

impl Enum {
    pub fn get_variant_by_name(&self, name: StringId) -> Option<&Variant> {
        self.variants.iter().find(|v| v.name == name)
    }
}

impl fmt::Display for ItemRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemRef::Local(item_id) => write!(f, "{}", item_id),
            ItemRef::Imported { module, item } => write!(f, "{}::{}", module, item),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::String => write!(f, "string"),
            Type::Bool => write!(f, "bool"),
            Type::Float => write!(f, "float"),
            Type::Optional(inner) => write!(f, "?{}", inner),
            Type::List(inner) => write!(f, "[{}]", inner),
            Type::UserDefined(item_ref) => write!(f, "UserDefined({})", item_ref),
        }
    }
}

impl From<&type_checker::types::Type> for Type {
    fn from(ty: &type_checker::types::Type) -> Self {
        match ty {
            type_checker::types::Type::Int => Type::Int,
            type_checker::types::Type::Float => Type::Float,
            type_checker::types::Type::Str => Type::String,
            type_checker::types::Type::Boolean => Type::Bool,
            type_checker::types::Type::List(inner) => {
                Type::List(Box::new(Type::from(inner.as_ref())))
            }
            type_checker::types::Type::Custom(_) => Type::Int, // caller handles this
            _ => Type::Int,
        }
    }
}

impl std::fmt::Display for ItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl std::fmt::Display for StringId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}

impl std::fmt::Display for FieldId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}
