use core::fmt;
use once_cell::sync::Lazy;
use std::{collections::HashMap, fmt::Debug, i32};

use crate::{
    evaluator::{eval_error::EvalError, object::Object},
    parser::ast::{ConstraintKind, DataTypeKind},
};

use super::{
    constrainted_type::Constraint,
    sampler::{BooleanSampler, IdentitySampler, Sampler, UniformSampler},
    util,
};

pub trait ConstraintBuilder: Send + Sync + fmt::Debug {
    fn is_compatible(&self, type_kind: &DataTypeKind) -> bool;
    fn build(&self, object: &Object) -> Result<Box<dyn Constraint>, EvalError>;
}

// TODO: Make it thread safe later
pub static CONSTRAINT_REGISTRY: Lazy<HashMap<ConstraintKind, Box<dyn ConstraintBuilder>>> =
    Lazy::new(|| {
        let mut m: HashMap<ConstraintKind, Box<dyn ConstraintBuilder>> = HashMap::new();
        m.insert(ConstraintKind::Range, Box::new(RangeBuilder));
        m.insert(ConstraintKind::MultipleOf, Box::new(MultipleOfBuilder));
        m.insert(ConstraintKind::Bias, Box::new(BiasBuilder));
        m.insert(ConstraintKind::Min, Box::new(MinBuilder));
        m.insert(ConstraintKind::Max, Box::new(MaxBuilder));
        m.insert(ConstraintKind::Count, Box::new(CountBuilder));
        m
    });

#[derive(Debug)]
pub struct RangeBuilder;

impl ConstraintBuilder for RangeBuilder {
    fn is_compatible(&self, type_kind: &DataTypeKind) -> bool {
        matches!(type_kind, DataTypeKind::Int | DataTypeKind::Float)
    }

    fn build(&self, object: &Object) -> Result<Box<dyn Constraint>, EvalError> {
        let (start, end) = util::extract_range(object)?;
        Ok(Box::new(RangeConstraint::new(start, end)))
    }
}

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
        if self.min == self.max {
            Some(Box::new(IdentitySampler::new(self.min)))
        } else {
            Some(Box::new(UniformSampler::new(
                self.min as i32,
                self.max as i32,
            )))
        }
    }

    fn clone_box(&self) -> Box<dyn Constraint> {
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct MultipleOfBuilder;

impl ConstraintBuilder for MultipleOfBuilder {
    fn is_compatible(&self, type_kind: &DataTypeKind) -> bool {
        *type_kind == DataTypeKind::Int
    }

    fn build(&self, object: &Object) -> Result<Box<dyn Constraint>, EvalError> {
        let integer = util::extract_int(object)?;
        Ok(Box::new(MultipleOfConstraint::new(integer)))
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

#[derive(Debug)]
pub struct BiasBuilder;

impl ConstraintBuilder for BiasBuilder {
    fn is_compatible(&self, type_kind: &DataTypeKind) -> bool {
        *type_kind == DataTypeKind::Boolean
    }

    fn build(&self, object: &Object) -> Result<Box<dyn Constraint>, EvalError> {
        let float = util::extract_float(object)?;
        Ok(Box::new(BiasConstraint::new(float)))
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

#[derive(Debug)]
pub struct MinBuilder;

impl ConstraintBuilder for MinBuilder {
    fn is_compatible(&self, type_kind: &DataTypeKind) -> bool {
        matches!(type_kind, DataTypeKind::Float | DataTypeKind::Int)
    }

    fn build(&self, object: &Object) -> Result<Box<dyn Constraint>, EvalError> {
        let integer = util::extract_int(object)?;
        Ok(Box::new(MinConstraint::new(integer)))
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
        Some(Box::new(UniformSampler::new(self.value, i32::MAX)))
    }

    fn clone_box(&self) -> Box<dyn Constraint> {
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct MaxBuilder;

impl ConstraintBuilder for MaxBuilder {
    fn is_compatible(&self, type_kind: &DataTypeKind) -> bool {
        matches!(type_kind, DataTypeKind::Float | DataTypeKind::Int)
    }

    fn build(&self, object: &Object) -> Result<Box<dyn Constraint>, EvalError> {
        let integer = util::extract_int(object)?;
        Ok(Box::new(MaxConstraint::new(integer)))
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
        Some(Box::new(UniformSampler::new(i32::MIN, self.value)))
    }

    fn clone_box(&self) -> Box<dyn Constraint> {
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct CountBuilder;

impl ConstraintBuilder for CountBuilder {
    fn is_compatible(&self, type_kind: &DataTypeKind) -> bool {
        matches!(type_kind, DataTypeKind::List(_))
    }

    fn build(&self, object: &Object) -> Result<Box<dyn Constraint>, EvalError> {
        match util::extract_int(object) {
            Ok(integer) => Ok(Box::new(CountConstraint::exact(integer))),
            Err(_) => {
                let (start, end) = util::extract_range(object)?;
                Ok(Box::new(CountConstraint::new(start as i32, end as i32)))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CountConstraint {
    pub min: i32,
    pub max: i32,
}

impl CountConstraint {
    pub fn new(min: i32, max: i32) -> Self {
        CountConstraint { min, max }
    }
    pub fn exact(value: i32) -> Self {
        CountConstraint {
            min: value,
            max: value,
        }
    }
}

impl Constraint for CountConstraint {
    fn validate(&self, value: &Object) -> bool {
        match value {
            Object::List(objects) => {
                objects.len() as i32 >= self.min && objects.len() as i32 <= self.max
            }
            _ => false,
        }
    }

    fn description(&self) -> String {
        todo!()
    }

    fn build_sampler(&self) -> Option<Box<dyn Sampler>> {
        if self.min == self.max {
            Some(Box::new(IdentitySampler::new(self.min)))
        } else {
            Some(Box::new(UniformSampler::new(
                self.min as i32,
                self.max as i32,
            )))
        }
    }

    fn clone_box(&self) -> Box<dyn Constraint> {
        Box::new(self.clone())
    }
}
