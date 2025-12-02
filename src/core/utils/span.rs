#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Location {
    pub offset: usize,
    pub line: u32,
    pub column: u32,
}

impl Location {
    pub fn new(offset: usize, line: u32, column: u32) -> Self {
        Self {
            offset,
            line,
            column,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

impl Span {
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    
    pub fn from_len(location: Location, len: usize) -> Self {
        Self {
            start: location,
            end: Location {
                offset: location.offset + len,
                line: location.line,
                column: location.column + len as u32,
            },
        }
    }

    pub fn inner(&self) -> std::ops::Range<usize> {
        self.start.offset + 1 .. self.end.offset - 1
    }

    pub fn single_char(location: Location) -> Self {
        Self::from_len(location, 1)
    }

    pub fn len(&self) -> usize {
        self.end.offset - self.start.offset
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains_offset(&self, offset: usize) -> bool {
        offset >= self.start.offset && offset < self.end.offset
    }

    pub fn merge(&self, other: Span) -> Span {
        let start = if self.start.offset < other.start.offset {
            self.start
        } else {
            other.start
        };
        let end = if self.end.offset > other.end.offset {
            self.end
        } else {
            other.end
        };
        Span { start, end }
    }

    pub fn extend_to(&self, end: Location) -> Span {
        Span {
            start: self.start,
            end,
        }
    }
}
