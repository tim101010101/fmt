use crate::parser::Node;

pub(crate) trait LiteralVisitor {
    fn visit_id(&mut self, name: &str) {}
    fn visit_string_literal(&mut self, value: &str, raw: &str) {}
    fn visit_number_literal(&mut self, value: i32, raw: &str) {}
    fn visit_object_literal(&mut self, attributes: &Vec<(String, Box<Node>)>) {}
    fn visit_array_literal(&mut self, items: &Vec<Box<Node>>) {}
}

pub(crate) trait ExprVisitor {}

pub(crate) trait StatVisitor {}
