mod lexed;
mod type_judgument;

pub use lexed::{LexedToken, DFA};

pub type TokenStream = Vec<LexedToken>;

pub fn lex(code: &str) -> TokenStream {
    let mut dfa = DFA::new(code.to_string());
    dfa.lexed(true);
    dfa.token_stream
}
