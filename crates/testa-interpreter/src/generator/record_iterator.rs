use crate::{
    evaluator::{Record, error::EvalError},
    generator::RecordGenerator,
};

impl<'a> Iterator for RecordGenerator<'a> {
    type Item = Result<Record, EvalError>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_statement < self.generate_infos.len() {
            let info = &self.generate_infos[self.current_statement];
            let ctx = &self.evaluator.context;
            let state = &mut self.evaluator.state;

            if self.current_count < info.total_count {
                self.current_count += 1;

                let result = if let Some(name) = &info.template_name {
                    Self::generate_from_template(ctx, state, name, info.span)
                } else {
                    Self::generate_anonymous(ctx, state, &info.body)
                };

                return Some(result);
            }

            self.current_statement += 1;
            self.current_count = 0;
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining: usize = self.generate_infos[self.current_statement..]
            .iter()
            .map(|info| info.total_count)
            .sum::<usize>()
            - self.current_count;

        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for RecordGenerator<'a> {
    fn len(&self) -> usize {
        self.size_hint().0
    }
}
