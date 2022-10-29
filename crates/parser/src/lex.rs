mod lexed;
// mod token_stream;
mod type_judgument;

pub use lexed::{LexedToken, DFA};

pub type TokenStream = Vec<LexedToken>;
