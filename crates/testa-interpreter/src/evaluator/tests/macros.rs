#![allow(unused_macros)]

macro_rules! infix_test {
    ($name:ident, $left:expr, $op:expr, $right:expr, $expected:expr) => {
        #[test]
        fn $name() {
            use testa_hir::{Module, module::Expr};

            let module = Module::empty("test");
            let context = $crate::evaluator::context::Context::new(module);
            let mut evaluator = $crate::evaluator::Evaluator::new(context, None);

            let expr = Expr::Infix {
                left: Box::new($left),
                right: Box::new($right),
                op: $op,
            };

            let result = evaluator
                .evaluate_expression(&expr)
                .expect("Evaluation failed");
            assert_eq!(result, $expected);
        }
    };
}

macro_rules! infix_tests {
    ($($name:ident: $left:expr, $op:expr, $right:expr => $expected:expr),* $(,)?) => {
        $(infix_test!($name, $left, $op, $right, $expected);)*
    };
}

macro_rules! infix_error_test {
    ($name:ident, $left:expr, $op:expr, $right:expr) => {
        #[test]
        fn $name() {
            use testa_hir::{Module, module::Expr};

            let module = Module::empty("test");
            let context = $crate::evaluator::context::Context::new(module);
            let mut evaluator = $crate::evaluator::Evaluator::new(context, None);

            let expr = Expr::Infix {
                left: Box::new($left),
                right: Box::new($right),
                op: $op,
            };

            assert!(evaluator.evaluate_expression(&expr).is_err());
        }
    };
}

macro_rules! infix_error_tests {
    ($($name:ident: $left:expr, $op:expr, $right:expr),* $(,)?) => {
        $(infix_error_test!($name, $left, $op, $right);)*
    };
}

macro_rules! prefix_test {
    ($name:ident, $op:expr, $expr:expr, $expected:expr) => {
        #[test]
        fn $name() {
            use testa_hir::{Module, module::Expr};

            let module = Module::empty("test");
            let context = $crate::evaluator::context::Context::new(module);
            let mut evaluator = $crate::evaluator::Evaluator::new(context, None);

            let expr = Expr::Prefix {
                op: $op,
                expr: Box::new($expr),
            };

            let result = evaluator
                .evaluate_expression(&expr)
                .expect("Evaluation failed");
            assert_eq!(result, $expected);
        }
    };
}

macro_rules! prefix_tests {
    ($($name:ident: $op:expr, $expr:expr => $expected:expr),* $(,)?) => {
        $(prefix_test!($name, $op, $expr, $expected);)*
    };
}

macro_rules! prefix_error_test {
    ($name:ident, $op:expr, $expr:expr) => {
        #[test]
        fn $name() {
            use testa_hir::{Module, module::Expr};

            let module = Module::empty("test");
            let context = $crate::evaluator::context::Context::new(module);
            let mut evaluator = $crate::evaluator::Evaluator::new(context, None);

            let expr = Expr::Prefix {
                op: $op,
                expr: Box::new($expr),
            };

            assert!(evaluator.evaluate_expression(&expr).is_err());
        }
    };
}

macro_rules! prefix_error_tests {
    ($($name:ident: $op:expr, $expr:expr),* $(,)?) => {
        $(prefix_error_test!($name, $op, $expr);)*
    };
}
