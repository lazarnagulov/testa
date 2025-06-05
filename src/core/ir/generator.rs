#![allow(dead_code)]
use crate::core::{
    ast::nodes::{Attribute, Program, Statement, Variant},
    ir::tac::{IR, Instruction},
    semantics::context::Context,
};

pub struct IRGenerator {
    context: Context,
}

impl IRGenerator {
    pub fn new(context: Context) -> Self {
        IRGenerator { context }
    }

    pub fn generate(&mut self, program: Program) -> IR {
        for statement in &program.0 {
            let instructions = match statement {
                Statement::Expression(..) => todo!(),
                Statement::Template { .. } => todo!(),
                Statement::OutputDirective { .. } => todo!(),
                Statement::OutputPathDirective { .. } => todo!(),
                Statement::TypeDecl { .. } => todo!(),
                Statement::ConstraintDecl { .. } => todo!(),
                Statement::Enum {
                    name,
                    variants,
                    attributes,
                } => self.generate_enum(name, variants, attributes),
                Statement::Resource { .. } => todo!(),
                Statement::Generate { .. } => todo!(),
            };
        }
        todo!()
    }

    fn generate_enum(
        &mut self,
        _name: &str,
        _variants: &[Variant],
        _attributes: &[Attribute],
    ) -> Vec<Instruction> {
        todo!()
    }
}