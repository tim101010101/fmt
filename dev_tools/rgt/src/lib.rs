use std::fmt::{Display, Formatter};

mod green;
mod red;
pub mod syntax_kind;

pub use crate::{
    green::{GreenNode, GreenNodeData, GreenToken, GreenTokenData},
    red::{RedNode, RedNodeData, RedToken, RedTokenData},
};

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct SyntaxKind(u16);

#[derive(Debug, Clone, Copy)]
pub enum NodeOrToken<N, T> {
    Node(N),
    Token(T),
}

impl NodeOrToken<GreenNode, GreenToken> {
    fn text_len(&self) -> usize {
        match self {
            NodeOrToken::Node(n) => n.text_len(),
            NodeOrToken::Token(t) => t.text_len(),
        }
    }
}

impl Display for NodeOrToken<GreenNode, GreenToken> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeOrToken::Node(n) => Display::fmt(n, f),
            NodeOrToken::Token(t) => Display::fmt(t, f),
        }
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

#[cfg(test)]
mod tests {
    use crate::syntax_kind::{BIN_EXR, INT, PLUS, STAR, WHITESPACE};
    use crate::{GreenNodeData, GreenTokenData};
    use std::sync::Arc;

    #[test]
    fn smoke() {
        // 1 + 2
        let ws = Arc::new(GreenTokenData::new(WHITESPACE, " ".to_string()));
        let one = Arc::new(GreenTokenData::new(INT, "1".to_string()));
        let plus = Arc::new(GreenTokenData::new(PLUS, "+".to_string()));
        let two = Arc::new(GreenTokenData::new(INT, "2".to_string()));

        let additional = Arc::new(GreenNodeData::new(
            BIN_EXR,
            vec![
                one.into(),
                ws.clone().into(),
                plus.clone().into(),
                ws.clone().into(),
                two.into(),
            ],
        ));
        eprintln!("addition = {}", additional);

        // (1 + 2) * (1 + 2)
        let star = Arc::new(GreenTokenData::new(STAR, "*".to_string()));

        let multiplication = Arc::new(GreenNodeData::new(
            BIN_EXR,
            vec![
                additional.clone().into(),
                ws.clone().into(),
                star.into(),
                ws.clone().into(),
                additional.into(),
            ],
        ));
        eprintln!("multiplication = {}", multiplication);

        let addition = Arc::new(GreenNodeData::new(
            BIN_EXR,
            vec![
                multiplication.clone().into(),
                ws.clone().into(),
                plus.clone().into(),
                ws.clone().into(),
                multiplication.into(),
            ],
        ));
        eprintln!("addition = {}", addition);

        let mul2 = addition.children().nth(4).unwrap().into_node().unwrap();
        let one2 = mul2.children().next().unwrap().into_token().unwrap();
        eprintln!("mul2 = {}", mul2);
        eprintln!("one2 = {}", one2);
    }
}
