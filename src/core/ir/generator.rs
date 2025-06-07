#![allow(dead_code)]
use crate::core::{
    ast::nodes::{
        Attribute, ConstraintExpression, DataTypeKind, Expression, ExpressionKind, Field,
        InfixOperator, PrefixOperator, Program, Statement, Variant,
    },
    ir::tac::{BaseType, Block, Instruction, Value, IR}
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
            blocks: vec![
                Block {
                    label: "DECLARATIONS".to_string(),
                    instructions: vec![],
                },
                Block {
                    label: "ENTRY".to_string(),
                    instructions: vec![],
                },
            ],
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
                Statement::Template {
                    parent,
                    attributes,
                    name,
                    body: fields,
                } => self.generate_template(parent.clone(), attributes, name, fields),
                Statement::OutputDirective { .. } => todo!(),
                Statement::OutputPathDirective { .. } => todo!(),
                Statement::TypeDecl {
                    name,
                    data_type,
                    attributes,
                } => self.generate_type_declaration(name, data_type, attributes),
                Statement::ConstraintDecl { .. } => todo!(),
                Statement::Enum {
                    name,
                    variants,
                    attributes,
                } => self.generate_enum(name, variants, attributes),
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
                self.generate_unary_instruction(operator, value)
            }
            ExpressionKind::Infix {
                left,
                operator,
                right,
            } => {
                let left_value = self.generate_expression(left);
                let right_value = self.generate_expression(right);
                self.generate_binary_instruction(left_value, operator, right_value)
            }
            ExpressionKind::FuncCall { .. } => todo!(),
        }
    }

    fn generate_unary_instruction(&mut self, operator: &PrefixOperator, source: Value) -> Value {
        let temp_id = self.get_id();
        let target = Value::Temp(temp_id);
        let instruction = match operator {
            PrefixOperator::BitNegate => Instruction::BitNegate { target: target.clone(), source },
            PrefixOperator::LogicalNegate => todo!("Implement LogicalNegate"),
            PrefixOperator::Negative => Instruction::Negative { target: target.clone(), source },
        };
        self.add_instruction(instruction);
        target
    }

    fn generate_binary_instruction(
        &mut self,
        left: Value,
        operator: &InfixOperator,
        right: Value,
    ) -> Value {
        let temp_id = self.get_id();
        let target = Value::Temp(temp_id);
        let instruction = match operator {
            InfixOperator::Plus => Instruction::Add {
                target: target.clone(),
                left,
                right,
            },
            InfixOperator::Minus => Instruction::Sub {
                target: target.clone(),
                left,
                right,
            },
            InfixOperator::Divide => Instruction::Div {
                target: target.clone(),
                left,
                right,
            },
            InfixOperator::Mod => Instruction::Mod {
                target: target.clone(),
                left,
                right,
            },
            InfixOperator::Multiply => Instruction::Mul {
                target: target.clone(),
                left,
                right,
            },
            InfixOperator::BitAnd => Instruction::BitAnd {
                target: target.clone(),
                left,
                right,
            },
            InfixOperator::BitOr => Instruction::BitOr {
                target: target.clone(),
                left,
                right,
            },
            InfixOperator::BitXor => Instruction::BitXor {
                target: target.clone(),
                left,
                right,
            },
            InfixOperator::BitLShift => Instruction::LShift {
                target: target.clone(),
                left,
                right,
            },
            InfixOperator::BitRShift => Instruction::RShift {
                target: target.clone(),
                left,
                right,
            },
            op => todo!("Implement binary op: {op}"),
        };
        self.add_instruction(instruction);
        target
    }

    fn generate_attributes(&mut self, attributes: &[Attribute]) {
        for attribute in attributes {
            let name = attribute.name.to_string();
            self.add_instruction(Instruction::Attr { name });
        }
    }

    fn generate_constraints(&mut self, constraints: &[ConstraintExpression]) {
        for constraint in constraints {
            let value = self.generate_expression(&constraint.expression);
            let name = constraint.kind.to_string();

            self.add_instruction(Instruction::Constraint { name, value });
        }
    }

    fn generate_enum(
        &mut self,
        name: &str,
        variants: &[Variant],
        attributes: &[Attribute],
    ) -> Value {
        self.current_block = 0;
        self.add_instruction(Instruction::BeginEnum {
            name: name.to_owned(),
        });
        self.generate_attributes(attributes);
        self.generate_variants(variants);

        self.add_instruction(Instruction::EndEnum);
        Value::NoValue
    }

    fn generate_variants(&mut self, variants: &[Variant]) {
        for variant in variants {
            let weight = variant
                .weight
                .as_ref()
                .map(|expr| self.generate_expression(expr))
                .unwrap_or(Value::Int(1));

            let name = variant.name.clone();
            self.add_instruction(Instruction::EnumVariant { name, weight });
        }
    }

    fn generate_type_declaration(
        &mut self,
        name: &str,
        data_type: &Expression,
        attributes: &[Attribute],
    ) -> Value {
        self.current_block = 0;
        let ExpressionKind::Type(data_type) = &data_type.kind else {
            unreachable!()
        };
        let base_type = match &data_type.kind {
            DataTypeKind::Int => BaseType::Int,
            DataTypeKind::Str => BaseType::Str,
            DataTypeKind::Float => BaseType::Float,
            DataTypeKind::Boolean => BaseType::Bool,
            DataTypeKind::List(..) => todo!(),
            DataTypeKind::Custom(_) => todo!(),
        };
        self.add_instruction(Instruction::BegindType {
            name: name.to_owned(),
            base_type,
        });
        self.generate_attributes(attributes);
        if let Some(constraints) = &data_type.constraints {
            self.generate_constraints(constraints);
        }
        self.add_instruction(Instruction::EndType);
        Value::NoValue
    }

    fn generate_template(
        &mut self,
        parent: Option<String>,
        attributes: &[Attribute],
        name: &str,
        fields: &[Field],
    ) -> Value {
        self.current_block = 0;
        self.add_instruction(Instruction::BeginTemplate {
            name: name.to_owned(),
        });
        if let Some(parent) = parent {
            self.add_instruction(Instruction::TemplateExtends { name: parent });
        }
        self.generate_attributes(attributes);
        self.generate_fields(fields);
        self.add_instruction(Instruction::EndTemplate);
        Value::NoValue
    }

    fn generate_fields(&mut self, fields: &[Field]) {
        for field in fields {
            self.add_instruction(Instruction::BeginField {
                name: field.name.clone(),
            });
            self.generate_attributes(&field.attributes);
            let value = self.generate_expression(&field.value);
            self.add_instruction(Instruction::Assign {
                target: Value::Identifier(field.name.clone()),
                source: value,
            });
            self.add_instruction(Instruction::EndField);
        }
    }
}
