use crate::codegen::Generator;
use crate::parser::visitor::Visitor;
use crate::parser::Node;

pub(crate) fn unary_expr(
    g: &mut Generator,
    prefix: bool,
    op: &str,
    expr: &Node,
) {
    if prefix {
        g.push(op);
        g.visit(expr);
    } else {
        g.visit(expr);
        g.push(op);
    }
}

pub(crate) fn binary_expr(
    g: &mut Generator,
    left: &Node,
    op: &str,
    right: &Node,
) {
    g.visit(left);
    g.push(op);
    g.visit(right);
}

pub(crate) fn ternary_expr(
    g: &mut Generator,
    condition: &Node,
    then_expr: &Node,
    else_expr: &Node,
) {
    g.visit(condition);
    g.push("?");
    g.visit(then_expr);
    g.push(":");
    g.visit(else_expr);
}

pub(crate) fn assignment_expr(
    g: &mut Generator,
    left: &Node,
    right: &Node,
) {
    g.visit(left);
    g.push("=");
    g.visit(right)
}

pub(crate) fn value_access_expr(
    g: &mut Generator,
    path: &Vec<Box<Node>>,
) {
    let mut i = path.iter();
    match i.next() {
        None => (),
        Some(first) => {
            g.visit(first);
            i.for_each(|p| {
                g.push(".");
                g.visit(p);
            });
        }
    }
}

pub(crate) fn function_call_expr(
    g: &mut Generator,
    callee: &Box<Node>,
    args: &Vec<Box<Node>>,
) {
    g.visit(callee);
    g.push("(");
    args.iter().for_each(|a| g.visit(a));
    g.push(")");
}

pub(crate) fn return_expr(g: &mut Generator, expr: &Node) {
    g.push("return");
    g.visit(expr);
}
