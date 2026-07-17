use testa_hir::{
    StringPool,
    module::{Module, node::Directive},
};

use crate::evaluator::{
    Evaluator,
    context::{Context, OutputFormat},
};

#[test]
fn test_output_directive() {
    let mut pool = StringPool::new();
    let format_id = pool.intern("csv");

    let mut module = Module::empty("test");
    module.string_pool = pool;
    module.directives = vec![Directive::Output {
        format: format_id,
        options: vec![],
    }];

    let context = Context::new(module);
    let mut evaluator = Evaluator::new(context, None);

    evaluator
        .evaluate_directives()
        .expect("Failed to evaluate output directive");
    assert_eq!(evaluator.context.output_format, OutputFormat::Csv);
}

#[test]
fn test_invalid_output_directive() {
    let mut pool = StringPool::new();
    let format_id = pool.intern("invalid");

    let mut module = Module::empty("test");
    module.string_pool = pool;
    module.directives = vec![Directive::Output {
        format: format_id,
        options: vec![],
    }];

    let context = Context::new(module);
    let mut evaluator = Evaluator::new(context, None);

    assert!(evaluator.evaluate_directives().is_err());
}
