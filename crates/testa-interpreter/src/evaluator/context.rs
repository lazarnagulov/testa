use std::{collections::HashMap, path::PathBuf, str::FromStr};

use rand::{SeedableRng, rngs::StdRng};
use testa_hir::module::{
    Module,
    node::{Enum, GlobalItemId, Item, ModuleId, StringId, Template, TypeAlias},
};

use crate::{generated_pool::GeneratedPool, object::Object};

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
    pub imported: HashMap<ModuleId, Module>,
    pub module: Module,
    pub output_format: OutputFormat,
}

pub struct State {
    pub rng: StdRng,
    pub pool: GeneratedPool,
}

impl State {
    pub fn new(seed: Option<u64>) -> Self {
        let rng = seed
            .map(StdRng::seed_from_u64)
            .unwrap_or_else(|| StdRng::from_rng(&mut rand::rng()));
        Self {
            rng,
            pool: GeneratedPool::default(),
        }
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
        self.imported = imported
            .into_values()
            .map(|m| (m.metadata.id, m))
            .collect();
        self
    }

    pub fn with_options(mut self, options: GenerateOptions) -> Self {
        self.output_path = options.output_path;
        if let Some(format) = options.format {
            self.output_format = format;
        }
        self
    }

    pub fn module_for(&self, item_ref: &GlobalItemId) -> &Module {
        if item_ref.module == self.module.metadata.id {
            &self.module
        } else {
            self.imported.get(&item_ref.module).unwrap_or(&self.module)
        }
    }

    pub fn resolve_item(&self, item_ref: &GlobalItemId) -> Option<&Item> {
        self.module_for(item_ref).get_item(item_ref.item)
    }

    pub fn resolve_template(&self, item_ref: &GlobalItemId) -> Option<&Template> {
        match self.resolve_item(item_ref)? {
            Item::Template(t) => Some(t),
            _ => None,
        }
    }

    pub fn resolve_enum(&self, item_ref: &GlobalItemId) -> Option<&Enum> {
        match self.resolve_item(item_ref)? {
            Item::Enum(e) => Some(e),
            _ => None,
        }
    }

    pub fn resolve_type_alias(&self, item_ref: &GlobalItemId) -> Option<&TypeAlias> {
        match self.resolve_item(item_ref)? {
            Item::TypeAlias(t) => Some(t),
            _ => None,
        }
    }

    pub fn resolve_string(&self, id: StringId, item_ref: &GlobalItemId) -> &str {
        self.module_for(item_ref).string_pool.resolve(id)
    }

    pub fn resolve_local_string(&self, id: StringId) -> &str {
        self.module.string_pool.resolve(id)
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