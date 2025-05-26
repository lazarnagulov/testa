use crate::{
    evaluator::{
        context::{Constraint, Context, Visitor},
        eval_error::EvalError,
        evaluator::evaluate_expression,
        object::Object,
    },
    parser::ast::{ConstraintExpression, ConstraintKind, DataType, DataTypeKind},
};

use super::constraints::{MultipleOfConstraint, RangeConstraint};

#[derive(Debug)]
pub struct ConstrainedType {
    pub type_kind: DataTypeKind,
    pub constraints: Vec<Box<dyn Constraint>>,
}

impl ConstrainedType {
    pub fn new(data_type: DataType, context: &Context) -> Result<Self, EvalError> {
        let Some(constraints) = data_type.constraints else {
            return Ok(ConstrainedType {
                type_kind: data_type.kind,
                constraints: Vec::new(),
            });
        };
        let evaluated_constraints = ConstrainedType::evaluate_constraints(&constraints, context)?;
        Ok(ConstrainedType {
            type_kind: data_type.kind,
            constraints: evaluated_constraints,
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

    pub fn extend_with(&mut self, _constraints: Vec<Box<dyn Constraint>>) -> Self {
        todo!()
    }
}

impl Visitor<Object> for ConstrainedType {
    fn visit(&self, _context: &Context) -> Result<Object, EvalError> {
        todo!()
    }
}
