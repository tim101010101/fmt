use crate::parser::Node;
use crate::syntax_kind::SyntaxKind;
use crate::transform::token_tree::Element;
use crate::traversal::Visitor;

pub(crate) struct Builder {
    root: Element,
}

impl Builder {
    pub(crate) fn build(&mut self) {
        todo!()
    }
}

impl Visitor for Builder {
    fn visit_id(&mut self, kind: &SyntaxKind, name: &str) {
        todo!()
    }

    fn visit_string_literal(&mut self, kind: &SyntaxKind, value: &str, raw: &str) {
        todo!()
    }

    fn visit_number_literal(&mut self, kind: &SyntaxKind, value: i32, raw: &str) {
        todo!()
    }

    fn visit_object_literal(&mut self, kind: &SyntaxKind, attributes: &Vec<(String, Box<Node>)>) {
        todo!()
    }

    fn visit_array_literal(&mut self, kind: &SyntaxKind, items: &Vec<Box<Node>>) {
        todo!()
    }

    fn visit_unary_expr(&mut self, kind: &SyntaxKind, prefix: bool, op: &str, expr: &Box<Node>) {
        todo!()
    }

    fn visit_binary_expr(
        &mut self,
        kind: &SyntaxKind,
        left: &Box<Node>,
        op: &str,
        right: &Box<Node>,
    ) {
        todo!()
    }

    fn visit_ternary_expr(
        &mut self,
        kind: &SyntaxKind,
        condition: &Box<Node>,
        then_expr: &Box<Node>,
        else_expr: &Box<Node>,
    ) {
        todo!()
    }

    fn visit_assignment_expr(&mut self, kind: &SyntaxKind, left: &Box<Node>, right: &Box<Node>) {
        todo!()
    }

    fn visit_value_access_expr(&mut self, kind: &SyntaxKind, path: &Vec<Box<Node>>) {
        todo!()
    }

    fn visit_function_call_expr(
        &mut self,
        kind: &SyntaxKind,
        callee: &Box<Node>,
        args: &Vec<Box<Node>>,
    ) {
        todo!()
    }

    fn visit_return_expr(&mut self, kind: &SyntaxKind, expr: &Box<Node>) {
        todo!()
    }

    fn visit_variable_decla_stat(
        &mut self,
        kind: &SyntaxKind,
        definator: &str,
        naem: &Box<Node>,
        init: &Box<Node>,
    ) {
        todo!()
    }

    fn visit_function_decla_stat(
        &mut self,
        kind: &SyntaxKind,
        name: &Box<Node>,
        args: &Vec<Box<Node>>,
        body: &Vec<Box<Node>>,
    ) {
        todo!()
    }

    fn visit_if_stat(
        &mut self,
        kind: &SyntaxKind,
        expr: &Box<Node>,
        then_block: &Vec<Box<Node>>,
        else_block: &Box<Node>,
    ) {
        todo!()
    }

    fn visit_switch_stat(
        &mut self,
        kind: &SyntaxKind,
        expr: &Box<Node>,
        then_block: &Vec<Box<Node>>,
    ) {
        todo!()
    }

    fn visit_case_stat(
        &mut self,
        kind: &SyntaxKind,
        expr: &Box<Node>,
        has_break: bool,
        then_block: &Vec<Box<Node>>,
    ) {
        todo!()
    }

    fn visit_default_stat(
        &mut self,
        kind: &SyntaxKind,
        has_break: bool,
        then_block: &Vec<Box<Node>>,
    ) {
        todo!()
    }

    fn visit_for_stat(
        &mut self,
        kind: &SyntaxKind,
        init: &Box<Node>,
        condition: &Box<Node>,
        step: &Box<Node>,
        then_block: &Vec<Box<Node>>,
    ) {
        todo!()
    }

    fn visit_while_stat(
        &mut self,
        kind: &SyntaxKind,
        condition: &Box<Node>,
        then_block: &Vec<Box<Node>>,
    ) {
        todo!()
    }
}
