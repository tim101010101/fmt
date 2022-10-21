use crate::{GreenNode, GreenNodeData, GreenToken, NodeOrToken, SyntaxKind};
use std::rc::Rc;
use std::sync::Arc;

pub type RedElement = NodeOrToken<RedNode, RedToken>;

impl From<RedNode> for NodeOrToken<RedNode, RedToken> {
    fn from(v: RedNode) -> Self {
        Self::Node(v)
    }
}

impl From<RedToken> for NodeOrToken<RedNode, RedToken> {
    fn from(v: RedToken) -> Self {
        Self::Token(v)
    }
}

impl RedElement {
    fn kind(&self) -> SyntaxKind {
        match self {
            RedElement::Node(n) => n.kind(),
            RedElement::Token(t) => t.kind(),
        }
    }
    fn text_offset(&self) -> usize {
        match self {
            RedElement::Node(n) => n.text_offset(),
            RedElement::Token(t) => t.text_offset(),
        }
    }
    fn text_len(&self) -> usize {
        match self {
            RedElement::Node(n) => n.text_len(),
            RedElement::Token(t) => t.text_len(),
        }
    }
    fn parent(&self) -> Option<&RedNode> {
        match self {
            RedElement::Node(n) => n.parent(),
            RedElement::Token(t) => t.parent(),
        }
    }
}

pub type RedNode = Rc<RedNodeData>;
#[derive(Clone)]
pub struct RedNodeData {
    parent: Option<RedNode>,
    text_offset: usize,
    green: GreenNode,
}

impl RedNodeData {
    fn new(root: GreenNode) -> RedNode {
        Rc::new(RedNodeData {
            parent: None,
            text_offset: 0,
            green: root,
        })
    }
    fn green(&self) -> &GreenNode {
        &self.green
    }
    fn text_offset(&self) -> usize {
        self.text_offset
    }
    fn parent(&self) -> Option<&RedNode> {
        self.parent.as_ref()
    }
    fn kind(&self) -> SyntaxKind {
        self.green().kind()
    }
    fn text_len(&self) -> usize {
        self.green().text_len()
    }
    fn children<'a>(self: &'a RedNode) -> impl Iterator<Item = RedElement> + 'a {
        let mut offset_in_parent = 0;
        self.green().children().map(move |green_child| {
            let text_offset = self.text_offset() + offset_in_parent;
            offset_in_parent += green_child.text_len();

            match green_child {
                NodeOrToken::Node(n) => Rc::new(RedNodeData {
                    parent: Some(Rc::clone(self)),
                    text_offset,
                    green: n,
                })
                .into(),
                NodeOrToken::Token(t) => Rc::new(RedTokenData {
                    parent: Some(Rc::clone(self)),
                    text_offset,
                    green: t,
                })
                .into(),
            }
        })
    }
}

pub type RedToken = Rc<RedTokenData>;
#[derive(Clone)]
pub struct RedTokenData {
    parent: Option<RedNode>,
    text_offset: usize,
    green: GreenToken,
}

impl RedTokenData {
    fn new(parent: Option<RedNode>, text_offset: usize, green: GreenToken) -> RedToken {
        Rc::new(RedTokenData {
            parent,
            text_offset,
            green,
        })
    }
    fn green(&self) -> &GreenToken {
        &self.green
    }
    fn text_offset(&self) -> usize {
        self.text_offset
    }
    fn parent(&self) -> Option<&RedNode> {
        self.parent.as_ref()
    }
    fn kind(&self) -> SyntaxKind {
        self.green().kind()
    }
    fn text_len(&self) -> usize {
        self.green().text_len()
    }
}
