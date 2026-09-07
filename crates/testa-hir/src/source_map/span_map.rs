use std::{collections::HashMap, sync::OnceLock};

use serde::{Deserialize, Serialize};

const HASHMAP_THRESHOLD: usize = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanMap<K, V> {
    entries: Vec<(K, V)>,

    #[serde(skip)]
    cache: OnceLock<HashMap<K, V>>,
}

impl<K, V> SpanMap<K, V>
where
    K: Eq + std::hash::Hash + Copy + Serialize,
    V: Copy,
{
    pub fn new(entries: Vec<(K, V)>) -> Self {
        Self {
            entries,
            cache: OnceLock::new(),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        if self.entries.len() < HASHMAP_THRESHOLD {
            return self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| *v);
        }

        self.cache
            .get_or_init(|| self.entries.iter().copied().collect())
            .get(key)
            .copied()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.entries.iter()
    }
}
