use rand::{distr::Uniform, RngCore};

use crate::{evaluator::{context::{Constraint, Context}, eval_error::EvalError, evaluator::evaluate_expression, object::Object}, parser::ast::{ConstraintKind, DataType, DataTypeKind}};

#[derive(Debug)]
pub struct ConstrainedType {
    pub type_kind: DataTypeKind,
    pub constraints: Vec<Box<dyn Constraint>>,
}

impl ConstrainedType {
    pub fn new(data_type: DataType, context: &Context) -> Result<Self, EvalError> {
        let Some(constraints) = data_type.constraints else {
            return Ok(ConstrainedType { type_kind: data_type.kind, constraints: Vec::new() });
        };
        let mut evaluated_constraints: Vec<Box<dyn Constraint>> = vec![];
        for constraint in constraints {
            let object = evaluate_expression(&constraint.expression, context)?;
            match constraint.kind {
                ConstraintKind::Range => {
                    let (start, end) = match object {
                        Object::Range(start, end) => Ok((start, end)),
                        obj => Err(EvalError::type_error("int".to_owned(), format!("{}", obj)))
                    }?;
                    evaluated_constraints.push(Box::new(RangeConstraint::new(start, end)));
                },
                ConstraintKind::MultipleOf => {
                    let integer = match object {
                        Object::Int(value) => Ok(value),
                        obj => Err(EvalError::type_error("int".to_owned(), format!("{}", obj)))
                    }?;
                    evaluated_constraints.push(Box::new(MultipleOfConstraint::new(integer as i32)));
                },
                _ => todo!()
            }
        }

        Ok(ConstrainedType { type_kind: data_type.kind, constraints: evaluated_constraints })
    }
    
    pub fn generate(&self, _rng: &mut dyn RngCore, _max_attempts: usize) -> Option<Object> {
        todo!()
    }
}


#[derive(Debug, Clone)]
pub struct RangeConstraint {
    pub min: isize,
    pub max: isize,
}

impl RangeConstraint {
    fn new(min: isize, max: isize) -> Self {
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
    
    fn build_sampler(&self) -> Option<Box<dyn std::any::Any>> {
        let dist = Uniform::new_inclusive(self.min as i32, self.max as i32);
        Some(Box::new(dist))
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
        todo!()
    }

    fn build_sampler(&self) -> Option<Box<dyn std::any::Any>> {
        None
    }
}