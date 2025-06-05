#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Span {
    pub start: usize,
    pub size: usize,
    pub line: usize,
    pub line_offset: usize,
}

impl Span {
    pub fn new(start: usize, size: usize, line: usize, line_offset: usize) -> Self {
        Span {
            start,
            size,
            line,
            line_offset,
        }
    }
}
