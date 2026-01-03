pub struct ProgressReporter<I> {
    inner: I,
    total: usize,
    current: usize,
    report_interval: usize,
}

impl<I: Iterator> ProgressReporter<I> {
    pub fn new(inner: I, total: usize, report_interval: usize) -> Self {
        Self {
            inner,
            total,
            current: 0,
            report_interval,
        }
    }
}

impl<I: Iterator> Iterator for ProgressReporter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(item) => {
                self.current += 1;

                if self.current % self.report_interval == 0 || self.current == self.total {
                    let percent = (self.current as f64 / self.total as f64) * 100.0;
                    println!(
                        "Progress: {}/{} ({:.1}%)",
                        self.current, self.total, percent
                    );
                }

                Some(item)
            }
            None => None,
        }
    }
}
