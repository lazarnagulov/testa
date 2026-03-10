use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{Module, module::error::ResolveError};

pub struct ModuleResolver {
    search_paths: Vec<PathBuf>,
    cache: HashMap<String, Module>,
}

impl ModuleResolver {
    pub fn new(search_paths: Vec<PathBuf>) -> Self {
        Self {
            cache: HashMap::new(),
            search_paths,
        }
    }

    pub fn with_defaults(relative_to: &Path) -> Self {
        let mut paths = Vec::new();

        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                paths.push(dir.join("std"));
            }
        }

        if let Some(parent) = relative_to.parent() {
            paths.push(parent.to_path_buf());
        }

        Self::new(paths)
    }

    fn resolve_one(&mut self, name: &str, relative_to: &Path) -> Result<(), ResolveError> {
        if self.cache.contains_key(name) {
            return Ok(());
        }

        let path = self
            .find_tmod(name, relative_to)
            .ok_or_else(|| ResolveError::NotFound(name.to_string()))?;

        let module = Module::load(&path)?;

        let transitive: Vec<String> = module
            .imports
            .iter()
            .map(|id| module.string_pool.resolve(*id).to_string())
            .collect();

        self.cache.insert(name.to_string(), module);

        for dep in transitive {
            self.resolve_one(&dep, &path)?;
        }

        Ok(())
    }

    pub fn resolve_all(
        &mut self,
        names: &[String],
        relative_to: &Path,
    ) -> Result<HashMap<String, Module>, ResolveError> {
        for name in names {
            self.resolve_one(name, relative_to)?;
        }
        Ok(self
            .cache
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect())
    }

    fn find_tmod(&self, name: &str, relative_to: &Path) -> Option<PathBuf> {
        let filename = format!("{}.tmod", name);

        if let Some(parent) = relative_to.parent() {
            let candidate = parent.join(&filename);

            if candidate.exists() {
                return Some(candidate);
            }
        }

        for path in &self.search_paths {
            let candidate = path.join(&filename);
            if candidate.exists() {
                return Some(candidate);
            }
        }

        None
    }
}
