use crate::parser::{Expr, Literal, Node, Stat};
use crate::syntax_kind::SyntaxKind;

pub(crate) trait Visitor {
    fn visit(&mut self, n: &Node) {
        match n {
            Node::Empty => {}
            Node::Root { statements, .. } => statements.iter().for_each(|s| self.visit(s)),
            Node::Literal(l) => self.visit_literal(l),
            Node::Expr(e) => self.visit_expr(e),
            Node::Stat(s) => self.visit_stat(s),
        }
    }
    fn visit_literal(&mut self, l: &Literal) {
        match l {
            Literal::Id { name, .. } => self.visit_id(name),
            Literal::StringLiteral { value, .. } => self.visit_string(value),
            Literal::NumberLiteral { raw, .. } => self.visit_number(raw),
            Literal::ObjectLiteral { attributes, .. } => self.visit_object(attributes),
            Literal::ArrayLiteral { items, .. } => self.visit_array(items),
        }
    }
    fn visit_expr(&mut self, e: &Expr) {
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
            Expr::ReturnExpr { expr, .. } => self.visit_return(expr),
        }
    }
    fn visit_stat(&mut self, s: &Stat) {
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
                else_block,
                ..
            } => self.visit_if(expr, then_block, else_block),
            Stat::SwitchStatement {
                expr, then_block, ..
            } => self.visit_switch(expr, then_block),
            Stat::CaseStatement {
                expr,
                has_break,
                then_block,
                ..
            } => self.visit_case(expr, *has_break, then_block),
            Stat::DefaultStatement {
                has_break,
                then_block,
                ..
            } => self.visit_default(*has_break, then_block),
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
        }
    }

    fn visit_id(&mut self, name: &str) {}
    fn visit_string(&mut self, value: &str) {}
    fn visit_number(&mut self, raw: &str) {}
    fn visit_object(&mut self, attributes: &[(String, Box<Node>)]) {}
    fn visit_array(&mut self, items: &[Box<Node>]) {}

    fn visit_unary(&mut self, prefix: bool, op: &str, expr: &Box<Node>) {}
    fn visit_binary(&mut self, left: &Box<Node>, op: &str, right: &Box<Node>) {}
    fn visit_ternary(
        &mut self,
        condition: &Box<Node>,
        then_expr: &Box<Node>,
        else_expr: &Box<Node>,
    ) {
    }
    fn visit_assignment(&mut self, left: &Box<Node>, right: &Box<Node>) {}
    fn visit_value_access(&mut self, path: &[Box<Node>]) {}
    fn visit_function_call(&mut self, callee: &Box<Node>, args: &[Box<Node>]) {}
    fn visit_return(&mut self, expr: &Box<Node>) {}

    fn visit_var_decla(&mut self, definator: &str, name: &Box<Node>, init: &Box<Node>) {}
    fn visit_fun_decla(&mut self, name: &Box<Node>, args: &[Box<Node>], body: &[Box<Node>]) {}
    fn visit_if(&mut self, expr: &Box<Node>, then_block: &[Box<Node>], else_block: &Box<Node>) {}
    fn visit_switch(&mut self, expr: &Box<Node>, then_block: &[Box<Node>]) {}
    fn visit_case(&mut self, expr: &Box<Node>, has_break: bool, then_block: &[Box<Node>]) {}
    fn visit_default(&mut self, has_break: bool, then_block: &[Box<Node>]) {}
    fn visit_for(
        &mut self,
        init: &Box<Node>,
        condition: &Box<Node>,
        step: &Box<Node>,
        then_block: &[Box<Node>],
    ) {
    }
    fn visit_while(&mut self, condition: &Box<Node>, then_block: &[Box<Node>]) {}
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
            fn visit_number(&mut self, raw: &str) {
                self.res.push_str(raw)
            }
            fn visit_id(&mut self, name: &str) {
                self.res.push_str(name)
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
