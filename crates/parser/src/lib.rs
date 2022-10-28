// use crate::ast::{Node, Parser};
use crate::lex::{LexedToken, DFA};

mod ast;
mod lex;
mod syntax_kind;

pub fn lex(code: &str) -> Vec<LexedToken> {
    let mut dfa = DFA::new(code.to_string());
    dfa.lexed(false);
    dfa.token_stream
}
//
// pub fn syntax(token_stream: Vec<Token>) -> Option<Node> {
//     Parser::new(token_stream).parse()
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::{lex, syntax};
//
//     fn get_source_code() -> String {
//         r#"function foo () {
//             const bar = "baz";
//         }
//
//         var bar = "foo"
//
//         function bar(foo, bar, a, b) {
//             let a = 1;
//         }"#
//         .to_string()
//     }
//
//     #[test]
//     fn smoke() {
//         let source = get_source_code();
//         let token_stream = lex(&source);
//         let ast = syntax(token_stream).unwrap();
//         println!("{}", ast);
//     }
// }
