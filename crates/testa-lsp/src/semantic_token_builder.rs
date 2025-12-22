use testa_core::{
    ast::{Attribute, Variant, visitor::Visitor},
    utils::Span,
};
use tower_lsp::lsp_types::SemanticToken;

use crate::semantic_token::{MODIFIER_DECLARATION, RawToken, TokenType};

pub struct SemanticTokenBuilder {
    tokens: Vec<RawToken>,
}

impl SemanticTokenBuilder {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn add_token(&mut self, span: Span, token_type: TokenType, modifiers: u32) {
        self.tokens
            .push(RawToken::new(span, token_type as u32, modifiers));
    }

    pub fn build(mut self) -> Vec<SemanticToken> {
        self.tokens.sort_by_key(|t| (t.line, t.start));

        let mut result = Vec::new();
        let mut prev_line = 0;
        let mut prev_start = 0;

        for token in self.tokens {
            let delta_line = token.line - prev_line;
            let delta_start = if delta_line == 0 {
                token.start - prev_start
            } else {
                token.start
            };

            result.push(SemanticToken {
                delta_line,
                delta_start,
                length: token.length,
                token_type: token.token_type,
                token_modifiers_bitset: token.modifiers,
            });

            prev_line = token.line;
            prev_start = token.start;
        }

        result
    }
}

impl Visitor for SemanticTokenBuilder {
    fn visit_enum(
        &mut self,
        _name: &str,
        variants: &[Variant],
        _attributes: &[Attribute],
        span: Span,
    ) {
        self.add_token(span, TokenType::Enum, MODIFIER_DECLARATION);
        for variant in variants {
            self.add_token(variant.span, TokenType::EnumMember, MODIFIER_DECLARATION);
        }
    }
}
