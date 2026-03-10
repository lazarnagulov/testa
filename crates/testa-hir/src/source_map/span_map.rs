use std::collections::HashMap;

use serde::{Deserialize, Serialize};

const HASHMAP_THRESHOLD: usize = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanMap<K, V> {
    entries: Vec<(K, V)>,

    #[serde(skip)]
    cache: Option<HashMap<K, V>>,
}

impl<K, V> SpanMap<K, V>
where
    K: Eq + std::hash::Hash + Copy + Serialize,
    V: Copy,
{
    pub fn new(entries: Vec<(K, V)>) -> Self {
        Self {
            entries,
            cache: None,
        }
    }
    pub fn get(&mut self, key: &K) -> Option<V> {
        if self.entries.len() < HASHMAP_THRESHOLD {
            return self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| *v);
        }

        if self.cache.is_none() {
            self.cache = Some(self.entries.iter().copied().collect());
        }

        self.cache.as_ref().unwrap().get(key).copied()
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
