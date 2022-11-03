use crate::demo::traits::AstNode;
use crate::demo::tree::{BoxedNode, Node};
use std::collections::HashMap;

pub struct NodeStore(pub HashMap<u64, BoxedNode>);

impl NodeStore {
    pub fn new() -> Self {
        NodeStore(HashMap::new())
    }
    pub fn node<N>(&mut self, n: N) -> Node
    where
        N: AstNode + 'static,
    {
        let hash = n.hash();
        let new_node = Node::new(hash);
        self.0.insert(hash, BoxedNode::new(n));

        new_node
    }
}
