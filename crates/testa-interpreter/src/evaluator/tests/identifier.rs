use crate::{
    evaluator::{Evaluator, context::Context, expression::evaluate_expression},
    object::Object,
};
use testa_hir::{
    Item, ItemId, Module, StringPool,
    module::{Enum, Expr, ItemRef, Variant},
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
        id: ItemId(0),
        name: enum_name,
        variants: vec![Variant {
            name: variant_name,
            weight: None,
        }],
        attributes: vec![],
    })];

    let context = Context::new(module);
    let mut evaluator = Evaluator::new(context, SEED);

    let expr = Expr::Identifier(ItemRef::Local(ItemId(0)));
    let result = evaluate_expression(&evaluator.context, &mut evaluator.state, &expr)
        .expect("Evaluation failed");

    assert_eq!(result, Object::String("variant1".to_string()));
}

#[test]
fn test_evaluate_type_alias() {
    use testa_hir::module::{Type, TypeAlias};

    let mut pool = StringPool::new();
    let alias_name = pool.intern("test");

    let mut module = Module::empty("test");
    module.string_pool = pool;
    module.items = vec![Item::TypeAlias(TypeAlias {
        id: ItemId(0),
        name: alias_name,
        target_type: Type::Int,
        constraints: vec![],
        attributes: vec![],
    })];

    let context = Context::new(module);
    let mut evaluator = Evaluator::new(context, SEED);

    let expr = Expr::Identifier(ItemRef::Local(ItemId(0)));
    let result = evaluate_expression(&evaluator.context, &mut evaluator.state, &expr)
        .expect("Evaluation failed");

    assert!(matches!(result, Object::Int(_)));
}
