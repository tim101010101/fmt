use crate::parser::visitor::Visitor;
use crate::parser::{Expr, Literal, Node, Stat};
use crate::rules::*;
use crate::syntax_kind::SyntaxKind;

pub(crate) struct Generator {
    output: String,
    indent_kind: String,
    indent_level: usize,
}

impl Generator {
    pub(crate) fn new() -> Self {
        Generator {
            output: String::new(),
            indent_kind: "\t".to_string(),
            indent_level: 1,
        }
    }
    pub(crate) fn walk(&mut self, root: &Node) {
        self.visit(root)
    }
    pub(crate) fn dump(&self) -> String {
        self.output.to_owned()
    }
    pub(crate) fn ws(&mut self) {
        self.push(" ");
    }
    pub(crate) fn tab(&mut self) {
        self.indent();
        self.indent_level += 1;
    }
    pub(crate) fn backspace(&mut self) {
        self.indent();
        self.indent_level -= 1;
    }
    pub(crate) fn new_line(&mut self) {
        self.push("\n");
        self.indent();
    }
    pub(crate) fn push(&mut self, code: &str) {
        self.output.push_str(code);
    }

    fn indent(&mut self) {
        self.push(
            self.indent_kind
                .repeat(self.indent_level)
                .as_str(),
        );
    }
}

impl Visitor for Generator {
    fn visit_literal(&mut self, n: &Literal) {
        match n {
            Literal::Id { name, .. } => self.push(name),

            Literal::StringLiteral { value, .. } => {
                self.push(value)
            }

            Literal::NumberLiteral { raw, .. } => {
                self.push(raw)
            }

            Literal::ObjectLiteral {
                attributes, ..
            } => object_literal(self, attributes),

            Literal::ArrayLiteral { items, .. } => {
                array_literal(self, items)
            }
        }
    }
    fn visit_expr(&mut self, n: &Expr) {
        match n {
            Expr::UnaryExpr {
                prefix, op, expr, ..
            } => unary_expr(
                self,
                *prefix,
                SyntaxKind::to_str(op),
                expr,
            ),

            Expr::BinaryExpr {
                left, op, right, ..
            } => binary_expr(
                self,
                left,
                SyntaxKind::to_str(op),
                right,
            ),

            Expr::TernaryExpr {
                condition,
                then_expr,
                else_expr,
                ..
            } => ternary_expr(
                self, condition, then_expr, else_expr,
            ),

            Expr::AssignmentExpr {
                left, right, ..
            } => assignment_expr(self, left, right),

            Expr::ValueAccessExpr { path, .. } => {
                value_access_expr(self, path)
            }

            Expr::FunctionCallExpr {
                callee, args, ..
            } => function_call_expr(self, callee, args),

            Expr::ReturnExpr { expr, .. } => {
                return_expr(self, expr)
            }
        }
    }
    fn visit_stat(&mut self, n: &Stat) {
        match n {
            Stat::VariableDeclaStatement { .. } => {}
            Stat::FunctionDeclaStatement { .. } => {}
            Stat::IfStatement { .. } => {}
            Stat::SwitchStatement { .. } => {}
            Stat::CaseStatement { .. } => {}
            Stat::DefaultStatement { .. } => {}
            Stat::ForStatement { .. } => {}
            Stat::WhileStatement { .. } => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{lex, syntax};

    fn get_input(code: &str) -> Node {
        syntax(lex(code)).unwrap()
    }

    #[test]
    fn test_unary_expr() {
        let mut g = Generator::new();
        let input = get_input("++1");
        g.walk(&input);
        assert_eq!("++1", g.dump());

        let mut g = Generator::new();
        let input = get_input("1++");
        g.walk(&input);
        assert_eq!("1++", g.dump());
    }

    #[test]
    fn binary_expr() {
        let mut g = Generator::new();
        let input = get_input("1+1");
        g.walk(&input);
        assert_eq!("1+1", g.dump());
    }

    #[test]
    fn value_access_expr() {
        let mut g = Generator::new();
        let input = get_input("foo.bar");
        g.walk(&input);
        assert_eq!("foo.bar", g.dump());
    }
}
