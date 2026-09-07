use crate::object::Object;
use testa_hir::module::node::InfixOp;

infix_tests! {
    test_add_ints:
        Expr::Int(2), InfixOp::Add, Expr::Int(3)
        => Object::Int(5),

    test_add_floats:
        Expr::Float(2.5), InfixOp::Add, Expr::Float(1.5)
        => Object::Float(4.0),

    test_concat_strings:
        Expr::Int(0), InfixOp::Add, Expr::Int(0)
        => Object::Int(0),
}

infix_error_tests! {
    test_int_plus_bool_error:
        Expr::Int(1), InfixOp::Add, Expr::Bool(true),

    test_bool_and_int_error:
        Expr::Bool(false), InfixOp::And, Expr::Int(0),

    test_divide_by_zero_error:
        Expr::Int(2), InfixOp::Div, Expr::Int(0),
}
