use crate::syntax_kind::SyntaxKind;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct BoxedNode(Box<Node>);

impl BoxedNode {
    pub fn new(node: Node) -> Self {
        BoxedNode(Box::new(node))
    }
}

impl From<Node> for BoxedNode {
    fn from(n: Node) -> Self {
        BoxedNode::new(n)
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Node {
    Empty,

    // literal
    StringLiteral {
        kind: SyntaxKind,
        value: String,
        raw: String,
    },
    NumberLiteral {
        kind: SyntaxKind,
        value: i32,
        raw: String,
    },
    ObjectLiteral {
        kind: SyntaxKind,
        attributes: Vec<(String, Box<Node>)>,
    },
    ArrayLiteral {
        kind: SyntaxKind,
        items: Vec<Box<Node>>,
    },

    // expression
    UnaryExpr {
        kind: SyntaxKind,
        prefix: bool,
        op: SyntaxKind,
        expr: Box<Node>,
    },
    BinaryExpr {
        kind: SyntaxKind,
        left: Box<Node>,
        op: SyntaxKind,
        right: Box<Node>,
    },
    TernaryExpr {
        kind: SyntaxKind,
        condition: Box<Node>,
        then_expr: Box<Node>,
        else_expr: Box<Node>,
    },
    AssignmentExpr {
        kind: SyntaxKind,
        left: Box<Node>,
        right: Box<Node>,
    },
    ValueAccessExpr {
        kind: SyntaxKind,
        path: Vec<String>,
    },
    FunctionCallExpr {
        kind: SyntaxKind,
        name: String,
        args: Vec<Box<Node>>,
    },
    VariableDeclaExpr {
        kind: SyntaxKind,
        defintor: String,
        name: String,
        init: Box<Node>,
    },
    FunctionDeclaExpr {
        kind: SyntaxKind,
        name: String,
        args: Vec<String>,
        body: Vec<Box<Node>>,
    },
}
