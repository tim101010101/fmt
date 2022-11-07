use crate::parser::ast::node::{Expr, Literal, Node, Stat};

pub(crate) mod visitor {
    use super::*;

    pub(crate) trait Visitor {
        fn visit(&mut self, n: &Node) {
            match n {
                Node::Root { statements, .. } => {
                    self.visit_root(statements)
                }

                Node::Literal(l) => self.visit_literal(l),
                Node::Expr(e) => self.visit_expr(e),
                Node::Stat(s) => self.visit_stat(s),

                Node::Empty => (),
            }
        }
        fn visit_root(
            &mut self,
            statements: &Vec<Box<Node>>,
        ) {
            statements
                .iter()
                .for_each(|node| self.visit(node))
        }

        fn visit_literal(&mut self, n: &Literal);
        fn visit_expr(&mut self, n: &Expr);
        fn visit_stat(&mut self, n: &Stat);
    }
}
