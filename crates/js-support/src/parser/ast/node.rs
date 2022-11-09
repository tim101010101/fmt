use crate::syntax_kind::{SyntaxKind, EMPTY, ROOT};

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

impl Node {
    pub(crate) fn kind(&self) -> SyntaxKind {
        match self {
            Node::Empty => EMPTY,
            Node::Root { .. } => ROOT,

            Node::Literal(l) => l.kind(),
            Node::Expr(e) => e.kind(),
            Node::Stat(s) => s.kind(),
        }
    }
}
impl Literal {
    pub(crate) fn kind(&self) -> SyntaxKind {
        match self {
            Literal::Id { kind, .. }
            | Literal::StringLiteral { kind, .. }
            | Literal::NumberLiteral { kind, .. }
            | Literal::ObjectLiteral { kind, .. }
            | Literal::ArrayLiteral { kind, .. } => *kind,
        }
    }
}
impl Expr {
    pub(crate) fn kind(&self) -> SyntaxKind {
        match self {
            Expr::UnaryExpr { kind, .. }
            | Expr::BinaryExpr { kind, .. }
            | Expr::TernaryExpr { kind, .. }
            | Expr::AssignmentExpr { kind, .. }
            | Expr::ValueAccessExpr { kind, .. }
            | Expr::FunctionCallExpr { kind, .. }
            | Expr::ReturnExpr { kind, .. } => *kind,
        }
    }
}
impl Stat {
    pub(crate) fn kind(&self) -> SyntaxKind {
        match self {
            Stat::VariableDeclaStatement { kind, .. }
            | Stat::FunctionDeclaStatement { kind, .. }
            | Stat::IfStatement { kind, .. }
            | Stat::SwitchStatement { kind, .. }
            | Stat::CaseStatement { kind, .. }
            | Stat::DefaultStatement { kind, .. }
            | Stat::ForStatement { kind, .. }
            | Stat::WhileStatement { kind, .. } => *kind,
        }
    }
}

// pub(crate) fn walk<V: Visitor>(visitor: &mut V, n: &Node) {
//     match n {
//         Node::Empty => {}
//         Node::Root { statements, .. } => statements.iter().for_each(|s| walk(visitor, s)),
//         Node::Literal(l) => walk_literal(visitor, l),
//         Node::Expr(e) => walk_expr(visitor, e),
//         Node::Stat(s) => walk_stat(visitor, s),
//     }
// }
// pub(crate) fn walk_literal<V: Visitor>(visitor: &mut V, l: &Literal) {
//     match l {
//         Literal::Id { name, .. } => visitor.visit_id(name),
//         Literal::StringLiteral { value, .. } => visitor.visit_string(value),
//         Literal::NumberLiteral { raw, .. } => visitor.visit_number(raw),
//         Literal::ObjectLiteral { attributes, .. } => visitor.visit_object(attributes),
//         Literal::ArrayLiteral { items, .. } => visitor.visit_array(items),
//     }
// }
// pub(crate) fn walk_expr<V: Visitor>(visitor: &mut V, e: &Expr) {
//     match e {
//         Expr::UnaryExpr {
//             prefix, op, expr, ..
//         } => visitor.visit_unary(*prefix, SyntaxKind::to_str(op), expr),
//         Expr::BinaryExpr {
//             left, op, right, ..
//         } => visitor.visit_binary(left, SyntaxKind::to_str(op), right),
//         Expr::TernaryExpr {
//             condition,
//             then_expr,
//             else_expr,
//             ..
//         } => visitor.visit_ternary(condition, then_expr, else_expr),
//         Expr::AssignmentExpr { left, right, .. } => visitor.visit_assignment(left, right),
//         Expr::ValueAccessExpr { path, .. } => visitor.visit_value_access(path),
//         Expr::FunctionCallExpr { callee, args, .. } => visitor.visit_function_call(callee, args),
//         Expr::ReturnExpr { expr, .. } => visitor.visit_return(expr),
//     }
// }
// pub(crate) fn walk_stat<V: Visitor>(visitor: &mut V, s: &Stat) {
//     match s {
//         Stat::VariableDeclaStatement {
//             definator,
//             name,
//             init,
//             ..
//         } => visitor.visit_var_decla(definator, name, init),
//         Stat::FunctionDeclaStatement {
//             name, args, body, ..
//         } => visitor.visit_fun_decla(name, args, body),
//         Stat::IfStatement {
//             expr,
//             then_block,
//             else_block,
//             ..
//         } => visitor.visit_if(expr, then_block, else_block),
//         Stat::SwitchStatement {
//             expr, then_block, ..
//         } => visitor.visit_switch(expr, then_block),
//         Stat::CaseStatement {
//             expr,
//             has_break,
//             then_block,
//             ..
//         } => visitor.visit_case(expr, *has_break, then_block),
//         Stat::DefaultStatement {
//             has_break,
//             then_block,
//             ..
//         } => visitor.visit_default(*has_break, then_block),
//         Stat::ForStatement {
//             init,
//             condition,
//             step,
//             then_block,
//             ..
//         } => visitor.visit_for(init, condition, step, then_block),
//         Stat::WhileStatement {
//             condition,
//             then_block,
//             ..
//         } => visitor.visit_while(condition, then_block),
//     }
// }
