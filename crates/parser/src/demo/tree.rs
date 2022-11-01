use crate::demo::traits::AstNode;

#[derive(
    Debug, Default, Hash, Copy, Clone, PartialOrd, PartialEq,
)]
pub struct Node(pub u64);
impl Node {
    pub fn new(hash: u64) -> Self {
        Node(hash)
    }
}
impl AstNode for Node {
    fn hash(&self) -> u64 {
        self.0
    }
}

pub struct BoxedNode {
    pub(crate) node: Box<dyn AstNode>,
}
impl BoxedNode {
    pub fn new<N>(node: N) -> Self
    where
        N: AstNode + 'static,
    {
        BoxedNode {
            node: Box::new(node),
        }
    }
}
impl<'store> AstNode for BoxedNode {
    fn hash(&self) -> u64 {
        self.node.hash()
    }
}
