use crate::core::{ast::nodes::DataTypeKind, semantics::context::Context};

pub fn get_fundamental_type<'a>(kind: &'a DataTypeKind, context: &'a Context) -> &'a DataTypeKind {
    match kind {
        DataTypeKind::Int
        | DataTypeKind::Str
        | DataTypeKind::Float
        | DataTypeKind::Boolean
        | DataTypeKind::List(_) => kind,
        DataTypeKind::Custom(name) => {
            let data_type = context.get_type(name).unwrap();
            get_fundamental_type(&data_type.type_kind, context)
        }
    }
}
