use std::{cell::RefCell, rc::Rc};

use crate::{
    evaluator::{
        context::{Context, Visitor},
        eval_error::EvalError,
        evaluator::evaluate_expression,
        object::Object,
    },
    parser::ast::{ConstraintExpression, ConstraintKind, DataType, DataTypeKind},
};

use super::{
    constraints::{MultipleOfConstraint, RangeConstraint},
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
            parent: None,
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
                    let (start, end) = match object {
                        Object::Range(start, end) => Ok((start, end)),
                        obj => Err(EvalError::type_error("int".to_owned(), format!("{}", obj))),
                    }?;
                    evaluated_constraints.push(Box::new(RangeConstraint::new(start, end)));
                }
                ConstraintKind::MultipleOf => {
                    let integer = match object {
                        Object::Int(value) => Ok(value),
                        obj => Err(EvalError::type_error("int".to_owned(), format!("{}", obj))),
                    }?;
                    evaluated_constraints.push(Box::new(MultipleOfConstraint::new(integer as i32)));
                }
                _ => todo!(),
            }
        }
        Ok(evaluated_constraints)
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
}

impl Visitor<Object> for ConstrainedType {
    fn visit(&self, _context: &Context) -> Result<Object, EvalError> {
        if let Some(sampler) = self.cached_sampler.borrow().as_ref() {
            return Ok(sampler.sample());
        };
        let constraints_set = ConstraintSet::new(self.collect_constraints());
        let sampler = constraints_set.build_sampler();

        *self.cached_sampler.borrow_mut() = Some(sampler);
        Ok(self.cached_sampler.borrow().as_ref().unwrap().sample())
    }
}
