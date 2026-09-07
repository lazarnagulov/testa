use crate::{
    evaluator::{Evaluator, context::Context, expression::evaluate_expression},
    object::Object,
};
use testa_hir::{
    StringPool,
    module::{
        Module,
        node::{Enum, Expr, GlobalItemId, Item, LocalItemId, Variant},
    },
};

const SEED: Option<u64> = Some(42);

#[test]
fn test_evaluate_enum() {
    let mut pool = StringPool::new();
    let enum_name = pool.intern("test");
    let variant_name = pool.intern("variant1");

    let mut module = Module::empty("test");
    module.string_pool = pool;
    module.items = vec![Item::Enum(Enum {
        id: LocalItemId(0),
        name: enum_name,
        variants: vec![Variant {
            name: variant_name,
            weight: None,
        }],
        attributes: vec![],
    })];

    let context = Context::new(module);
    let mut evaluator = Evaluator::new(context, SEED);

    let item_ref = GlobalItemId {
        module: evaluator.context.module.metadata.id,
        item: LocalItemId(0),
    };
    let expr = Expr::Identifier(item_ref);
    let result = evaluate_expression(&evaluator.context, &mut evaluator.state, &expr)
        .expect("Evaluation failed");

    assert_eq!(result, Object::String("variant1".to_string()));
}

#[test]
fn test_evaluate_type_alias() {
    use testa_hir::module::node::{Type, TypeAlias};

    let mut pool = StringPool::new();
    let alias_name = pool.intern("test");

    let mut module = Module::empty("test");
    module.string_pool = pool;
    module.items = vec![Item::TypeAlias(TypeAlias {
        id: LocalItemId(0),
        name: alias_name,
        target_type: Type::Int,
        constraints: vec![],
        attributes: vec![],
        expr: None,
    })];

    let context = Context::new(module);
    let mut evaluator = Evaluator::new(context, SEED);

    let item_ref = GlobalItemId {
        module: evaluator.context.module.metadata.id,
        item: LocalItemId(0),
    };
    let expr = Expr::Identifier(item_ref);
    let result = evaluate_expression(&evaluator.context, &mut evaluator.state, &expr)
        .expect("Evaluation failed");

    assert!(matches!(result, Object::Int(_)));
}
