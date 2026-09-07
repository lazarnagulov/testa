use std::fmt;

use serde::{Deserialize, Serialize};
use testa_core::analyser::type_checker;

use crate::lowering::context::ItemKind;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StringId(pub u32);

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModuleId(pub u32);

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LocalItemId(pub u32);

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GlobalItemId {
    pub module: ModuleId,
    pub item: LocalItemId,
}

impl GlobalItemId {
    pub fn module_id(self) -> ModuleId {
        self.module
    }

    pub fn item_id(self) -> LocalItemId {
        self.item
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Directive {
    Output {
        format: StringId,
        options: Vec<(StringId, Expr)>,
    },
    OutputPath(StringId),
    Generate {
        template: GlobalItemId,
        count: Expr,
    },
    Import(StringId),
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FieldId(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    Template(Template),
    Enum(Enum),
    TypeAlias(TypeAlias),
    Struct(Struct),
}

impl Item {
    pub fn id(&self) -> LocalItemId {
        match self {
            Item::Template(t) => t.id,
            Item::Enum(e) => e.id,
            Item::Struct(s) => s.id,
            Item::TypeAlias(t) => t.id,
        }
    }

    pub fn name(&self) -> StringId {
        match self {
            Item::Template(t) => t.name,
            Item::Struct(s) => s.name,
            Item::Enum(e) => e.name,
            Item::TypeAlias(t) => t.name,
        }
    }

    pub fn kind(&self) -> ItemKind {
        match self {
            Item::Template(_) => ItemKind::Template,
            Item::Struct(_) => ItemKind::Struct,
            Item::Enum(_) => ItemKind::Enum,
            Item::TypeAlias(_) => ItemKind::TypeAlias,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: LocalItemId,
    pub name: StringId,
    pub parent: Option<GlobalItemId>,
    pub fields: Vec<Field>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    pub id: LocalItemId,
    pub name: StringId,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub id: LocalItemId,
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
    pub id: LocalItemId,
    pub name: StringId,
    pub target_type: Type,
    pub constraints: Vec<Constraint>,
    pub attributes: Vec<Attribute>,
    pub expr: Option<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub id: FieldId,
    pub name: StringId,
    pub ty: Type,
    pub value: Expr,
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
    UserDefined(GlobalItemId),
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
    Type(Type),
    Identifier(GlobalItemId),
    StringPattern(Vec<PatternPart>),
    Infix {
        left: Box<Expr>,
        right: Box<Expr>,
        op: InfixOp,
    },
    Prefix {
        op: PrefixOp,
        expr: Box<Expr>,
    },
    List(Vec<Expr>),
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        inclusive: bool,
    },
    ConstrainedType {
        ty: Type,
        constraints: Vec<Constraint>,
    },
    Reference {
        template: GlobalItemId,
        field: FieldId,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternPart {
    Literal(StringId),
    RepeatChar {
        ch: PatternChar,
        count: usize,
        count_expr: Option<Box<Expr>>,
    },
    RepeatGroup {
        chars: Vec<PatternChar>,
        count: Box<Expr>,
    },
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PatternChar {
    Lowercase,
    Uppercase,
    Digit,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum PrefixOp {
    Neg,
    Not,
    BitNeg,
}

impl fmt::Display for PrefixOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrefixOp::Neg => write!(f, "-"),
            PrefixOp::Not => write!(f, "!"),
            PrefixOp::BitNeg => write!(f, "~"),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    BitLShift,
    BitRShift,
    Equal,
    And,
    Or,
    NotEqual,
    LessThen,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    ExclusiveRange,
    InclusiveRange,
}

impl fmt::Display for InfixOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfixOp::Add => write!(f, "+"),
            InfixOp::Sub => write!(f, "-"),
            InfixOp::Div => write!(f, "/"),
            InfixOp::Mod => write!(f, "%"),
            InfixOp::Mul => write!(f, "*"),
            InfixOp::BitAnd => write!(f, "&"),
            InfixOp::BitOr => write!(f, "|"),
            InfixOp::BitXor => write!(f, "^"),
            InfixOp::BitLShift => write!(f, "<<"),
            InfixOp::BitRShift => write!(f, ">>"),
            InfixOp::Equal => write!(f, "="),
            InfixOp::And => write!(f, "&&"),
            InfixOp::Or => write!(f, "||"),
            InfixOp::NotEqual => write!(f, "!="),
            InfixOp::LessThen => write!(f, "<"),
            InfixOp::GreaterThan => write!(f, ">"),
            InfixOp::LessThanOrEqual => write!(f, "<="),
            InfixOp::GreaterThanOrEqual => write!(f, ">="),
            InfixOp::ExclusiveRange => write!(f, "exclusive range"),
            InfixOp::InclusiveRange => write!(f, "inclusive range"),
        }
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

impl fmt::Display for LocalItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl fmt::Display for ModuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.0)
    }
}

impl fmt::Display for GlobalItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.module, self.item)
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
