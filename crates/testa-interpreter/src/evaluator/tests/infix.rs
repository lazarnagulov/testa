use testa_core::ast::{ExpressionKind, InfixOperator};

use crate::object::Object;

infix_tests! {
    test_add_ints:
        ExpressionKind::IntLiteral(2),
        InfixOperator::Plus,
        ExpressionKind::IntLiteral(3)
        => Object::Int(5),

    test_add_floats:
        ExpressionKind::FloatLiteral("2.5".into()),
        InfixOperator::Plus,
        ExpressionKind::FloatLiteral("1.5".into())
        => Object::Float(4.0),

    test_concat_strings:
        ExpressionKind::StringLiteral("Hello ".into()),
        InfixOperator::Plus,
        ExpressionKind::StringLiteral("World".into())
        => Object::String("Hello World".into()),
}

infix_error_tests! {
    test_int_plus_bool_error:
        ExpressionKind::IntLiteral(1),
        InfixOperator::Plus,
        ExpressionKind::BooleanLiteral(true),

    test_bool_and_int_error:
        ExpressionKind::BooleanLiteral(false),
        InfixOperator::And,
        ExpressionKind::IntLiteral(0),

    test_devide_by_zero_error:
        ExpressionKind::IntLiteral(2),
        InfixOperator::Divide,
        ExpressionKind::IntLiteral(0),
}
