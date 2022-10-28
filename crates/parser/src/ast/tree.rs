use crate::ast::node::{Node, NodeData};
use crate::ast::token::{Token, TokenData};
use crate::syntax_kind::SyntaxKind;
use std::fmt::{Display, Formatter};

pub type Element = NodeOrToken<Node, Token>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NodeOrToken<N, T> {
    Node(N),
    Token(T),
}

impl Element {
    pub(crate) fn kind(&self) -> SyntaxKind {
        match self {
            Element::Node(n) => n.kind(),
            Element::Token(t) => t.kind(),
        }
    }
    pub(crate) fn text(&self) -> Option<&str> {
        match self {
            Element::Node(_) => None,
            Element::Token(t) => Some(t.text()),
        }
    }
    pub(crate) fn text_len(&self) -> usize {
        match self {
            Element::Node(n) => n.text_len(),
            Element::Token(t) => t.text_len(),
        }
    }
    pub(crate) fn children(&self) -> Option<&[Element]> {
        match self {
            Element::Node(n) => {
                if n.children().len() > 0 {
                    Some(n.children())
                } else {
                    None
                }
            }
            Element::Token(_) => None,
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Node(n) => Display::fmt(n, f),
            Element::Token(t) => Display::fmt(t, f),
        }
    }
}

impl From<Node> for Element {
    fn from(n: Node) -> Self {
        Self::Node(n)
    }
}

impl From<NodeData> for Element {
    fn from(n: NodeData) -> Self {
        Self::Node(Node::new(n))
    }
}

impl From<Token> for Element {
    fn from(t: Token) -> Self {
        Self::Token(t)
    }
}

impl From<TokenData> for Element {
    fn from(t: TokenData) -> Self {
        Self::Token(Token::new(t))
    }
}

impl<N, T> NodeOrToken<N, T> {
    fn into_node(self) -> Option<N> {
        match self {
            NodeOrToken::Node(n) => Some(n),
            NodeOrToken::Token(_) => None,
        }
    }
    fn into_token(self) -> Option<T> {
        match self {
            NodeOrToken::Node(_) => None,
            NodeOrToken::Token(t) => Some(t),
        }
    }
}
