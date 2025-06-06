#![allow(dead_code)]
use crate::core::{
    ast::nodes::{
        Attribute, ConstraintExpression, DataTypeKind, Expression, ExpressionKind, InfixOperator, PrefixOperator, Program, Statement, Variant
    },
    ir::tac::{BaseType, Block, Instruction, Value, IR},
};

pub struct IRGenerator {
    blocks: Vec<Block>,
    current_block: usize,
    temp_val_id: usize,
}

impl Default for IRGenerator {
    fn default() -> Self {
        IRGenerator {
            temp_val_id: 0,
            blocks: vec![Block {
                label: "DECLARATIONS".to_string(),
                instructions: vec![],
            }, Block {
                label: "ENTRY".to_string(),
                instructions: vec![],
            }],
            current_block: 0,
        }
    }
}

impl IRGenerator {
    fn get_id(&mut self) -> usize {
        let id = self.temp_val_id;
        self.temp_val_id += 1;
        id
    }

    fn add_instruction(&mut self, instr: Instruction) {
        self.blocks[self.current_block].instructions.push(instr);
    }

    pub fn generate(mut self, program: Program) -> IR {
        program.0.iter().for_each(|statement| {
            match statement {
                Statement::Expression(statement) => self.generate_expression(&statement.expression),
                Statement::Template { .. } => todo!(),
                Statement::OutputDirective { .. } => todo!(),
                Statement::OutputPathDirective { .. } => todo!(),
                Statement::TypeDecl { name, data_type, attributes } => self.generate_type_declaration(name, data_type, attributes),
                Statement::ConstraintDecl { .. } => todo!(),
                Statement::Enum { name, variants, attributes } => self.generate_enum(name, variants, attributes),
                Statement::Resource { .. } => todo!(),
                Statement::Generate { .. } => todo!(),
            };
        });
        IR(self.blocks)
    }

    fn generate_expression(&mut self, expression: &Expression) -> Value {
        match &expression.kind {
            ExpressionKind::IntLiteral(value) => Value::Int(*value as i32),
            ExpressionKind::FloatLiteral(value) => Value::Float(value.parse::<f32>().unwrap()),
            ExpressionKind::StringLiteral(value) => Value::Str(value.clone()),
            ExpressionKind::BooleanLiteral(value) => Value::Boolean(*value),
            ExpressionKind::StringPattern(..) => todo!(),
            ExpressionKind::Identifier(identifier) => Value::Identifier(identifier.clone()),
            ExpressionKind::List(..) => todo!(),
            ExpressionKind::Type(..) => todo!(),
            ExpressionKind::Prefix {
                operator,
                expression,
            } => {
                let value = self.generate_expression(expression);
                match operator {
                    PrefixOperator::BitNegate => {
                        let temp_id = self.get_id();
                        self.add_instruction(Instruction::BitNegate {
                            target: Value::Temp(temp_id),
                            source: value,
                        });
                        Value::Temp(temp_id)
                    }
                    PrefixOperator::LogicalNegate => todo!(),
                    PrefixOperator::Negative => {
                        let temp_id = self.get_id();
                        self.add_instruction(Instruction::Negative {
                            target: Value::Temp(temp_id),
                            source: value,
                        });
                        Value::Temp(temp_id)
                    }
                }
            }
            ExpressionKind::Infix {
                left,
                operator,
                right,
            } => {
                let left_value = self.generate_expression(left);
                let right_value = self.generate_expression(right);
                match operator {
                    InfixOperator::Plus => {
                        let temp_id = self.get_id();
                        self.add_instruction(Instruction::Add {
                            target: Value::Temp(temp_id),
                            left: left_value,
                            right: right_value,
                        });
                        Value::Temp(temp_id)
                    }
                    InfixOperator::Minus => {
                        let temp_id = self.get_id();
                        self.add_instruction(Instruction::Sub {
                            target: Value::Temp(temp_id),
                            left: left_value,
                            right: right_value,
                        });
                        Value::Temp(temp_id)
                    }
                    InfixOperator::Divide => {
                        let temp_id = self.get_id();
                        self.add_instruction(Instruction::Div {
                            target: Value::Temp(temp_id),
                            left: left_value,
                            right: right_value,
                        });
                        Value::Temp(temp_id)
                    }
                    InfixOperator::Mod => {
                        let temp_id = self.get_id();
                        self.add_instruction(Instruction::Mod {
                            target: Value::Temp(temp_id),
                            left: left_value,
                            right: right_value,
                        });
                        Value::Temp(temp_id)
                    }
                    InfixOperator::Multiply => {
                        let temp_id = self.get_id();
                        self.add_instruction(Instruction::Mul {
                            target: Value::Temp(temp_id),
                            left: left_value,
                            right: right_value,
                        });
                        Value::Temp(temp_id)
                    }
                    InfixOperator::BitAnd => {
                        let temp_id = self.get_id();
                        self.add_instruction(Instruction::BitAnd {
                            target: Value::Temp(temp_id),
                            left: left_value,
                            right: right_value,
                        });
                        Value::Temp(temp_id)
                    }
                    InfixOperator::BitOr => {
                        let temp_id = self.get_id();
                        self.add_instruction(Instruction::BitOr {
                            target: Value::Temp(temp_id),
                            left: left_value,
                            right: right_value,
                        });
                        Value::Temp(temp_id)
                    }
                    InfixOperator::BitXor => {
                        let temp_id = self.get_id();
                        self.add_instruction(Instruction::BitXor {
                            target: Value::Temp(temp_id),
                            left: left_value,
                            right: right_value,
                        });
                        Value::Temp(temp_id)
                    }
                    InfixOperator::BitLShift => {
                        let temp_id = self.get_id();
                        self.add_instruction(Instruction::LShift {
                            target: Value::Temp(temp_id),
                            left: left_value,
                            right: right_value,
                        });
                        Value::Temp(temp_id)
                    }
                    InfixOperator::BitRShift => {
                        let temp_id = self.get_id();
                        self.add_instruction(Instruction::RShift {
                            target: Value::Temp(temp_id),
                            left: left_value,
                            right: right_value,
                        });
                        Value::Temp(temp_id)
                    }
                    _ => todo!(),
                }
            }
            ExpressionKind::FuncCall { .. } => todo!(),
        }
    }

    fn generate_attributes(&mut self, attributes: &[Attribute]) {
        for attribute in attributes {
            self.add_instruction(Instruction::Attr {
                name: attribute.name.clone(),
            });
        }
    }

    fn generate_constraints(&mut self, constraints: &[ConstraintExpression]) {
            for constraint in constraints {
                let value = self.generate_expression(&constraint.expression);

                self.add_instruction(Instruction::Constraint {
                    name: constraint.kind.to_string().clone(),
                    value
                });
            }
    }

    fn generate_enum(
        &mut self,
        name: &str,
        variants: &[Variant],
        attributes: &[Attribute],
    ) -> Value {
        self.current_block = 0;
        self.add_instruction(Instruction::BeginEnum { name: name.to_owned() });
        self.generate_attributes(attributes);
        for variant in variants {
            let weight = match &variant.weight {
                Some(expression) => self.generate_expression(expression),
                None => Value::Int(1),
            }; 
            self.add_instruction(Instruction::EnumVariant {
                name: variant.name.clone(),
                weight 
          });
        }

        self.add_instruction(Instruction::End);
        Value::NoValue
    }
    
    fn generate_type_declaration(&mut self, name: &str, data_type: &Expression, attributes: &[Attribute]) -> Value {
        self.current_block = 0;
        let ExpressionKind::Type(data_type) = &data_type.kind else {unreachable!()}; 
        let base_type = match &data_type.kind {
            DataTypeKind::Int => BaseType::Int,
            DataTypeKind::Str => BaseType::Str,
            DataTypeKind::Float => BaseType::Float,
            DataTypeKind::Boolean => BaseType::Bool,
            DataTypeKind::List(..) => todo!(),
            DataTypeKind::Custom(_) => todo!(),
        };
        self.add_instruction(Instruction::BegindType { name: name.to_owned(), base_type  });
        self.generate_attributes(attributes);
        if let Some(constraints) = &data_type.constraints {
            self.generate_constraints(constraints);
        }
        self.add_instruction(Instruction::End);
        Value::NoValue
    }

}
