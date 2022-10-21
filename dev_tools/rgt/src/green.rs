use crate::{NodeOrToken, SyntaxKind};
use std::fmt::{Display, Formatter};
use std::iter;
use std::sync::Arc;

pub type GreenElement = NodeOrToken<GreenNode, GreenToken>;
pub type GreenToken = Arc<GreenTokenData>;
#[derive(Debug, Clone)]
pub struct GreenTokenData {
    kind: SyntaxKind,
    text: String,
}

impl From<GreenNode> for NodeOrToken<GreenNode, GreenToken> {
    fn from(v: GreenNode) -> Self {
        Self::Node(v)
    }
}

impl From<GreenToken> for NodeOrToken<GreenNode, GreenToken> {
    fn from(v: GreenToken) -> Self {
        Self::Token(v)
    }
}

impl GreenTokenData {
    pub(crate) fn new(kind: SyntaxKind, text: String) -> Self {
        GreenTokenData { kind, text }
    }
    pub(crate) fn kind(&self) -> SyntaxKind {
        self.kind
    }
    fn text(&self) -> &str {
        self.text.as_ref()
    }
    pub(crate) fn text_len(&self) -> usize {
        self.text().len()
    }
}

impl Display for GreenTokenData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.text(), f)
    }
}

pub type GreenNode = Arc<GreenNodeData>;
#[derive(Debug, Clone)]
pub struct GreenNodeData {
    kind: SyntaxKind,
    len: usize,
    children: Vec<GreenElement>,
}

impl GreenNodeData {
    pub(crate) fn new(kind: SyntaxKind, children: Vec<GreenElement>) -> Self {
        let len = children.iter().map(|it| it.text_len()).sum();
        GreenNodeData {
            kind,
            len,
            children,
        }
    }
    pub(crate) fn kind(&self) -> SyntaxKind {
        self.kind
    }
    pub(crate) fn text_len(&self) -> usize {
        self.len
    }
    pub(crate) fn children<'a>(&'a self) -> impl Iterator<Item = GreenElement> + '_ {
        self.children.iter().cloned()
    }
    pub(crate) fn replace_child(&self, idx: usize, new_child: GreenElement) -> GreenNodeData {
        assert!(idx < self.children.len());

        let left_children = self.children().take(idx);
        let right_children = self.children().skip(idx + 1);
        let new_children: Vec<_> = left_children
            .chain(iter::once(new_child))
            .chain(right_children)
            .collect();

        GreenNodeData::new(self.kind, new_children)
    }
}

impl Display for GreenNodeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for child in self.children() {
            Display::fmt(&child, f)?
        }
        Ok(())
    }
}
