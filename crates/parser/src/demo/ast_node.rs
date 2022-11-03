use crate::demo::traits::AstNode;
use crate::demo::tree::Node;
use crate::syntax_kind::{
    SyntaxKind, ASSIGNMENT_EXPR, STRING,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
pub struct StringLiteral {
    kind: SyntaxKind,
    text: String,
}
impl StringLiteral {
    pub fn new(text: String) -> Self {
        StringLiteral { kind: STRING, text }
    }
}

impl AstNode for StringLiteral {
    fn hash(&self) -> u64 {
        let mut h = DefaultHasher::default();
        self.kind.hash(&mut h);
        self.text.hash(&mut h);
        h.finish()
    }
}

pub struct AssignmentExpr {
    kind: SyntaxKind,
    left: Node,
    right: Node,
}

impl AssignmentExpr {
    pub fn new(left: Node, right: Node) -> Self {
        AssignmentExpr {
            kind: ASSIGNMENT_EXPR,
            left,
            right,
        }
    }
}

impl AstNode for AssignmentExpr {
    fn hash(&self) -> u64 {
        let mut h = DefaultHasher::default();
        self.kind.hash(&mut h);

        // TODO Hash conflict
        h.finish() + self.left.0 + self.right.0
    }
}
