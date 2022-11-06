mod grammar;
mod node;
mod tree;
mod visit;

pub use node::Node;

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
