use crate::syntax_kind::SyntaxKind;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Element {
    Token(TokenData),
    Node(NodeData),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TokenData {
    kind: SyntaxKind,
    text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NodeData {
    kind: SyntaxKind,
    children: Vec<Element>,
}

impl TokenData {
    pub(crate) fn new(kind: SyntaxKind, text: String) -> Self {
        TokenData { kind, text }
    }
    pub(crate) fn kind(&self) -> SyntaxKind {
        self.kind
    }
    pub(crate) fn text(&self) -> &str {
        self.text.as_str()
    }
}

impl NodeData {
    pub(crate) fn new(kind: SyntaxKind, children: Vec<Element>) -> Self {
        NodeData { kind, children }
    }
    pub(crate) fn kind(&self) -> SyntaxKind {
        self.kind
    }
    pub(crate) fn text(&self) -> String {
        let mut result = String::new();
        self.children()
            .iter()
            .for_each(|el| result.push_str(el.text().as_str()));
        result
    }
    pub(crate) fn children(&self) -> &[Element] {
        self.children.as_slice()
    }
}

impl Element {
    pub(crate) fn kind(&self) -> SyntaxKind {
        match self {
            Element::Token(t) => t.kind(),
            Element::Node(n) => n.kind(),
        }
    }
    pub(crate) fn text(&self) -> String {
        match self {
            Element::Token(t) => t.text().to_string(),
            Element::Node(n) => n.text(),
        }
    }
}
