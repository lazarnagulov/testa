use testa_core::ast::{ExpressionKind, PrefixOperator};

use crate::object::Object;

prefix_tests! {
    test_negate_int:
        PrefixOperator::Negative,
        ExpressionKind::IntLiteral(5)
        => Object::Int(-5),

    test_bitwise_negate_int:
        PrefixOperator::BitNegate,
        ExpressionKind::IntLiteral(5)
        => Object::Int(-6),

    test_not_true:
        PrefixOperator::LogicalNegate,
        ExpressionKind::BooleanLiteral(true)
        => Object::Boolean(false),

    test_negate_float:
        PrefixOperator::Negative,
        ExpressionKind::FloatLiteral("2.5".into())
        => Object::Float(-2.5),
}

prefix_error_tests! {
    test_not_int_is_error:
        PrefixOperator::LogicalNegate,
        ExpressionKind::IntLiteral(1),
}
