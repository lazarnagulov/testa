use std::fmt;

use rand::distr::{Distribution, Uniform};

use crate::evaluator::object::Object;

use super::constrainted_type::Constraint;

pub trait Sampler: std::fmt::Debug {
    fn sample(&self) -> Object;
}

pub struct ConstraintSet {
    constraints: Vec<Box<dyn Constraint>>,
}

impl ConstraintSet {
    pub fn new(constraints: Vec<Box<dyn Constraint>>) -> Self {
        Self { constraints }
    }

    pub fn build_sampler(&self) -> Box<dyn Sampler> {
        let mut samplers: Vec<Box<dyn Sampler>> = vec![];
        let mut validators: Vec<Box<dyn Fn(&Object) -> bool>> = vec![];

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
    validators: Vec<Box<dyn Fn(&Object) -> bool>>,
}

impl CompositeSampler {
    pub fn new(
        samplers: Vec<Box<dyn Sampler>>,
        validators: Vec<Box<dyn Fn(&Object) -> bool>>,
    ) -> Self {
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

#[derive(Debug)]
struct UniformSampler {
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
        let mut rng = rand::rng();
        let sample = Uniform::try_from(self.min..self.max).unwrap();
        Object::new(sample.sample(&mut rng) as isize)
    }
}
