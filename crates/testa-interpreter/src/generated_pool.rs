use rand::Rng;
use rand::rngs::StdRng;
use std::collections::{HashMap, VecDeque};
use testa_hir::module::{FieldId, ItemRef};

use crate::object::Object;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PoolKey {
    item_ref: ItemRef,
    field_id: FieldId,
}

pub struct GeneratedPool {
    values: HashMap<PoolKey, PoolBuffer>,
    capacity: usize,
}

impl GeneratedPool {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            values: HashMap::new(),
        }
    }

    pub fn push(&mut self, item_ref: &ItemRef, field_id: FieldId, value: Object) {
        let key = PoolKey {
            item_ref: *item_ref,
            field_id,
        };
        self.values
            .entry(key)
            .or_insert_with(|| PoolBuffer::new(self.capacity))
            .push(value);
    }

    pub fn sample(
        &self,
        rng: &mut StdRng,
        item_ref: &ItemRef,
        field_id: FieldId,
    ) -> Option<&Object> {
        let key = PoolKey {
            item_ref: *item_ref,
            field_id,
        };
        self.values.get(&key)?.sample(rng)
    }
}

impl Default for GeneratedPool {
    fn default() -> Self {
        Self::new(1000)
    }
}

struct PoolBuffer {
    values: VecDeque<Object>,
    capacity: usize,
}

impl PoolBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            values: VecDeque::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Object) {
        if self.values.len() >= self.capacity {
            self.values.pop_front();
        }
        self.values.push_back(value);
    }

    fn sample(&self, rng: &mut StdRng) -> Option<&Object> {
        if self.values.is_empty() {
            return None;
        }
        let idx = rng.random_range(0..self.values.len());
        self.values.get(idx)
    }
}
