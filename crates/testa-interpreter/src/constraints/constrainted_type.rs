// use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// use rand::{Rng, distr::Alphanumeric};
// use testa_core::ast::{ConstraintExpression, DataType, DataTypeKind};

// use crate::{
//     constraints::{
//         constraint::CONSTRAINT_REGISTRY,
//         sampler::{ConstraintSet, Sampler},
//     },
//     evaluator,
//     evaluator::context::{Context, Visitor},
//     evaluator::error::EvalError,
//     object::Object,
// };

// pub trait Constraint: std::fmt::Debug {
//     fn validate(&self, value: &Object) -> bool;
//     fn description(&self) -> String;
//     fn build_sampler(&self) -> Option<Box<dyn Sampler>>;

//     fn clone_box(&self) -> Box<dyn Constraint>;
// }

// impl Clone for Box<dyn Constraint> {
//     fn clone(&self) -> Self {
//         self.clone_box()
//     }
// }

// #[derive(Debug)]
// pub struct ConstrainedType {
//     pub parent: Option<Rc<ConstrainedType>>,
//     pub type_kind: DataTypeKind,
//     pub constraints: Vec<Box<dyn Constraint>>,
//     pub cached_sampler: RefCell<Option<Box<dyn Sampler>>>,
// }

// impl ConstrainedType {
//     pub fn new(data_type: DataType, context: &Context) -> Result<Self, EvalError> {
//         let parent = ConstrainedType::find_parent(&data_type.kind, context)?;
//         let Some(constraints) = data_type.constraints else {
//             return Ok(ConstrainedType {
//                 parent: None,
//                 type_kind: data_type.kind,
//                 constraints: Vec::new(),
//                 cached_sampler: RefCell::new(None),
//             });
//         };
//         let fundamental_type = ConstrainedType::get_fundamental_type(&data_type.kind, context);
//         let evaluated_constraints =
//             ConstrainedType::evaluate_constraints(&constraints, fundamental_type, context)?;
//         Ok(ConstrainedType {
//             parent,
//             type_kind: data_type.kind,
//             constraints: evaluated_constraints,
//             cached_sampler: RefCell::new(None),
//         })
//     }

//     pub fn evaluate_constraints(
//         constraints: &Vec<ConstraintExpression>,
//         type_kind: &DataTypeKind,
//         context: &Context,
//     ) -> Result<Vec<Box<dyn Constraint>>, EvalError> {
//         let mut evaluated_constraints =
//             ConstrainedType::build_constraints(constraints, type_kind, context)?;

//         if let DataTypeKind::List(inner_type) = type_kind {
//             let mut inner_type = inner_type;
//             if let Some(constraints) = &inner_type.constraints {
//                 evaluated_constraints.extend(ConstrainedType::build_constraints(
//                     constraints,
//                     &inner_type.kind,
//                     context,
//                 )?);
//             }
//             while let DataTypeKind::List(inner_type_inner) = &inner_type.kind {
//                 if let Some(constraints) = &inner_type_inner.constraints {
//                     evaluated_constraints.extend(ConstrainedType::build_constraints(
//                         constraints,
//                         &inner_type_inner.kind,
//                         context,
//                     )?);
//                 } else {
//                     break;
//                 }
//                 inner_type = inner_type_inner;
//             }
//         }
//         Ok(evaluated_constraints)
//     }

//     fn build_constraints(
//         constraints: &Vec<ConstraintExpression>,
//         type_kind: &DataTypeKind,
//         context: &Context,
//     ) -> Result<Vec<Box<dyn Constraint>>, EvalError> {
//         let mut evaluated_constraints: Vec<Box<dyn Constraint>> = Vec::new();
//         for constraint in constraints {
//             let registry = &CONSTRAINT_REGISTRY;
//             let builder = registry.get(&constraint.kind).ok_or_else(|| {
//                 EvalError::NotDefined(format!("Constraint kind {:?}", constraint.kind))
//             })?;
//             let object = evaluator::evaluate_expression(&constraint.expression, context)?;

//             if !builder.is_compatible(type_kind) {
//                 return Err(EvalError::incompatible_constraint(
//                     &format!("{}", type_kind),
//                     &format!("{}", constraint.kind),
//                 ));
//             }
//             evaluated_constraints.push(builder.build(&object)?);
//         }
//         Ok(evaluated_constraints)
//     }

//     fn collect_constraints(&self) -> VecDeque<Box<dyn Constraint>> {
//         let mut all_constraints: VecDeque<Box<dyn Constraint>> = VecDeque::new();
//         let mut current = Some(self);

//         while let Some(current_type) = current {
//             all_constraints.extend(current_type.constraints.iter().cloned());
//             current = current_type.parent.as_deref();
//         }

//         all_constraints
//     }

//     fn find_parent(
//         type_kind: &DataTypeKind,
//         context: &Context,
//     ) -> Result<Option<Rc<ConstrainedType>>, EvalError> {
//         match type_kind {
//             DataTypeKind::Custom(parent_name) => {
//                 match context.get_type(parent_name).map(Rc::clone) {
//                     Some(parent) => Ok(Some(parent)),
//                     None => Err(EvalError::NotDefined(format!("Type '{}'", parent_name))),
//                 }
//             }
//             DataTypeKind::List(data_type) => ConstrainedType::find_parent(&data_type.kind, context),
//             _ => Ok(None),
//         }
//     }

//     fn get_fundamental_type<'a>(kind: &'a DataTypeKind, context: &'a Context) -> &'a DataTypeKind {
//         match kind {
//             DataTypeKind::Int
//             | DataTypeKind::Str
//             | DataTypeKind::Float
//             | DataTypeKind::Boolean
//             | DataTypeKind::List(_) => kind,
//             DataTypeKind::Custom(name) => {
//                 let data_type = context.get_type(name).unwrap();
//                 ConstrainedType::get_fundamental_type(&data_type.type_kind, context)
//             }
//         }
//     }

//     fn generate_without_constraints(&self, context: &Context) -> Result<Object, EvalError> {
//         let mut rng = rand::rng();
//         match &self.type_kind {
//             DataTypeKind::Int => Ok(Object::new(rng.random::<i32>() as isize)),
//             DataTypeKind::Str => {
//                 let size = rng.random_range(6..=20);
//                 let value: String = rng
//                     .sample_iter(&Alphanumeric)
//                     .take(size)
//                     .map(char::from)
//                     .collect();
//                 Ok(Object::new(value))
//             }
//             DataTypeKind::Boolean => Ok(Object::new(rng.random_bool(50.0))),
//             DataTypeKind::Float => Ok(Object::new(rng.random::<f32>())),
//             DataTypeKind::List(data_type) => {
//                 let count = rng.random_range(0..=16);
//                 let mut values = vec![];
//                 values.extend(
//                     (0..count)
//                         .map(|_| evaluator::evaluate_data_type(data_type, context))
//                         .collect::<Result<Vec<_>, _>>()?,
//                 );
//                 Ok(Object::new(values))
//             }
//             DataTypeKind::Custom(name) => evaluator::evaluate_identifier(name, context),
//         }
//     }

//     fn sample(&self, constraints_set: &ConstraintSet) -> Result<Object, EvalError> {
//         if let Some(sampler) = self.cached_sampler.borrow().as_ref() {
//             return Ok(sampler.sample());
//         };
//         let sampler = constraints_set.build_sampler();

//         *self.cached_sampler.borrow_mut() = Some(sampler);
//         Ok(self.cached_sampler.borrow().as_ref().unwrap().sample())
//     }
// }

// impl Visitor<Object> for ConstrainedType {
//     fn visit(&self, context: &Context) -> Result<Object, EvalError> {
//         if self.constraints.is_empty() {
//             return self.generate_without_constraints(context);
//         }

//         let mut constraints_set = ConstraintSet::new(self.collect_constraints());
//         if let DataTypeKind::List(data_type) = &self.type_kind {
//             let sampler = constraints_set.build_list_sampler().unwrap();
//             let Object::Int(count) = sampler.sample() else {
//                 unreachable!()
//             };
//             (0..count)
//                 .map(|_| evaluator::evaluate_data_type(data_type, context))
//                 .collect::<Result<Vec<_>, _>>()
//                 .map(Object::List)
//         } else {
//             self.sample(&constraints_set)
//         }
//     }
// }
