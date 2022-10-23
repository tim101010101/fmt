use crate::ast::{Node, Parser};
use crate::lex::{Token, NFA};

mod ast;
mod lex;
mod syntax_kind;

pub fn lex(code: &str) -> Vec<Token> {
    let mut nfa = NFA::new(code.to_string());
    nfa.lexed(false);
    nfa.token_stream
}

pub fn syntax(token_stream: Vec<Token>) -> Option<Node> {
    Parser::new(token_stream).parse()
}

#[cfg(test)]
mod tests {
    use crate::{lex, syntax};

    fn get_source_code() -> String {
        "function foo () {\
            const bar = \"baz\";\
        }"
        .to_string()
    }

    #[test]
    fn smoke() {
        let source = get_source_code();
        let token_stream = lex(&source);
        let ast = syntax(token_stream).unwrap();
        println!("{}", ast);
        // function foo () {const bar = "baz";}
    }
}
