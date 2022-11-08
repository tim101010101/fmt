use crate::parser::{Expr, Literal, Node, Stat};
use crate::syntax_kind::SyntaxKind;
use crate::traversal::visitor::Visitor;
use std::cell::RefCell;

pub(crate) struct Walker {
    visitor: Vec<RefCell<Box<dyn Visitor>>>,
}

impl Walker {
    pub(crate) fn new() -> Self {
        Walker {
            visitor: Vec::new(),
        }
    }
    pub(crate) fn walk(&self, n: &Node) {
        self.visit(n)
    }
    pub(crate) fn register_visitor<V: Visitor + 'static>(&mut self, v: V) {
        self.visitor.push(RefCell::new(Box::new(v)))
    }

    fn visit(&self, n: &Node) {
        self.dispatch_visitor(n);
    }
}

/// visit
impl Walker {
    fn dispatch_visitor(&self, n: &Node) {
        match n {
            Node::Root { statements, .. } => statements.iter().for_each(|node| self.visit(node)),

            Node::Literal(l) => self.dispatch_literal_visitor(l),
            Node::Expr(e) => self.dispatch_expr_visitor(e),
            Node::Stat(s) => self.dispatch_stat_visitor(s),

            Node::Empty => (),
        }
    }
    fn dispatch_literal_visitor(&self, l: &Literal) {
        match l {
            Literal::Id { kind, name } => self
                .visitor
                .iter()
                .for_each(|v| v.borrow_mut().visit_id(kind, name)),

            Literal::StringLiteral { kind, value, raw } => self
                .visitor
                .iter()
                .for_each(|v| v.borrow_mut().visit_string_literal(kind, value, raw)),

            Literal::NumberLiteral { kind, value, raw } => self
                .visitor
                .iter()
                .for_each(|v| v.borrow_mut().visit_number_literal(kind, *value, raw)),

            Literal::ObjectLiteral { kind, attributes } => self.visitor.iter().for_each(|v| {
                v.borrow_mut().visit_object_literal(kind, attributes);
                attributes.iter().for_each(|(_, n)| self.visit(n))
            }),

            Literal::ArrayLiteral { kind, items } => self.visitor.iter().for_each(|v| {
                v.borrow_mut().visit_array_literal(kind, items);
                items.iter().for_each(|n| self.visit(n));
            }),
        }
    }
    fn dispatch_expr_visitor(&self, e: &Expr) {
        match e {
            Expr::UnaryExpr {
                kind,
                prefix,
                op,
                expr,
            } => self.visitor.iter().for_each(|v| {
                v.borrow_mut()
                    .visit_unary_expr(kind, *prefix, SyntaxKind::to_str(op), expr);
                self.visit(expr);
            }),

            Expr::BinaryExpr {
                kind,
                left,
                op,
                right,
            } => self.visitor.iter().for_each(|v| {
                v.borrow_mut()
                    .visit_binary_expr(kind, left, SyntaxKind::to_str(op), right);
                self.visit(left);
                self.visit(right);
            }),

            Expr::TernaryExpr {
                kind,
                condition,
                then_expr,
                else_expr,
            } => self.visitor.iter().for_each(|v| {
                v.borrow_mut()
                    .visit_ternary_expr(kind, condition, then_expr, else_expr);
                self.visit(condition);
                self.visit(then_expr);
                self.visit(else_expr);
            }),

            Expr::AssignmentExpr { kind, left, right } => self.visitor.iter().for_each(|v| {
                v.borrow_mut().visit_assignment_expr(kind, left, right);
                self.visit(left);
                self.visit(right);
            }),

            Expr::ValueAccessExpr { kind, path } => self.visitor.iter().for_each(|v| {
                v.borrow_mut().visit_value_access_expr(kind, path);
                path.iter().for_each(|n| self.visit(n));
            }),

            Expr::FunctionCallExpr { kind, callee, args } => self.visitor.iter().for_each(|v| {
                v.borrow_mut().visit_function_call_expr(kind, callee, args);
                self.visit(callee);
                args.iter().for_each(|n| self.visit(n));
            }),

            Expr::ReturnExpr { kind, expr } => self.visitor.iter().for_each(|v| {
                v.borrow_mut().visit_return_expr(kind, expr);
                self.visit(expr);
            }),
        }
    }
    fn dispatch_stat_visitor(&self, s: &Stat) {
        match s {
            Stat::VariableDeclaStatement {
                kind,
                definator,
                name,
                init,
            } => self.visitor.iter().for_each(|v| {
                v.borrow_mut()
                    .visit_variable_decla_stat(kind, definator, name, init);
                self.visit(name);
                self.visit(init);
            }),

            Stat::FunctionDeclaStatement {
                kind,
                name,
                args,
                body,
            } => self.visitor.iter().for_each(|v| {
                v.borrow_mut()
                    .visit_function_decla_stat(kind, name, args, body);
                self.visit(name);
                args.iter().for_each(|n| self.visit(n));
                body.iter().for_each(|n| self.visit(n));
            }),

            Stat::IfStatement {
                kind,
                expr,
                then_block,
                else_block,
            } => self.visitor.iter().for_each(|v| {
                v.borrow_mut()
                    .visit_if_stat(kind, expr, then_block, else_block);
                self.visit(expr);
                then_block.iter().for_each(|n| self.visit(n));
                self.visit(else_block);
            }),

            Stat::SwitchStatement {
                kind,
                expr,
                then_block,
            } => self.visitor.iter().for_each(|v| {
                v.borrow_mut().visit_switch_stat(kind, expr, then_block);
                self.visit(expr);
                then_block.iter().for_each(|n| self.visit(n));
            }),

            Stat::CaseStatement {
                kind,
                expr,
                has_break,
                then_block,
            } => self.visitor.iter().for_each(|v| {
                v.borrow_mut()
                    .visit_case_stat(kind, expr, *has_break, then_block);
                self.visit(expr);
                then_block.iter().for_each(|n| self.visit(n));
            }),

            Stat::DefaultStatement {
                kind,
                has_break,
                then_block,
            } => self.visitor.iter().for_each(|v| {
                v.borrow_mut()
                    .visit_default_stat(kind, *has_break, then_block);
                then_block.iter().for_each(|n| self.visit(n));
            }),

            Stat::ForStatement {
                kind,
                init,
                condition,
                step,
                then_block,
            } => self.visitor.iter().for_each(|v| {
                v.borrow_mut()
                    .visit_for_stat(kind, init, condition, step, then_block);
                self.visit(init);
                self.visit(condition);
                self.visit(step);
                then_block.iter().for_each(|n| self.visit(n));
            }),

            Stat::WhileStatement {
                kind,
                condition,
                then_block,
            } => self.visitor.iter().for_each(|v| {
                v.borrow_mut().visit_while_stat(kind, condition, then_block);
                self.visit(condition);
                then_block.iter().for_each(|n| self.visit(n));
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{lex, syntax, Node};
    use crate::syntax_kind::SyntaxKind;
    use crate::traversal::visitor::Visitor;
    use crate::traversal::walk::Walker;

    struct V {}
    impl Visitor for V {
        fn visit_id(&mut self, _: &SyntaxKind, name: &str) {
            println!("id {name}")
        }
        fn visit_number_literal(&mut self, _: &SyntaxKind, value: i32, _: &str) {
            println!("number {value}")
        }
        fn visit_variable_decla_stat(
            &mut self,
            kind: &SyntaxKind,
            definator: &str,
            naem: &Box<Node>,
            init: &Box<Node>,
        ) {
            println!("definator {definator}")
        }
        fn visit_ternary_expr(
            &mut self,
            kind: &SyntaxKind,
            condition: &Box<Node>,
            then_expr: &Box<Node>,
            else_expr: &Box<Node>,
        ) {
            println!("kind {:?}", kind)
        }
    }

    #[test]
    fn smoke() {
        let ast = syntax(lex("const foo = bar == baz ? 1 : 2")).unwrap();
        // let ast = syntax(lex("foo")).unwrap();
        let mut w = Walker::new();
        w.register_visitor(V {});

        w.walk(&ast);
    }
}
