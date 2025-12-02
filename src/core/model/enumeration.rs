use rand::Rng;

use crate::{
    core::ast::Variant,
    interpreter::{
        context::{Context, Visitor},
        eval_error::EvalError,
        evaluator,
        object::Object,
    },
};

#[derive(Debug, Default)]
pub struct EvaluatedVariant {
    pub name: String,
    pub weight: isize,
}

impl EvaluatedVariant {
    pub fn new(name: &str, weight: isize) -> Self {
        EvaluatedVariant {
            name: name.to_owned(),
            weight,
        }
    }
}

#[derive(Debug, Default)]
pub struct Enum {
    pub variants: Vec<EvaluatedVariant>,
    pub cummulative_weights: Vec<EvaluatedVariant>,
    pub total_weight: isize,
}

impl Enum {
    pub fn new(variants: Vec<Variant>, context: &Context) -> Result<Self, EvalError> {
        let evaluated_variants = Enum::evaluate_variants(&variants, context)
            .collect::<Result<Vec<EvaluatedVariant>, EvalError>>()?;
        let total_weight = evaluated_variants.iter().map(|v| v.weight).sum();
        let cummulative_weights =
            Enum::calculate_cummulative_weights(&evaluated_variants).collect::<Vec<_>>();
        let cummulative_weights =
            Enum::zip_cummulative_weights(&cummulative_weights, &evaluated_variants)
                .collect::<Vec<_>>();

        Ok(Enum {
            variants: evaluated_variants,
            cummulative_weights,
            total_weight,
        })
    }

    pub fn insert_variant(&mut self, variant: EvaluatedVariant) {
        self.variants.push(variant)
    }

    pub fn variant(&self, index: usize) -> Option<&EvaluatedVariant> {
        self.variants.get(index)
    }

    pub fn weights(&self) -> impl Iterator<Item = isize> {
        self.variants.iter().map(|variant| variant.weight)
    }

    fn zip_cummulative_weights(
        cummulative_weights: &[isize],
        evaluated_variants: &[EvaluatedVariant],
    ) -> impl Iterator<Item = EvaluatedVariant> {
        evaluated_variants
            .iter()
            .map(|v| v.name.as_str())
            .zip(cummulative_weights)
            .map(|(name, weight)| EvaluatedVariant::new(name, *weight))
    }

    fn calculate_cummulative_weights(
        evaluated_variants: &[EvaluatedVariant],
    ) -> impl Iterator<Item = isize> {
        evaluated_variants.iter().scan(0, |weight, current| {
            *weight += current.weight;
            Some(*weight)
        })
    }

    fn evaluate_variants(
        variants: &[Variant],
        context: &Context,
    ) -> impl Iterator<Item = Result<EvaluatedVariant, EvalError>> {
        variants.iter().map(|variant| match &variant.weight {
            Some(expr) => evaluator::evaluate_expression(expr, context).and_then(|obj| match obj {
                Object::Int(value) => Ok(EvaluatedVariant::new(&variant.name, value)),
                other => Err(EvalError::type_mismatch(
                    "int".to_owned(),
                    format!("{}", other),
                )),
            }),
            None => Ok(EvaluatedVariant::new(&variant.name, 1)),
        })
    }
}

impl Visitor<String> for Enum {
    fn visit(&self, _context: &Context) -> Result<String, EvalError> {
        let mut rng = rand::rng();
        let random_number = rng.random_range(0..self.total_weight as usize);
        Ok(self
            .cummulative_weights
            .iter()
            .find(|variant| variant.weight >= random_number as isize)
            .unwrap()
            .name
            .clone())
    }
}
