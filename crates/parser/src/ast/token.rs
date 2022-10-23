use crate::syntax_kind::SyntaxKind;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

pub type Token = Arc<TokenData>;
#[derive(Debug, PartialEq)]
pub struct TokenData {
    kind: SyntaxKind,
    text: String,
}

impl TokenData {
    pub fn new(kind: SyntaxKind, text: String) -> Self {
        TokenData { kind, text }
    }
    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }
    pub fn text(&self) -> &str {
        self.text.as_ref()
    }
    pub fn text_len(&self) -> usize {
        self.text().len()
    }
}

impl Display for TokenData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.text(), f)
    }
}
