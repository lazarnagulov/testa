use crate::object::Object;
use testa_hir::module::PrefixOp;

prefix_tests! {
    test_negate_int:
        PrefixOp::Neg, Expr::Int(5)
        => Object::Int(-5),

    test_bitwise_negate_int:
        PrefixOp::BitNeg, Expr::Int(5)
        => Object::Int(-6),

    test_not_true:
        PrefixOp::Not, Expr::Bool(true)
        => Object::Boolean(false),

    test_negate_float:
        PrefixOp::Neg, Expr::Float(2.5)
        => Object::Float(-2.5),
}

prefix_error_tests! {
    test_not_int_is_error:
        PrefixOp::Not, Expr::Int(1),
}
