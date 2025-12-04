use std::{collections::VecDeque, fmt};

use rand::{
    Rng,
    distr::{Distribution, Uniform},
};

use crate::object::Object;

use super::constrainted_type::Constraint;

type ValidatorFn = Box<dyn Fn(&Object) -> bool>;

pub trait Sampler: std::fmt::Debug {
    fn sample(&self) -> Object;
}

#[derive(Debug)]
pub struct ConstraintSet {
    constraints: VecDeque<Box<dyn Constraint>>,
}

impl ConstraintSet {
    pub fn new(constraints: VecDeque<Box<dyn Constraint>>) -> Self {
        Self { constraints }
    }

    pub fn build_list_sampler(&mut self) -> Option<Box<dyn Sampler>> {
        let constraint = self.constraints.pop_front()?;
        constraint.build_sampler()
    }

    pub fn build_sampler(&self) -> Box<dyn Sampler> {
        let mut samplers: Vec<Box<dyn Sampler>> = vec![];
        let mut validators: Vec<ValidatorFn> = Vec::new();

        for constraint in &self.constraints {
            if let Some(sampler) = constraint.build_sampler() {
                samplers.push(sampler);
            }

            // TODO: thing about removing clone? Rc<>?
            let constraint_clone = constraint.clone();
            let validator = Box::new(move |obj: &Object| constraint_clone.validate(obj))
                as Box<dyn Fn(&Object) -> bool>;

            validators.push(validator);
        }

        if samplers.is_empty() {
            Box::new(UniformSampler::new(i32::MIN, i32::MAX))
        } else {
            Box::new(CompositeSampler::new(samplers, validators))
        }
    }
}

pub struct CompositeSampler {
    samplers: Vec<Box<dyn Sampler>>,
    validators: Vec<ValidatorFn>,
}

impl CompositeSampler {
    pub fn new(samplers: Vec<Box<dyn Sampler>>, validators: Vec<ValidatorFn>) -> Self {
        CompositeSampler {
            samplers,
            validators,
        }
    }
}

impl fmt::Debug for CompositeSampler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompositeSampler")
            .field("samplers", &self.samplers)
            .field(
                "validators",
                &format_args!("<{} function(s)>", self.validators.len()),
            )
            .finish()
    }
}

impl Sampler for CompositeSampler {
    fn sample(&self) -> Object {
        loop {
            for sampler in &self.samplers {
                let candidate = sampler.sample();
                if self.validators.iter().all(|v| v(&candidate)) {
                    return candidate;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdentitySampler<T> {
    pub value: T,
}

impl<T> IdentitySampler<T> {
    pub fn new(value: T) -> Self {
        IdentitySampler { value }
    }
}

impl<T> Sampler for IdentitySampler<T>
where
    T: Into<Object> + fmt::Debug + Clone,
{
    fn sample(&self) -> Object {
        Object::new(self.value.clone())
    }
}

#[derive(Debug)]
pub struct UniformSampler {
    min: i32,
    max: i32,
}

impl UniformSampler {
    pub fn new(min: i32, max: i32) -> Self {
        UniformSampler { min, max }
    }
}

impl Sampler for UniformSampler {
    fn sample(&self) -> Object {
        // TODO: do not create rng every type, extract it somewhere
        let mut rng = rand::rng();
        let sample = Uniform::try_from(self.min..self.max).unwrap();
        Object::new(sample.sample(&mut rng) as isize)
    }
}

#[derive(Debug)]
pub struct BooleanSampler {
    bias: f32,
}

impl BooleanSampler {
    pub fn new(bias: f32) -> Self {
        BooleanSampler { bias }
    }
}

impl Sampler for BooleanSampler {
    fn sample(&self) -> Object {
        let mut rng = rand::rng();
        Object::new(rng.random_bool((self.bias / 100.0) as f64))
    }
}
