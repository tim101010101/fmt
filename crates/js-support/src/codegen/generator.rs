use crate::parser::{Expr, Literal, Node, Stat};
use crate::traversal::Visitor;

pub(crate) struct Generator {
    output: String,
    indent_level: usize,
}
impl Generator {
    pub(crate) fn new() -> Self {
        Generator {
            output: String::new(),
            indent_level: 0,
        }
    }
    pub(crate) fn gen(&mut self, root: &Node) -> &str {
        self.visit(root);
        self.output.as_str()
    }

    fn push(&mut self, str: &str) {
        self.output.push_str(str);
    }
    fn push_list<'input, I>(&mut self, mut iter: I, sep: &str)
    where
        I: Iterator<Item = &'input Box<Node>>,
    {
        if let Some(item) = iter.next() {
            self.visit(item);

            while let Some(item) = iter.next() {
                self.push(sep);
                self.visit(item);
            }
        }
    }
    fn ws(&mut self) {
        self.push(" ");
    }
    fn newline(&mut self) {
        self.push("\n");
    }
    fn tab(&mut self) {
        self.indent_level += 1;
        self.indent();
    }
    fn backspace(&mut self) {
        self.indent_level -= 1;
        self.indent();
    }
    fn indent(&mut self) {
        let indent = "\t".repeat(self.indent_level);
        self.push(&indent);
    }
}
impl Visitor for Generator {
    fn visit_id(&mut self, name: &str) {
        self.push(name);
    }
    fn visit_string(&mut self, value: &str) {
        self.push(value);
    }
    fn visit_number(&mut self, raw: &str) {
        self.push(raw);
    }
    fn visit_object(&mut self, attributes: &[(String, Box<Node>)]) {
        self.push("{ ");

        let mut it = attributes.iter();
        if let Some((key, value)) = it.next() {
            self.push(key);
            self.push(": ");
            self.visit(value);

            while let Some((key, value)) = it.next() {
                self.push(", ");
                self.push(key);
                self.push(": ");
                self.visit(value);
            }
        }

        self.push(" }");
    }
    fn visit_array(&mut self, items: &[Box<Node>]) {
        self.push("[ ");

        // let mut it = items.iter();
        // if let Some(item) = it.next() {
        //     self.visit(item);
        //
        //     while let Some(item) = it.next() {
        //         self.push(", ");
        //         self.visit(item);
        //     }
        // }
        self.push_list(items.iter(), ", ");

        self.push(" ]");
    }

    fn visit_unary(&mut self, prefix: bool, op: &str, expr: &Box<Node>) {
        if prefix {
            self.push(op);
            self.visit(expr);
        } else {
            self.visit(expr);
            self.push(op);
        }
    }
    fn visit_binary(&mut self, left: &Box<Node>, op: &str, right: &Box<Node>) {
        self.visit(left);
        self.ws();
        self.push(op);
        self.ws();
        self.visit(right)
    }
    fn visit_ternary(
        &mut self,
        condition: &Box<Node>,
        then_expr: &Box<Node>,
        else_expr: &Box<Node>,
    ) {
        self.visit(condition);
        self.push(" ? ");
        self.ws();
        self.visit(then_expr);
        self.push(": ");
        self.ws();
        self.visit(else_expr);
    }
    fn visit_assignment(&mut self, left: &Box<Node>, right: &Box<Node>) {
        self.visit(left);
        self.ws();
        self.push("=");
        self.ws();
        self.visit(right);
    }
    fn visit_value_access(&mut self, path: &[Box<Node>]) {
        // let mut it = path.iter();
        // if let Some(item) = it.next() {
        //     self.visit(item);
        //
        //     while let Some(item) = it.next() {
        //         self.push(".");
        //         self.visit(item);
        //     }
        // }
        self.push_list(path.iter(), ".");
    }
    fn visit_function_call(&mut self, callee: &Box<Node>, args: &[Box<Node>]) {
        self.visit(callee);
        self.push("(");

        // let mut it = args.iter();
        // if let Some(arg) = it.next() {
        //     self.visit(arg);
        //     while let Some(arg) = it.next() {
        //         self.push(", ");
        //         self.visit(arg);
        //     }
        // }
        self.push_list(args.iter(), ", ");

        self.push(")");
    }
    fn visit_return(&mut self, expr: &Box<Node>) {
        self.push("return");
        self.ws();
        self.visit(expr);
    }

    fn visit_var_decla(&mut self, definator: &str, name: &Box<Node>, init: &Box<Node>) {
        self.push(definator);
        self.ws();
        self.visit(name);
        self.ws();
        self.push("=");
        self.ws();
        self.visit(init);
    }
    fn visit_fun_decla(&mut self, name: &Box<Node>, args: &[Box<Node>], body: &[Box<Node>]) {
        self.push("function");
        self.ws();
        self.visit(name);

        self.push_list(args.iter(), ", ");
        self.push_list(body.iter(), "\n");
    }
    fn visit_if(&mut self, expr: &Box<Node>, then_block: &[Box<Node>], else_block: &Box<Node>) {
        self.push("if");
        self.ws();
        self.push("(");
        self.visit(expr);
        self.push(")");
        self.ws();
        self.push("{");
        self.ws();
        self.push_list(then_block.iter(), "\n");
        self.push("}");

        self.visit(else_block);
    }
    fn visit_switch(&mut self, expr: &Box<Node>, then_block: &[Box<Node>]) {
        self.push("switch");
        self.ws();
        self.visit(expr);
        self.ws();
        self.push("{");
        self.push_list(then_block.iter(), "\n");
        self.push("}");
    }
    fn visit_case(&mut self, expr: &Box<Node>, has_break: bool, then_block: &[Box<Node>]) {
        self.push("case");
        self.ws();
        self.push(":");
        self.visit(expr);
        self.push_list(then_block.iter(), "\n");

        if has_break {
            self.push("break");
        }
    }
    fn visit_default(&mut self, has_break: bool, then_block: &[Box<Node>]) {
        self.push("default");
        self.push_list(then_block.iter(), "\n");

        if has_break {
            self.push("break");
        }
    }
    fn visit_for(
        &mut self,
        init: &Box<Node>,
        condition: &Box<Node>,
        step: &Box<Node>,
        then_block: &[Box<Node>],
    ) {
        self.push("for");
        self.ws();
        self.push("(");
        self.visit(init);
        self.push("; ");
        self.visit(condition);
        self.push("; ");
        self.visit(step);
        self.push(")");
        self.ws();
        self.push("{");
        self.ws();
        self.push_list(then_block.iter(), "\n");
        self.push("}");
    }
    fn visit_while(&mut self, condition: &Box<Node>, then_block: &[Box<Node>]) {
        self.push("while");
        self.ws();
        self.push("(");
        self.visit(condition);
        self.push(")");
        self.ws();
        self.push("{");
        self.push_list(then_block.iter(), "\n");
        self.push("}");
    }
}

#[cfg(test)]
mod tests {
    use crate::codegen::generator::Generator;
    use crate::parser::{lex, syntax};

    #[test]
    fn smoke() {
        let ast = syntax(lex("const a=1++;")).unwrap();
        let mut g = Generator::new();
        println!("{}", g.gen(&ast));
    }
}
