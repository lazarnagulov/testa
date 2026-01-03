#![allow(unused_macros)]

macro_rules! infix_test {
    (
        $name:ident,
        $left_kind:expr,
        $op:expr,
        $right_kind:expr,
        $expected:expr
    ) => {
        #[test]
        fn $name() {
            let context = $crate::evaluator::context::Context::new(
                ::testa_core::analyser::symbol_table::SymbolTable::new(),
            );

            let evaluator = $crate::evaluator::Evaluator::new(context);

            let expression = ::testa_core::ast::Expression::new(
                ::testa_core::ast::ExpressionKind::Infix {
                    left: Box::new(::testa_core::ast::Expression {
                        kind: $left_kind,
                        span: ::testa_core::utils::Span::default(),
                    }),
                    operator: $op,
                    right: Box::new(::testa_core::ast::Expression {
                        kind: $right_kind,
                        span: ::testa_core::utils::Span::default(),
                    }),
                },
                ::testa_core::utils::Span::default(),
            );

            let result = evaluator
                .evaluate_expression(&expression)
                .expect("Evaluation failed");

            assert_eq!(result, $expected);
        }
    };
}

macro_rules! infix_tests {
    (
        $(
            $name:ident:
                $left_kind:expr,
                $op:expr,
                $right_kind:expr
                => $expected:expr
        ),* $(,)?
    ) => {
        $(
            infix_test!(
                $name,
                $left_kind,
                $op,
                $right_kind,
                $expected
            );
        )*
    };
}

macro_rules! prefix_test {
    (
        $name:ident,
        $op:expr,
        $expr_kind:expr,
        $expected:expr
    ) => {
        #[test]
        fn $name() {
            let context = $crate::evaluator::context::Context::new(
                ::testa_core::analyser::symbol_table::SymbolTable::new(),
            );

            let evaluator = $crate::evaluator::Evaluator::new(context);

            let expression = ::testa_core::ast::Expression::new(
                ::testa_core::ast::ExpressionKind::Prefix {
                    operator: $op,
                    expression: Box::new(::testa_core::ast::Expression {
                        kind: $expr_kind,
                        span: ::testa_core::utils::Span::default(),
                    }),
                },
                ::testa_core::utils::Span::default(),
            );

            let result = evaluator
                .evaluate_expression(&expression)
                .expect("Evaluation failed");

            assert_eq!(result, $expected);
        }
    };
}

macro_rules! prefix_tests {
    (
        $(
            $name:ident:
                $op:expr,
                $expr_kind:expr
                => $expected:expr
        ),* $(,)?
    ) => {
        $(
            prefix_test!(
                $name,
                $op,
                $expr_kind,
                $expected
            );
        )*
    };
}

macro_rules! prefix_error_test {
    (
        $name:ident,
        $op:expr,
        $expr_kind:expr
    ) => {
        #[test]
        fn $name() {
            let context = $crate::evaluator::context::Context::new(
                ::testa_core::analyser::symbol_table::SymbolTable::new(),
            );

            let evaluator = $crate::evaluator::Evaluator::new(context);

            let expression = ::testa_core::ast::Expression::new(
                ::testa_core::ast::ExpressionKind::Prefix {
                    operator: $op,
                    expression: Box::new(::testa_core::ast::Expression {
                        kind: $expr_kind,
                        span: ::testa_core::utils::Span::default(),
                    }),
                },
                ::testa_core::utils::Span::default(),
            );

            assert!(evaluator.evaluate_expression(&expression).is_err());
        }
    };
}

macro_rules! infix_error_test {
    (
        $name:ident,
        $left_kind:expr,
        $op:expr,
        $right_kind:expr
    ) => {
        #[test]
        fn $name() {
            let context = $crate::evaluator::context::Context::new(
                ::testa_core::analyser::symbol_table::SymbolTable::new(),
            );

            let evaluator = $crate::evaluator::Evaluator::new(context);

            let expression = ::testa_core::ast::Expression::new(
                ::testa_core::ast::ExpressionKind::Infix {
                    left: Box::new(::testa_core::ast::Expression {
                        kind: $left_kind,
                        span: ::testa_core::utils::Span::default(),
                    }),
                    operator: $op,
                    right: Box::new(::testa_core::ast::Expression {
                        kind: $right_kind,
                        span: ::testa_core::utils::Span::default(),
                    }),
                },
                ::testa_core::utils::Span::default(),
            );

            assert!(evaluator.evaluate_expression(&expression).is_err());
        }
    };
}

macro_rules! infix_error_tests {
    (
        $(
            $name:ident:
                $left_kind:expr,
                $op:expr,
                $right_kind:expr
        ),* $(,)?
    ) => {
        $(
            infix_error_test!(
                $name,
                $left_kind,
                $op,
                $right_kind
            );
        )*
    };
}

macro_rules! prefix_error_tests {
    (
        $(
            $name:ident:
                $op:expr,
                $expr_kind:expr
        ),* $(,)?
    ) => {
        $(
            prefix_error_test!(
                $name,
                $op,
                $expr_kind
            );
        )*
    };
}
