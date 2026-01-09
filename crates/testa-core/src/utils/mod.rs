#[cfg(test)]
pub mod test_utils;

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
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

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
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

    pub fn outer(&self) -> std::ops::Range<usize> {
        self.start.offset..self.end.offset
    }

    pub fn inner(&self) -> std::ops::Range<usize> {
        self.start.offset + 1..self.end.offset - 1
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

    pub fn to_lsp_range(&self) -> lsp_types::Range {
        lsp_types::Range {
            start: lsp_types::Position {
                line: self.start.line - 1,
                character: self.start.column - 1,
            },
            end: lsp_types::Position {
                line: self.end.line - 1,
                character: self.end.column - 1,
            },
        }
    }

    pub fn contains_position(&self, line: u32, column: u32) -> bool {
        if line < self.start.line || line > self.end.line {
            return false;
        }
        if line == self.start.line && column < self.start.column {
            return false;
        }
        if line == self.end.line && column > self.end.column {
            return false;
        }
        true
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start.line == self.end.line {
            write!(
                f,
                "{}:{}-{}",
                self.start.line, self.start.column, self.end.column
            )
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}
