use crate::{
    analyser::{
        error::SemanticError,
        symbol_table::symbol::SymbolKind,
        type_checker::{TypeChecker, types::Type},
    },
    ast::{Element, Expression, ExpressionKind},
};

impl<'a> TypeChecker<'a> {
    pub(crate) fn infer_type(&mut self, expr: &Expression) -> Type {
        match &expr.kind {
            ExpressionKind::IntLiteral(_) => Type::Int,
            ExpressionKind::FloatLiteral(_) => Type::Float,
            ExpressionKind::StringLiteral(_) => Type::Str,
            ExpressionKind::BooleanLiteral(_) => Type::Boolean,
            ExpressionKind::StringPattern(_) => Type::Str,
            ExpressionKind::Identifier(name) => self.get_identifier_type(name),
            ExpressionKind::List(elements) => self.infer_list_type(elements),
            ExpressionKind::Type(data_type) => {
                let base_type = Type::from(data_type);
                self.check_constraints(data_type, &base_type);
                base_type
            }

            ExpressionKind::Prefix {
                operator,
                expression,
            } => self.check_prefix_op(*operator, expression),

            ExpressionKind::Infix {
                left,
                operator,
                right,
            } => self.check_infix_op(left, *operator, right),

            ExpressionKind::FuncCall { arguments: _ } => {
                // Functions not yet implemented
                Type::Unknown
            }
        }
    }

    pub(crate) fn get_identifier_type(&mut self, name: &str) -> Type {
        if let Some(symbol) = self.symbol_table.lookup(name) {
            match &symbol.kind {
                SymbolKind::Field { expression, .. } => self.infer_type(expression),
                SymbolKind::TypeAlias { data_type, .. } => {
                    if let ExpressionKind::Type(dt) = &data_type.kind {
                        Type::from(dt)
                    } else {
                        Type::Unknown
                    }
                }
                SymbolKind::Enum { .. } => Type::Custom(name.to_string()),
                SymbolKind::Variant { enum_name } => Type::Custom(enum_name.clone()),
                _ => Type::Unknown,
            }
        } else {
            Type::Unknown
        }
    }

    pub(crate) fn infer_list_type(&mut self, elements: &[Element]) -> Type {
        if elements.is_empty() {
            return Type::List(Box::new(Type::Unknown));
        }

        let first_type = self.infer_type(&elements[0].value);

        for element in elements.iter().skip(1) {
            let elem_type = self.infer_type(&element.value);

            if !first_type.is_compatible_with(&elem_type) {
                self.errors.push(SemanticError::TypeMismatch {
                    expected: first_type.display(),
                    found: elem_type.display(),
                    span: element.span,
                });
            }
        }

        for element in elements {
            if let Some(weight) = &element.weight {
                let weight_type = self.infer_type(weight);
                if !weight_type.is_numeric() && !weight_type.is_unknown() {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: "int".to_string(),
                        found: weight_type.display(),
                        span: weight.span,
                    });
                }
            }
        }

        Type::List(Box::new(first_type))
    }
}
