use crate::codegen::generator::{Generator, WhitespaceSurround::*};
use crate::parser::{Node, Stat};
use crate::traversal::Visitor;
use std::ops::Deref;

/// TODO
/// function foo(bar, baz, qux) {
///   // cur width: 120
///   const iter = [1, 2, 3].map().reduce().chainl().zip().flatMap().collect();
/// }
///
/// // set `cur_width: 90`
///
/// function foo(bar, baz, qux) {
///   // cur width: 60
///   const iter = [1, 2, 3]
///                   .map()
///                   .reduce()
///                   .chainl()
///                   .zip()
///                   .flatMap()
///                   .collect();
/// }
///

impl Visitor for Generator {
    fn after_visit_root(&mut self, _: &Node) {
        self.newline();
    }
    fn after_visit_stat(&mut self, s: &Stat) {
        match s {
            Stat::VariableDeclaStatement { .. }
            | Stat::BreakStatement { .. }
            | Stat::ReturnStat { .. } => {
                if self.rules().semi() {
                    self.append(";");
                }
            }
            _ => {}
        }
    }

    fn visit_id(&mut self, name: &str) {
        self.append(name);
    }
    fn visit_string(&mut self, value: &str) {
        let quote = if self.rules().single_quote() {
            "'"
        } else {
            "\""
        };
        self.append(quote);
        self.append(value);
        self.append(quote);
    }
    fn visit_number(&mut self, raw: &str) {
        self.append(raw);
    }
    fn visit_object(&mut self, attributes: &[(String, Box<Node>)]) {
        self.append_token("{", Right);

        let mut it = attributes.iter();
        if let Some((key, value)) = it.next() {
            self.append(key);
            self.append_token(":", Right);
            self.visit(value);

            while let Some((key, value)) = it.next() {
                self.append_token(",", Right);
                self.visit(value);
                self.append(key);
                self.append_token(":", Right);
                self.visit(value);
            }
        }

        self.append_token("}", Left);
    }

    fn visit_array(&mut self, items: &[Box<Node>]) {
        self.append("[");
        self.push_list(items.iter(), ",", Right);
        self.append("]");
    }
    fn visit_unary(&mut self, prefix: bool, op: &str, expr: &Box<Node>) {
        if prefix {
            self.append_token(op, None);
            self.visit(expr);
        } else {
            self.visit(expr);
            self.append_token(op, None);
        }
    }
    fn visit_binary(&mut self, left: &Box<Node>, op: &str, right: &Box<Node>) {
        self.visit(left);
        self.append_token(op, Both);
        self.visit(right)
    }
    fn visit_ternary(
        &mut self,
        condition: &Box<Node>,
        then_expr: &Box<Node>,
        else_expr: &Box<Node>,
    ) {
        self.visit(condition);
        self.append_token("?", Both);
        self.visit(then_expr);
        self.append_token(":", Both);
        self.visit(else_expr);
    }
    fn visit_assignment(&mut self, left: &Box<Node>, right: &Box<Node>) {
        self.visit(left);
        self.append_token("=", Both);
        self.visit(right);
    }
    fn visit_value_access(&mut self, path: &[Box<Node>]) {
        self.push_list(path.iter(), ".", None);
    }
    fn visit_function_call(&mut self, callee: &Box<Node>, args: &[Box<Node>]) {
        self.visit(callee);
        self.append_token("(", None);
        self.push_list(args.iter(), ",", Right);
        self.append_token(")", None);
    }

    fn visit_var_decla(&mut self, definator: &str, name: &Box<Node>, init: &Box<Node>) {
        self.append(definator);
        self.ws();
        self.visit(name);
        self.append_token("=", Both);
        self.visit(init);
    }
    fn visit_fun_decla(&mut self, name: &Box<Node>, args: &[Box<Node>], body: &[Box<Node>]) {
        self.append_token("function", Right);
        self.visit(name);

        self.append_token("(", None);
        self.push_list(args.iter(), ",", Right);
        self.append_token(")", Right);
        self.push_block(body.iter());
    }
    fn visit_if(&mut self, expr: &Box<Node>, then_block: &[Box<Node>], else_node: &Box<Node>) {
        self.append("if");
        self.append_token("(", Left);
        self.visit(expr);
        self.append_token(")", Right);
        self.push_block(then_block.iter());

        match else_node.deref() {
            Node::Stat(Stat::IfStatement {
                           expr,
                           then_block,
                           else_node,
                           ..
                       }) => self.visit_else_if(expr, then_block, else_node),
            _ => {}
        }
    }
    fn visit_else_if(&mut self, expr: &Box<Node>, then_block: &[Box<Node>], else_node: &Box<Node>) {
        match expr.deref() {
            Node::Empty => {
                self.visit_else(then_block);
            }
            _ => {
                self.append_token("else if", Both);
                self.append("(");
                self.visit(expr);
                self.append_token(")", Right);
                self.push_block(then_block.iter());

                match else_node.deref() {
                    Node::Stat(Stat::IfStatement {
                                   expr,
                                   then_block,
                                   else_node,
                                   ..
                               }) => self.visit_else_if(expr, then_block, else_node),
                    _ => {}
                }
            }
        }
    }
    fn visit_else(&mut self, then_block: &[Box<Node>]) {
        self.append_token("else", Both);
        self.push_block(then_block.iter());
    }
    fn visit_switch(&mut self, expr: &Box<Node>, then_block: &[Box<Node>]) {
        self.append("switch");
        self.append_token("(", Left);
        self.visit(expr);
        self.append_token(")", Right);
        self.push_block(then_block.iter())
    }
    fn visit_case(&mut self, expr: &Box<Node>, then_block: &[Box<Node>]) {
        self.append_token("case", Right);
        self.visit(expr);
        self.append_token(":", None);
        self.tab();
        self.newline();
        self.push_list(then_block.iter(), "\n", None);
        self.backspace();
    }
    fn visit_default(&mut self, then_block: &[Box<Node>]) {
        self.append("default");
        self.append_token(":", None);
        self.tab();
        self.newline();
        self.push_list(then_block.iter(), "\n", None);
        self.backspace();
    }
    fn visit_break(&mut self) {
        self.append("break");
    }
    fn visit_for(
        &mut self,
        init: &Box<Node>,
        condition: &Box<Node>,
        step: &Box<Node>,
        then_block: &[Box<Node>],
    ) {
        self.append_token("for", Right);
        self.append_token("(", None);
        self.visit(init);
        self.ws();
        self.visit(condition);
        self.append_token(";", Right);
        self.visit(step);
        self.append_token(")", Right);
        self.push_block(then_block.iter());
    }
    fn visit_while(&mut self, condition: &Box<Node>, then_block: &[Box<Node>]) {
        self.append_token("while", Right);
        self.append_token("(", None);
        self.visit(condition);
        self.append_token(")", Right);
        self.push_block(then_block.iter());
    }
    fn visit_return(&mut self, expr: &Box<Node>) {
        self.append_token("return", Right);
        self.visit(expr);
    }
}

#[cfg(test)]
mod tests {
    use crate::codegen::generator::Generator;
    use crate::parser::{lex, syntax};

    #[test]
    fn smoke() {
        let input = r#"const a=1;function foo(bar,baz){const a=1;}if(a==1){const a=1;}else if(b==2){const b=1;}else if(c==2){const c=1;}else{const d=1;}switch(a){case 1:const b=1;break;case 2:const c=1;break;default:const d=1;break;}for(let i=0;i<1;i++){let a = 1;}while(a){const a=1;}"#;
        let ast = syntax(lex(input)).unwrap();
        let mut g = Generator::new();
        let expect = r#"const a = 1;
function foo(bar, baz) {
  const a = 1;
}
if (a == 1) {
  const a = 1;
} else if (b == 2) {
  const b = 1;
} else if (c == 2) {
  const c = 1;
} else {
  const d = 1;
}
switch (a) {
  case 1:
    const b = 1;
  break;
  case 2:
    const c = 1;
  break;
  default:
    const d = 1;
  break;
}
for (let i = 0; i < 1; i++) {
  let a = 1;
}
while (a) {
  const a = 1;
}
"#;
        let output = g.gen(&ast);
        assert_eq!(output, expect);
    }

    #[test]
    fn smoke1() {
        let input = r#"const a=1;function foo(bar,baz){const a=1;if(a==1){const a=1;}else if(b==2){const b=1;}}"#;
        let ast = syntax(lex(input)).unwrap();
        let mut g = Generator::new();
        let output = g.gen(&ast);
        println!("{}", output);
    }
}
