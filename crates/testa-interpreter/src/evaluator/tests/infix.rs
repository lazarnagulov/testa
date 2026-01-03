use testa_core::ast::{ExpressionKind, InfixOperator};

use crate::object::Object;

infix_tests! {
    add_ints:
        ExpressionKind::IntLiteral(2),
        InfixOperator::Plus,
        ExpressionKind::IntLiteral(3)
        => Object::Int(5),

    add_floats:
        ExpressionKind::FloatLiteral("2.5".into()),
        InfixOperator::Plus,
        ExpressionKind::FloatLiteral("1.5".into())
        => Object::Float(4.0),

    concat_strings:
        ExpressionKind::StringLiteral("Hello ".into()),
        InfixOperator::Plus,
        ExpressionKind::StringLiteral("World".into())
        => Object::String("Hello World".into()),
}
