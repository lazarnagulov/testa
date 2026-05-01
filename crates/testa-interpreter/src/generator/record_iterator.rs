use crate::{
    evaluator::error::EvalError,
    generator::{Record, RecordGenerator},
};

impl<'a> Iterator for RecordGenerator<'a> {
    type Item = Result<Record, EvalError>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_statement < self.generate_infos.len() {
            let info = &self.generate_infos[self.current_statement];

            if self.current_count < info.total_count {
                self.current_count += 1;

                let item_ref = info.template_ref.clone();
                let ctx = &self.evaluator.context;
                let state = &mut self.evaluator.state;

                return Some(Self::generate_record(ctx, state, &item_ref));
            }

            self.current_statement += 1;
            self.current_count = 0;
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.generate_infos[self.current_statement..]
            .iter()
            .map(|info| info.total_count)
            .sum::<usize>()
            .saturating_sub(self.current_count);

        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for RecordGenerator<'a> {
    fn len(&self) -> usize {
        self.size_hint().0
    }
}
