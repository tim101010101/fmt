use crate::ast::node::Node;
use crate::ast::token::Token;
use std::fmt::{Display, Formatter};

pub type Element = NodeOrToken<Node, Token>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NodeOrToken<N, T> {
    Node(N),
    Token(T),
}

impl Element {
    pub(crate) fn text_len(&self) -> usize {
        match self {
            Element::Node(n) => n.text_len(),
            Element::Token(t) => t.text_len(),
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

impl From<Token> for Element {
    fn from(t: Token) -> Self {
        Self::Token(t)
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
