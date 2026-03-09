use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::StringId;



#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StringPool {
    strings: Vec<String>,
    map: HashMap<String, StringId>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            map: HashMap::new()
        }
    }

    pub fn intern(&mut self, s: &str) -> StringId {
        if let Some(&id) = self.map.get(s) {
            return id;
        }
        let id = StringId(self.strings.len() as u32);
        self.strings.push(s.to_string());
        self.map.insert(s.to_string(), id);
        id
    }
    
    pub fn resolve(&self, id: StringId) -> &str {
        &self.strings[id.0 as usize]
    }
}