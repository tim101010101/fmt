use crate::parser::{Expr, Literal, Node, Stat};
use crate::syntax_kind::SyntaxKind;

pub(crate) trait Visitor {
    fn before_visit_root(&mut self, _: &Node) {}
    fn visit(&mut self, n: &Node) {
        match n {
            Node::Empty => {}
            Node::Literal(l) => self.visit_literal(l),
            Node::Expr(e) => self.visit_expr(e),
            Node::Stat(s) => self.visit_stat(s),
            Node::Root { statements, .. } => statements.iter().for_each(|s| {
                self.before_visit_root(s);
                self.visit(s);
                self.after_visit_root(s);
            }),
        }
    }
    fn after_visit_root(&mut self, _: &Node) {}

    fn before_visit_literal(&mut self, _: &Literal) {}
    fn visit_literal(&mut self, l: &Literal) {
        self.before_visit_literal(l);
        match l {
            Literal::Id { name, .. } => self.visit_id(name),
            Literal::StringLiteral { value, .. } => self.visit_string(value),
            Literal::NumberLiteral { raw, .. } => self.visit_number(raw),
            Literal::ObjectLiteral { attributes, .. } => self.visit_object(attributes),
            Literal::ArrayLiteral { items, .. } => self.visit_array(items),
        }
        self.after_visit_literal(l);
    }
    fn after_visit_literal(&mut self, _: &Literal) {}

    fn before_visit_expr(&mut self, _: &Expr) {}
    fn visit_expr(&mut self, e: &Expr) {
        self.before_visit_expr(e);
        match e {
            Expr::UnaryExpr {
                prefix, op, expr, ..
            } => self.visit_unary(*prefix, SyntaxKind::to_str(op), expr),
            Expr::BinaryExpr {
                left, op, right, ..
            } => self.visit_binary(left, SyntaxKind::to_str(op), right),
            Expr::TernaryExpr {
                condition,
                then_expr,
                else_expr,
                ..
            } => self.visit_ternary(condition, then_expr, else_expr),
            Expr::AssignmentExpr { left, right, .. } => self.visit_assignment(left, right),
            Expr::ValueAccessExpr { path, .. } => self.visit_value_access(path),
            Expr::FunctionCallExpr { callee, args, .. } => self.visit_function_call(callee, args),
        }
        self.after_visit_expr(e);
    }
    fn after_visit_expr(&mut self, _: &Expr) {}

    fn before_visit_stat(&mut self, _: &Stat) {}
    fn visit_stat(&mut self, s: &Stat) {
        self.before_visit_stat(s);
        match s {
            Stat::VariableDeclaStatement {
                definator,
                name,
                init,
                ..
            } => self.visit_var_decla(definator, name, init),
            Stat::FunctionDeclaStatement {
                name, args, body, ..
            } => self.visit_fun_decla(name, args, body),
            Stat::IfStatement {
                expr,
                then_block,
                else_node,
                ..
            } => self.visit_if(expr, then_block, else_node),
            Stat::SwitchStatement {
                expr, then_block, ..
            } => self.visit_switch(expr, then_block),
            Stat::CaseStatement {
                expr, then_block, ..
            } => self.visit_case(expr, then_block),
            Stat::DefaultStatement { then_block, .. } => self.visit_default(then_block),
            Stat::BreakStatement { .. } => self.visit_break(),
            Stat::ForStatement {
                init,
                condition,
                step,
                then_block,
                ..
            } => self.visit_for(init, condition, step, then_block),
            Stat::WhileStatement {
                condition,
                then_block,
                ..
            } => self.visit_while(condition, then_block),
            Stat::ReturnStat { expr, .. } => self.visit_return(expr),
        }
        self.after_visit_stat(s);
    }
    fn after_visit_stat(&mut self, _: &Stat) {}

    fn visit_id(&mut self, _: &str) {}
    fn visit_string(&mut self, _: &str) {}
    fn visit_number(&mut self, _: &str) {}
    fn visit_object(&mut self, _: &[(String, Box<Node>)]) {}
    fn visit_array(&mut self, _: &[Box<Node>]) {}

    fn visit_unary(&mut self, _: bool, _: &str, _: &Box<Node>) {}
    fn visit_binary(&mut self, _: &Box<Node>, _: &str, _: &Box<Node>) {}
    fn visit_ternary(&mut self, _: &Box<Node>, _: &Box<Node>, _: &Box<Node>) {}
    fn visit_assignment(&mut self, _: &Box<Node>, _: &Box<Node>) {}
    fn visit_value_access(&mut self, _: &[Box<Node>]) {}
    fn visit_function_call(&mut self, _: &Box<Node>, _: &[Box<Node>]) {}

    fn visit_var_decla(&mut self, _: &str, _: &Box<Node>, _: &Box<Node>) {}
    fn visit_fun_decla(&mut self, _: &Box<Node>, _: &[Box<Node>], _: &[Box<Node>]) {}
    fn visit_if(&mut self, _: &Box<Node>, _: &[Box<Node>], _: &Box<Node>) {}
    fn visit_else_if(&mut self, _: &Box<Node>, _: &[Box<Node>], _: &Box<Node>) {}
    fn visit_else(&mut self, _: &[Box<Node>]) {}
    fn visit_switch(&mut self, _: &Box<Node>, _: &[Box<Node>]) {}
    fn visit_case(&mut self, _: &Box<Node>, _: &[Box<Node>]) {}
    fn visit_default(&mut self, _: &[Box<Node>]) {}
    fn visit_break(&mut self) {}
    fn visit_for(&mut self, _: &Box<Node>, _: &Box<Node>, _: &Box<Node>, _: &[Box<Node>]) {}
    fn visit_while(&mut self, _: &Box<Node>, _: &[Box<Node>]) {}
    fn visit_return(&mut self, _: &Box<Node>) {}
}

#[cfg(test)]
mod tests {
    use crate::parser::{lex, syntax, Node};
    use crate::traversal::visitor::Visitor;

    #[test]
    fn smoke() {
        struct Logger {
            res: String,
        }
        impl Logger {
            fn new() -> Self {
                Logger { res: String::new() }
            }
        }
        impl Visitor for Logger {
            fn visit_id(&mut self, name: &str) {
                self.res.push_str(name)
            }
            fn visit_number(&mut self, raw: &str) {
                self.res.push_str(raw)
            }
            fn visit_object(&mut self, attributes: &[(String, Box<Node>)]) {
                self.res.push_str("{ ");
                let mut it = attributes.iter();
                let (key, value) = it.next().unwrap();
                self.res.push_str(key);
                self.res.push_str(": ");
                self.visit(value);

                while let Some((key, value)) = it.next() {
                    self.res.push_str(", ");
                    self.res.push_str(key);
                    self.res.push_str(": ");
                    self.visit(value);
                }

                self.res.push_str(" }");
            }
            fn visit_unary(&mut self, prefix: bool, op: &str, expr: &Box<Node>) {
                if prefix {
                    self.res.push_str(op);
                    self.visit(expr);
                } else {
                    self.visit(expr);
                    self.res.push_str(op);
                }
            }
        }

        let ast = syntax(lex("{foo:1++,bar:++1}")).unwrap();
        let mut w = Logger::new();
        w.visit(&ast);
        assert_eq!("{ foo: 1++, bar: ++1 }", w.res)
    }
}
