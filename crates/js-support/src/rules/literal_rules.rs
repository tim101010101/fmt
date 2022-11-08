use crate::codegen::Generator;
use crate::parser::visitor::Visitor;
use crate::parser::Node;

pub(crate) fn object_literal(
    g: &mut Generator,
    attributes: &Vec<(String, Box<Node>)>,
) {
    g.push("{");
    attributes.iter().for_each(|(key, value)| {
        g.ws();
        g.push(key);
        g.push(":");
        g.ws();
        g.visit(value);
        g.push(",");
    });
    g.push("}");
}

pub(crate) fn array_literal(
    g: &mut Generator,
    items: &Vec<Box<Node>>,
) {
    g.push("[");
    items.iter().for_each(|item| {
        g.visit(item);
        g.push(",");
    });
    g.push("]");
}
