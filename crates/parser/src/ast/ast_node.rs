use crate::syntax_kind::{
    SyntaxKind, ASSIGNMENT_EXPR, STRING,
};

pub trait AstNode {}

pub struct BoxedNode {
    node: Box<dyn AstNode>,
}
impl BoxedNode {
    pub(crate) fn new<Node>(node: Node) -> Self
    where
        Node: AstNode + 'static,
    {
        BoxedNode {
            node: Box::new(node),
        }
    }
}

impl AstNode for BoxedNode {}

pub struct Empty {}

impl Empty {
    pub fn new() -> Self {
        Empty {}
    }
}

impl AstNode for Empty {}

pub struct StringLiteral {
    kind: SyntaxKind,
    text: String,
}
impl StringLiteral {
    pub fn new(text: String) -> Self {
        StringLiteral { kind: STRING, text }
    }
}
impl AstNode for StringLiteral {}

pub struct AssignmentExpr {
    kind: SyntaxKind,
    left: BoxedNode,
    right: BoxedNode,
}
impl AssignmentExpr {
    pub fn new(left: BoxedNode, right: BoxedNode) -> Self {
        AssignmentExpr {
            kind: ASSIGNMENT_EXPR,
            left,
            right,
        }
    }
}
impl AstNode for AssignmentExpr {}
