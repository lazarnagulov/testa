use std::{cell::RefCell, rc::Rc};

use rand::{Rng, distr::Alphanumeric};

use crate::{
    evaluator::{
        context::{Context, Visitor},
        eval_error::EvalError,
        evaluator::{evaluate_data_type, evaluate_expression, evaluate_identifier},
        object::Object,
    },
    parser::ast::{ConstraintExpression, ConstraintKind, DataType, DataTypeKind},
};

use super::{
    constraints::{
        BiasConstraint, MaxConstraint, MinConstraint, MultipleOfConstraint, RangeConstraint,
    },
    sampler::{ConstraintSet, Sampler},
};

pub trait Constraint: std::fmt::Debug {
    fn validate(&self, value: &Object) -> bool;
    fn description(&self) -> String;
    fn build_sampler(&self) -> Option<Box<dyn Sampler>>;

    fn clone_box(&self) -> Box<dyn Constraint>;
}

impl Clone for Box<dyn Constraint> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug)]
pub struct ConstrainedType {
    pub parent: Option<Rc<ConstrainedType>>,
    pub type_kind: DataTypeKind,
    pub constraints: Vec<Box<dyn Constraint>>,
    pub cached_sampler: RefCell<Option<Box<dyn Sampler>>>,
}

impl ConstrainedType {
    pub fn new(data_type: DataType, context: &Context) -> Result<Self, EvalError> {
        let parent = match &data_type.kind {
            DataTypeKind::Custom(parent_name) => {
                context.get_type(&parent_name).map(|rc| Rc::clone(rc))
            }
            _ => None,
        };

        let Some(constraints) = data_type.constraints else {
            return Ok(ConstrainedType {
                parent: None,
                type_kind: data_type.kind,
                constraints: Vec::new(),
                cached_sampler: RefCell::new(None),
            });
        };
        let evaluated_constraints = ConstrainedType::evaluate_constraints(&constraints, context)?;
        Ok(ConstrainedType {
            parent,
            type_kind: data_type.kind,
            constraints: evaluated_constraints,
            cached_sampler: RefCell::new(None),
        })
    }

    pub fn evaluate_constraints(
        constraints: &Vec<ConstraintExpression>,
        context: &Context,
    ) -> Result<Vec<Box<dyn Constraint>>, EvalError> {
        let mut evaluated_constraints: Vec<Box<dyn Constraint>> = vec![];
        for constraint in constraints {
            let object = evaluate_expression(&constraint.expression, context)?;
            match constraint.kind {
                ConstraintKind::Range => {
                    let (start, end) = ConstrainedType::extract_range(&object)?;
                    evaluated_constraints.push(Box::new(RangeConstraint::new(start, end)));
                }
                ConstraintKind::MultipleOf => {
                    let integer = ConstrainedType::extract_int(&object)?;
                    evaluated_constraints.push(Box::new(MultipleOfConstraint::new(integer)));
                }
                ConstraintKind::Bias => {
                    let float = ConstrainedType::extract_float(&object)?;
                    evaluated_constraints.push(Box::new(BiasConstraint::new(float)));
                }
                ConstraintKind::Min => {
                    let integer = ConstrainedType::extract_int(&object)?;
                    evaluated_constraints.push(Box::new(MinConstraint::new(integer)));
                }
                ConstraintKind::Max => {
                    let integer = ConstrainedType::extract_int(&object)?;
                    evaluated_constraints.push(Box::new(MaxConstraint::new(integer)));
                }
                _ => todo!("add new kind to ConstrainedType::evaluate_constraints()"),
            }
        }
        Ok(evaluated_constraints)
    }

    fn extract_int(object: &Object) -> Result<i32, EvalError> {
       match object {
            Object::Int(value) => Ok((*value) as i32),
            obj => Err(EvalError::type_error("int".to_owned(), format!("{}", obj))),
        }   
    }

    fn extract_float(object: &Object) -> Result<f32, EvalError> {
        match object {
            Object::Float(value) => Ok(*value),
            obj => Err(EvalError::type_error("float".to_owned(), format!("{}", obj))),
        }
    }

    fn extract_range(object: &Object) -> Result<(isize, isize), EvalError> {
        match object {
            Object::Range(start, end) => Ok((*start, *end)),
            obj => Err(EvalError::type_error("int".to_owned(), format!("{}", obj))),
        }
    }

    fn collect_constraints(&self) -> Vec<Box<dyn Constraint>> {
        let mut all_constraints: Vec<Box<dyn Constraint>> = vec![];
        let mut current = Some(self);

        while let Some(current_type) = current {
            all_constraints.extend(current_type.constraints.iter().cloned());
            current = current_type.parent.as_deref();
        }

        all_constraints
    }

    fn generate_without_constraints(&self, context: &Context) -> Result<Object, EvalError> {
        let mut rng = rand::rng();
        return match &self.type_kind {
            DataTypeKind::Int => Ok(Object::new(rng.random::<i32>() as isize)),
            DataTypeKind::Str => {
                let size = rng.random_range(6..=20);
                let value: String = rng
                    .sample_iter(&Alphanumeric)
                    .take(size)
                    .map(char::from)
                    .collect();
                Ok(Object::new(value))
            }
            DataTypeKind::Boolean => Ok(Object::new(rng.random_bool(50.0))),
            DataTypeKind::Float => Ok(Object::new(rng.random::<f32>())),
            DataTypeKind::List(data_type) => {
                let count = rng.random_range(0..=16);
                let mut values = vec![];
                for _ in 0..count {
                    values.push(evaluate_data_type(data_type, context)?);
                }
                Ok(Object::new(values))
            }
            DataTypeKind::Custom(name) => evaluate_identifier(name, context),
        };
    }
}

impl Visitor<Object> for ConstrainedType {
    fn visit(&self, context: &Context) -> Result<Object, EvalError> {
        if self.constraints.is_empty() {
            return self.generate_without_constraints(context);
        }

        if let Some(sampler) = self.cached_sampler.borrow().as_ref() {
            return Ok(sampler.sample());
        };
        let constraints_set = ConstraintSet::new(self.collect_constraints());
        let sampler = constraints_set.build_sampler();

        *self.cached_sampler.borrow_mut() = Some(sampler);
        Ok(self.cached_sampler.borrow().as_ref().unwrap().sample())
    }
}
