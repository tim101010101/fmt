mod grammar;
mod node;
mod tree;
mod visit;

pub(crate) use node::{expr_node, literal_node, stat_node};
pub use node::{Expr, Literal, Node, Stat};

use crate::parser::TokenStream;
use grammar::root;
use shared::parser_combiner::Parser;

pub fn syntax(
    token_stream: TokenStream,
) -> Result<Node, String> {
    match root().parse(token_stream) {
        Ok((_, node)) => Ok(node),
        Err(_) => Err("Parsing failed".to_string()),
    }
}
