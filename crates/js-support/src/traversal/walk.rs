use crate::parser::{Expr, Literal, Node, Stat};
use crate::traversal::visitor::{ExprVisitor, LiteralVisitor, StatVisitor};
use std::cell::RefCell;

#[derive(Default)]
pub(crate) struct Walker {
    literal_visitor: Vec<RefCell<Box<dyn LiteralVisitor>>>,
    expr_visitor: Vec<RefCell<Box<dyn ExprVisitor>>>,
    stat_visitor: Vec<RefCell<Box<dyn StatVisitor>>>,
}

impl Walker {
    pub(crate) fn new() -> Self {
        Walker::default()
    }
    pub(crate) fn walk(&self, n: &Node) {
        self.visit(n)
    }
    pub(crate) fn register_literal_visitor<V>(&mut self, lv: V)
    where
        V: LiteralVisitor + 'static,
    {
        self.literal_visitor.push(RefCell::new(Box::new(lv)));
    }
    pub(crate) fn register_expr_visitor<V>(&mut self, ev: V)
    where
        V: ExprVisitor + 'static,
    {
        self.expr_visitor.push(RefCell::new(Box::new(ev)));
    }
    pub(crate) fn register_stat_visitor<V>(&mut self, sv: V)
    where
        V: StatVisitor + 'static,
    {
        self.stat_visitor.push(RefCell::new(Box::new(sv)));
    }

    fn visit(&self, n: &Node) {
        match n {
            Node::Root { statements, .. } => {
                statements.iter().for_each(|node| self.visit(node))
            }

            Node::Literal(l) => self.visit_literal(l),
            Node::Expr(e) => self.visit_expr(e),
            Node::Stat(s) => self.visit_stat(s),

            Node::Empty => (),
        }
    }
    fn visit_literal(&self, n: &Literal) {
        match n {
            Literal::Id { name, .. } => self
                .literal_visitor
                .iter()
                .for_each(|v| v.borrow_mut().visit_id(name)),

            Literal::StringLiteral { value, .. } => {}
            Literal::NumberLiteral { value, .. } => {}
            Literal::ObjectLiteral { attributes, .. } => {}
            Literal::ArrayLiteral { items, .. } => {}
        }
    }
    fn visit_expr(&self, n: &Expr) {
        match n {
            Expr::UnaryExpr {
                prefix, op, expr, ..
            } => {}
            Expr::BinaryExpr {
                left, op, right, ..
            } => {}
            Expr::TernaryExpr {
                condition,
                then_expr,
                else_expr,
                ..
            } => {}
            Expr::AssignmentExpr { left, right, .. } => {}
            Expr::ValueAccessExpr { path, .. } => {}
            Expr::FunctionCallExpr { callee, args, .. } => {}
            Expr::ReturnExpr { expr, .. } => {}
        }
    }
    fn visit_stat(&self, n: &Stat) {
        match n {
            Stat::VariableDeclaStatement {
                definator,
                name,
                init,
                ..
            } => {}
            Stat::FunctionDeclaStatement {
                name, args, body, ..
            } => {}
            Stat::IfStatement {
                expr,
                then_block,
                else_block,
                ..
            } => {}
            Stat::SwitchStatement {
                expr, then_block, ..
            } => {}
            Stat::CaseStatement {
                expr,
                has_break,
                then_block,
                ..
            } => {}
            Stat::DefaultStatement {
                has_break,
                then_block,
                ..
            } => {}
            Stat::ForStatement {
                init,
                condition,
                step,
                then_block,
                ..
            } => {}
            Stat::WhileStatement {
                condition,
                then_block,
                ..
            } => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{lex, syntax};
    use crate::traversal::visitor::LiteralVisitor;
    use crate::traversal::walk::Walker;

    struct V1 {}
    impl V1 {
        fn new() -> Self {
            V1 {}
        }
    }

    impl LiteralVisitor for V1 {
        fn visit_id(&mut self, name: &str) {
            println!("{name} {name}")
        }
    }

    struct V {}
    impl V {
        fn new() -> Self {
            V {}
        }
    }

    impl LiteralVisitor for V {
        fn visit_id(&mut self, name: &str) {
            println!("{name}")
        }
    }

    #[test]
    fn smoke() {
        let mut w = Walker::new();
        let v = V::new();
        let v1 = V1::new();
        w.register_literal_visitor(v);
        w.register_literal_visitor(v1);
        let ast = syntax(lex("a")).unwrap();
        println!("{:?}", ast);
        w.walk(&ast);
    }
}
