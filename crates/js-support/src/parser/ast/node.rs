use crate::syntax_kind::SyntaxKind;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Node {
    Empty,

    Root {
        kind: SyntaxKind,
        statements: Vec<Box<Node>>,
    },

    Literal(Literal),
    Expr(Expr),
    Stat(Stat),
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Literal {
    Id {
        kind: SyntaxKind,
        name: String,
    },
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
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Expr {
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
        path: Vec<Box<Node>>,
    },
    FunctionCallExpr {
        kind: SyntaxKind,
        callee: Box<Node>,
        args: Vec<Box<Node>>,
    },
    ReturnExpr {
        kind: SyntaxKind,
        expr: Box<Node>,
    },
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Stat {
    VariableDeclaStatement {
        kind: SyntaxKind,
        definator: String,
        name: Box<Node>,
        init: Box<Node>,
    },
    FunctionDeclaStatement {
        kind: SyntaxKind,
        name: Box<Node>,
        args: Vec<Box<Node>>,
        body: Vec<Box<Node>>,
    },
    IfStatement {
        kind: SyntaxKind,
        expr: Box<Node>,
        then_block: Vec<Box<Node>>,
        else_block: Box<Node>, // IfStat | Block
    },
    SwitchStatement {
        kind: SyntaxKind,
        expr: Box<Node>,
        then_block: Vec<Box<Node>>,
    },
    CaseStatement {
        kind: SyntaxKind,
        expr: Box<Node>,
        has_break: bool,
        then_block: Vec<Box<Node>>,
    },
    DefaultStatement {
        kind: SyntaxKind,
        has_break: bool,
        then_block: Vec<Box<Node>>,
    },
    ForStatement {
        kind: SyntaxKind,
        init: Box<Node>,
        condition: Box<Node>,
        step: Box<Node>,
        then_block: Vec<Box<Node>>,
    },
    WhileStatement {
        kind: SyntaxKind,
        condition: Box<Node>,
        then_block: Vec<Box<Node>>,
    },
}

pub(crate) fn literal_node(literal: Literal) -> Node {
    Node::Literal(literal)
}
pub(crate) fn expr_node(expr: Expr) -> Node {
    Node::Expr(expr)
}
pub(crate) fn stat_node(stat: Stat) -> Node {
    Node::Stat(stat)
}
