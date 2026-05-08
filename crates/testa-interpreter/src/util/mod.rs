use rand::Rng;

pub mod extract_object;

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

                if self.current.is_multiple_of(self.report_interval) || self.current == self.total {
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

pub fn generate_random_string(rng: &mut impl Rng, length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz\
                            ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            0123456789";

    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
