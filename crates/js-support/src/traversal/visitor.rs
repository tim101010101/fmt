use crate::parser::Node;
use crate::syntax_kind::SyntaxKind;

// make function declarations shorter....
type BN = Box<Node>;
type SK = SyntaxKind;

pub(crate) trait Visitor {
    fn visit_id(&mut self, kind: &SK, name: &str) {}
    fn visit_string_literal(&mut self, kind: &SK, value: &str, raw: &str) {}
    fn visit_number_literal(&mut self, kind: &SK, value: i32, raw: &str) {}
    fn visit_object_literal(&mut self, kind: &SK, attributes: &Vec<(String, BN)>) {}
    fn visit_array_literal(&mut self, kind: &SK, items: &Vec<BN>) {}

    fn visit_unary_expr(&mut self, kind: &SK, prefix: bool, op: &str, expr: &BN) {}
    fn visit_binary_expr(&mut self, kind: &SK, left: &BN, op: &str, right: &BN) {}
    fn visit_ternary_expr(&mut self, kind: &SK, condition: &BN, then_expr: &BN, else_expr: &BN) {}
    fn visit_assignment_expr(&mut self, kind: &SK, left: &BN, right: &BN) {}
    fn visit_value_access_expr(&mut self, kind: &SK, path: &Vec<BN>) {}
    fn visit_function_call_expr(&mut self, kind: &SK, callee: &BN, args: &Vec<BN>) {}
    fn visit_return_expr(&mut self, kind: &SK, expr: &BN) {}

    fn visit_variable_decla_stat(&mut self, kind: &SK, definator: &str, naem: &BN, init: &BN) {}
    fn visit_function_decla_stat(&mut self, kind: &SK, name: &BN, args: &Vec<BN>, body: &Vec<BN>) {}
    fn visit_if_stat(&mut self, kind: &SK, expr: &BN, then_block: &Vec<BN>, else_block: &BN) {}
    fn visit_switch_stat(&mut self, kind: &SK, expr: &BN, then_block: &Vec<BN>) {}
    fn visit_case_stat(&mut self, kind: &SK, expr: &BN, has_break: bool, then_block: &Vec<BN>) {}
    fn visit_default_stat(&mut self, kind: &SK, has_break: bool, then_block: &Vec<BN>) {}
    fn visit_for_stat(
        &mut self,
        kind: &SK,
        init: &BN,
        condition: &BN,
        step: &BN,
        then_block: &Vec<BN>,
    ) {
    }
    fn visit_while_stat(&mut self, kind: &SK, condition: &BN, then_block: &Vec<BN>) {}
}
