use std::{collections::HashMap, path::PathBuf, str::FromStr};

use rand::{SeedableRng, rngs::StdRng};
use testa_hir::{
    Item, Module, StringId,
    module::{Enum, ItemRef, Template, TypeAlias},
};

use crate::object::Object;

#[derive(Debug, Clone)]
pub struct GenerateOptions {
    pub input: PathBuf,
    pub output_path: Option<PathBuf>,
    pub format: Option<OutputFormat>,
    pub count: Option<usize>,
    pub seed: Option<u64>,
}

#[derive(Debug)]
pub struct Context {
    pub output_path: Option<PathBuf>,
    pub output_options: HashMap<String, Object>,
    pub imported: HashMap<String, Module>,
    pub module: Module,
    pub output_format: OutputFormat,
}

pub struct State {
    pub rng: StdRng,
}

impl State {
    pub fn new(seed: Option<u64>) -> Self {
        let rng = seed
            .map(StdRng::seed_from_u64)
            .unwrap_or_else(|| StdRng::from_rng(&mut rand::rng()));
        Self { rng }
    }
}

impl Context {
    pub fn new(module: Module) -> Self {
        Self {
            module,
            imported: HashMap::new(),
            output_path: None,
            output_options: HashMap::new(),
            output_format: OutputFormat::default(),
        }
    }

    pub fn with_imported(mut self, imported: HashMap<String, Module>) -> Self {
        self.imported = imported;
        self
    }

    pub fn with_options(mut self, options: GenerateOptions) -> Self {
        self.output_path = options.output_path;
        if let Some(format) = options.format {
            self.output_format = format;
        }
        self
    }

    pub fn resolve_item(&self, item_ref: &ItemRef) -> Option<&Item> {
        match item_ref {
            ItemRef::Local(id) => self.module.get_item(*id),
            ItemRef::Imported { module, item } => {
                let module_name = self.module.string_pool.resolve(*module);
                self.imported.get(module_name)?.get_item(*item)
            }
        }
    }

    pub fn resolve_template(&self, item_ref: &ItemRef) -> Option<&Template> {
        match self.resolve_item(item_ref)? {
            Item::Template(t) => Some(t),
            _ => None,
        }
    }

    pub fn resolve_enum(&self, item_ref: &ItemRef) -> Option<&Enum> {
        match self.resolve_item(item_ref)? {
            Item::Enum(e) => Some(e),
            _ => None,
        }
    }

    pub fn resolve_type_alias(&self, item_ref: &ItemRef) -> Option<&TypeAlias> {
        match self.resolve_item(item_ref)? {
            Item::TypeAlias(t) => Some(t),
            _ => None,
        }
    }

    pub fn resolve_string(&self, id: StringId, item_ref: &ItemRef) -> &str {
        match item_ref {
            ItemRef::Local(_) => self.module.string_pool.resolve(id),
            ItemRef::Imported { module, .. } => {
                let module_name = self.module.string_pool.resolve(*module);
                self.imported
                    .get(module_name)
                    .map(|m| m.string_pool.resolve(id))
                    .unwrap_or("")
            }
        }
    }

    pub fn resolve_local_string(&self, id: StringId) -> &str {
        self.module.string_pool.resolve(id)
    }

    pub fn module_for(&self, item_ref: &ItemRef) -> &Module {
        match item_ref {
            ItemRef::Local(_) => &self.module,
            ItemRef::Imported { module, .. } => {
                let name = self.module.string_pool.resolve(*module);
                self.imported.get(name).unwrap_or(&self.module)
            }
        }
    }

    pub fn find_template_by_name(&self, name: &str) -> Option<(&Template, &Module)> {
        if let Some(Item::Template(t)) = self.module.get_item_by_name(name) {
            return Some((t, &self.module));
        }
        for module in self.imported.values() {
            if let Some(Item::Template(t)) = module.get_item_by_name(name) {
                return Some((t, module));
            }
        }
        None
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum OutputFormat {
    #[default]
    Csv,
    Json,
    Sql,
    Xml,
}

impl OutputFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Csv => "csv",
            OutputFormat::Json => "json",
            OutputFormat::Sql => "sql",
            OutputFormat::Xml => "xml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.eq_ignore_ascii_case("csv") => Ok(OutputFormat::Csv),
            s if s.eq_ignore_ascii_case("json") => Ok(OutputFormat::Json),
            s if s.eq_ignore_ascii_case("sql") => Ok(OutputFormat::Sql),
            s if s.eq_ignore_ascii_case("xml") => Ok(OutputFormat::Xml),
            other => Err(format!("Unknown output format: {}", other)),
        }
    }
}
