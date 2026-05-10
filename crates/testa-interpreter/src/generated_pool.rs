use std::collections::{HashMap, VecDeque};

use rand::{Rng, rngs::StdRng};
use testa_hir::{FieldId, Item, ItemId, Module, module::Expr};

use crate::object::Object;

pub struct GeneratedPool {
    values: HashMap<ItemId, HashMap<FieldId, PoolBuffer>>,
    capacity: usize,
}

impl GeneratedPool {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            values: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            capacity: 0,
            values: HashMap::new()
        }
    }

    pub fn from_module(module: &Module) -> Self {
        let mut pool = Self::new(1000);

        for item in &module.items {
            if let Item::Template(t) = item {
                for field in &t.fields {
                    Self::register_refs_in_expr(&field.value, &mut pool);
                }
            }
        }

        pool
    }

    fn register_refs_in_expr(expr: &Expr, pool: &mut GeneratedPool) {
        if let Expr::Reference { template, field } = expr {
            let item_id = template.item_id();
            pool.values
                .entry(item_id)
                .or_default()
                .entry(*field)
                .or_insert_with(|| PoolBuffer::new(pool.capacity));
        }
    }
}

pub struct PoolBuffer {
    values: VecDeque<Object>,
    capacity: usize,
}

impl PoolBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            values: VecDeque::new(),
        }
    }

    pub fn push(&mut self, value: Object) {
        if self.values.len() >= self.capacity {
            self.values.pop_front();
        }
        self.values.push_back(value);
    }

    pub fn sample(&self, rng: &mut StdRng) -> Option<&Object> {
        if self.values.is_empty() {
            return None;
        }
        let idx = rng.random_range(0..self.values.len());
        self.values.get(idx)
    }
}
