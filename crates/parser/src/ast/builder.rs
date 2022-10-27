use crate::ast::node::{Node, NodeData};
use crate::ast::token::{Token, TokenData};
use crate::ast::tree::Element;
use crate::syntax_kind::SyntaxKind;

#[derive(Debug)]
struct Cache {
    cache: Vec<(SyntaxKind, Vec<Element>)>,
}

impl Cache {
    fn new() -> Self {
        Cache { cache: Vec::new() }
    }
    fn len(&self) -> usize {
        self.cache.len()
    }
    fn cache(&mut self, ele: Element) -> Result<(), ()> {
        if let Some((k, mut children)) = self.cache.pop() {
            children.push(ele);
            self.cache.push((k, children));
            Ok(())
        } else {
            Err(())
        }
    }
    fn push_ctx(&mut self, kind: SyntaxKind) {
        self.cache.push((kind, Vec::new()))
    }
    fn merge_ctx(&mut self) {
        if self.len() < 2 {
            return;
        }

        let (kind, children) = self.cache.pop().unwrap();
        let parent = self.cache.pop().unwrap();
        let new_node = Node::new(NodeData::new(kind, children));
        let (kind, mut children) = parent;
        children.push(new_node.into());
        self.cache.push((kind, children));
    }
}

#[derive(Debug)]
pub struct Builder {
    cache: Cache,
}

impl Builder {
    pub(crate) fn new() -> Self {
        Builder {
            cache: Cache::new(),
        }
    }
    pub(crate) fn token(&mut self, kind: SyntaxKind, text: &str) {
        self.cache
            .cache(Token::new(TokenData::new(kind, text.to_string())).into())
            .expect(format!("failed to parse {}", text).as_str())
    }
    fn node(&mut self, kind: SyntaxKind, children: Vec<Element>) {
        self.cache
            .cache(Node::new(NodeData::new(kind, children)).into())
            .expect(format!("failed to parse {:?}", kind).as_str())
    }
    pub(crate) fn start_node(&mut self, kind: SyntaxKind) {
        self.cache.push_ctx(kind);
    }
    pub(crate) fn finish_node(&mut self) {
        self.cache.merge_ctx();
    }
    pub(crate) fn syntax(&mut self) -> Option<Node> {
        match self.cache.len() {
            1 => {
                let (kind, children) = self.cache.cache.pop().unwrap();
                Some(Node::new(NodeData::new(kind, children)))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::builder::Builder;
    use crate::syntax_kind::*;

    #[test]
    fn test_build() {
        let mut b = Builder::new();
        // const foo = 1 + 2;
        b.start_node(BLOCK);
        b.token(DEFINATOR, "const");
        b.token(WHITESPACE, " ");
        b.token(ID, "foo");
        b.token(WHITESPACE, " ");
        b.token(EQ, "=");
        b.token(WHITESPACE, " ");

        b.start_node(BINARY_EXPR);
        b.token(NUMBER, "1");
        b.token(WHITESPACE, " ");
        b.token(PLUS, "+");
        b.token(WHITESPACE, " ");
        b.token(NUMBER, "2");
        b.token(SEMI, ";");
        b.finish_node();

        b.finish_node();

        let ast = b.syntax().unwrap();
        println!("{:?}", ast);
        println!("{}", ast);
    }
}
