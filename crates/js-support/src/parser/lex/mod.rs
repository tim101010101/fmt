mod lexed;
pub(crate) mod type_judgument;

use lexed::DFA;

pub(crate) use lexed::LexedToken;
pub(crate) type TokenStream = Vec<LexedToken>;

pub fn lex(code: &str) -> TokenStream {
    let mut dfa = DFA::new(code.to_string());
    dfa.lexed(true);
    dfa.token_stream
}
