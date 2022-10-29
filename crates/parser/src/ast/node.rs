use crate::ast::token::{Token, TokenData};
use crate::ast::tree::Element;
use crate::lex::LexedToken;
use crate::syntax_kind::SyntaxKind;
use std::fmt::{Debug, Display, Formatter};
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
    pub fn new(
        kind: SyntaxKind,
        children: Vec<Element>,
    ) -> Self {
        let len = children
            .iter()
            .map(|item| item.text_len())
            .sum();
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
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        for child in self.children() {
            Display::fmt(&child, f)?
        }
        Ok(())
    }
}

pub fn node(
    kind: SyntaxKind,
    children: Vec<Element>,
) -> Node {
    Node::new(NodeData::new(kind, children))
}

// DEBUG
pub fn dump_all(ele: Element) -> String {
    struct Output {
        res: String,
        ident: usize,
    }
    impl Output {
        fn new() -> Self {
            Output {
                res: String::new(),
                ident: 0,
            }
        }
        fn join(&mut self, s: &str) {
            self.res
                .push_str("  ".repeat(self.ident).as_str());
            self.res.push_str(s);
        }
        fn walk_node(&mut self, n: Element) {
            self.join(&format!("{}\n", n.kind().to_str()));
            self.ident += 1;
            self.join(&format!("len: {}\n", n.text_len()));

            if let Some(text) = n.text() {
                self.join(&format!(
                    "text: \"{}\",\n",
                    text
                ));
            }

            if let Some(children) = n.children() {
                self.join("children: \n");
                self.ident += 1;
                children.iter().for_each(|child| {
                    self.walk_node(child.to_owned())
                });
                self.ident -= 1;
            }

            self.ident -= 1;
        }
    }

    let mut o = Output::new();
    o.walk_node(ele);
    o.res
}
