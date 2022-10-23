use crate::ast::tree::Element;
use crate::syntax_kind::SyntaxKind;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[allow(dead_code)]

pub type Node = Arc<NodeData>;
#[derive(Debug, PartialEq)]
pub struct NodeData {
    kind: SyntaxKind,
    len: usize,
    children: Vec<Element>,
}

impl NodeData {
    pub fn new(kind: SyntaxKind, children: Vec<Element>) -> Self {
        let len = children.iter().map(|item| item.text_len()).sum();
        NodeData {
            kind,
            len,
            children,
        }
    }
    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }
    pub fn text_len(&self) -> usize {
        self.len
    }
    pub fn children(&self) -> &[Element] {
        self.children.as_slice()
    }
}

impl Display for NodeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for child in self.children() {
            Display::fmt(&child, f)?
        }
        Ok(())
    }
}
