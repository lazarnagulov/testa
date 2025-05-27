use std::i32;

use crate::evaluator::object::Object;

use super::{
    constrainted_type::Constraint,
    sampler::{BooleanSampler, Sampler, UniformSampler},
};

#[derive(Debug, Clone)]
pub struct RangeConstraint {
    pub min: isize,
    pub max: isize,
}

impl RangeConstraint {
    pub fn new(min: isize, max: isize) -> Self {
        RangeConstraint { min, max }
    }
}

impl Constraint for RangeConstraint {
    fn validate(&self, object: &Object) -> bool {
        matches!(object, Object::Int(v) if *v >= self.min && *v <= self.max)
    }

    fn description(&self) -> String {
        format!("range={}..={}", self.min, self.max)
    }

    fn build_sampler(&self) -> Option<Box<dyn Sampler>> {
        Some(Box::new(UniformSampler::new(
            self.min as i32,
            self.max as i32,
        )))
    }

    fn clone_box(&self) -> Box<dyn Constraint> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct MultipleOfConstraint {
    pub base: i32,
}

impl MultipleOfConstraint {
    pub fn new(base: i32) -> Self {
        MultipleOfConstraint { base }
    }
}

impl Constraint for MultipleOfConstraint {
    fn validate(&self, value: &Object) -> bool {
        matches!(value, Object::Int(v) if *v as i32 % self.base == 0)
    }

    fn description(&self) -> String {
        format!("multiple_of({})", self.base)
    }

    fn build_sampler(&self) -> Option<Box<dyn Sampler>> {
        None
    }

    fn clone_box(&self) -> Box<dyn Constraint> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct BiasConstraint {
    pub percent: f32,
}

impl BiasConstraint {
    pub fn new(percent: f32) -> Self {
        BiasConstraint { percent }
    }
}

impl Constraint for BiasConstraint {
    fn validate(&self, _value: &Object) -> bool {
        true
    }

    fn description(&self) -> String {
        format!("bias({}%)", self.percent)
    }

    fn build_sampler(&self) -> Option<Box<dyn Sampler>> {
        Some(Box::new(BooleanSampler::new(self.percent)))
    }

    fn clone_box(&self) -> Box<dyn Constraint> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct MinConstraint {
    value: i32,
}

impl MinConstraint {
    pub fn new(value: i32) -> Self {
        MinConstraint { value }
    }
}

impl Constraint for MinConstraint {
    fn validate(&self, object: &Object) -> bool {
        matches!(object, Object::Int(v) if *v >= self.value as isize)
    }

    fn description(&self) -> String {
        format!("min({})", self.value)
    }

    fn build_sampler(&self) -> Option<Box<dyn Sampler>> {
        Some(Box::new(UniformSampler::new(
            self.value,
            i32::MAX,
        )))
    }

    fn clone_box(&self) -> Box<dyn Constraint> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct MaxConstraint {
    value: i32,
}

impl MaxConstraint {
    pub fn new(value: i32) -> Self {
        MaxConstraint { value }
    }
}

impl Constraint for MaxConstraint {
    fn validate(&self, object: &Object) -> bool {
        matches!(object, Object::Int(v) if *v <= self.value as isize)
    }

    fn description(&self) -> String {
        format!("max({})", self.value)
    }

    fn build_sampler(&self) -> Option<Box<dyn Sampler>> {
        Some(Box::new(UniformSampler::new(
            i32::MIN,
            self.value,
        )))
    }

    fn clone_box(&self) -> Box<dyn Constraint> {
        Box::new(self.clone())
    }
}
