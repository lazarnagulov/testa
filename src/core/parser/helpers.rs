use crate::core::utils::span::Span;
use super::Parser;

impl<'src> Parser<'src> {

    pub(super) fn source_text(&self, span: Span) -> &'src str {
        &self.source[span.start.offset..span.end.offset]
    }
    
    pub(super) fn token_text(&self, span: Span) -> &'src str {
        &self.source[span.start.offset..span.end.offset]
    }
    
    pub(super) fn string_literal_content(&self, span: Span) -> &'src str {
        &self.source[span.inner()]
    }

    pub(super) fn make_subspan(&self, parent: Span, start_offset: usize, end_offset: usize) -> Span {
        use crate::core::utils::span::Location;
        
        Span {
            start: Location {
                offset: parent.start.offset + start_offset,
                line: parent.start.line, 
                column: parent.start.column + start_offset as u32,
            },
            end: Location {
                offset: parent.start.offset + end_offset,
                line: parent.start.line,
                column: parent.start.column + end_offset as u32,
            },
        }
    }
}