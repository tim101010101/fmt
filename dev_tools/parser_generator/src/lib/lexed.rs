use crate::lib::{
    token_kind::TokenKind::{self, *},
    type_judgument::{is_blank as is_blank_from_str, is_number},
};
use regex::Regex;

pub struct NFA {
    code: String,
    lexical_stream: Vec<String>,
    token_stream: Vec<(TokenKind, String)>,
}

impl NFA {
    pub fn new(code: String) -> Self {
        NFA {
            code,
            lexical_stream: Vec::new(),
            token_stream: Vec::new(),
        }
    }

    pub fn lexed(&mut self) {
        let state_table = [
            (0, 0, 0, 0, 0),
            (2, 3, 0, 5, 6),
            (2, 3, 0, 5, 6),
            (3, 3, 4, 3, 3),
            (2, 3, 0, 5, 6),
            (2, 3, 0, 5, 6),
            (2, 3, 0, 5, 6),
        ];
        let end_state = [2, 4, 5, 6];

        let mut idx = 0;
        let mut state = 1;
        let mut prev_state = 0;
        let mut text_cache = String::new();
        let mut lexical_stream: Vec<String> = Vec::new();
        while let Some(c) = self.code.chars().nth(idx) {
            if is_operator(c) {
                state = state_table[state].0;
            } else if is_block_start(c) {
                state = state_table[state].1;
            } else if is_block_end(c) {
                state = state_table[state].2;
            } else if is_strip(c) {
                state = state_table[state].3;
            } else if is_blank(c) {
                state = state_table[state].4;
            } else {
                state = 0;
            }

            if prev_state == 2 || end_state.contains(&prev_state) && state != prev_state {
                lexical_stream.push(text_cache.clone());
                text_cache.clear();
                text_cache.push_str(&c.to_string());
            } else {
                text_cache.push_str(&c.to_string());
            }

            idx += 1;
            prev_state = state;
        }
        lexical_stream.push(text_cache.clone());

        self.lexical_stream = lexical_stream;
    }

    pub fn tokenize(&mut self) {
        for s in self.lexical_stream.iter() {
            let text = s.to_string();
            let token = if let Some(token_kind) = TokenKind::from_str(s) {
                (token_kind, text)
            } else if let Some(token_kind) = TokenKind::from_keyword(s) {
                (token_kind, text)
            } else {
                if is_blank_from_str(s) {
                    (WHITE_SPACE, text)
                } else if is_number(s) {
                    (NUMBER, text)
                } else {
                    (ID, text)
                }
            };

            self.token_stream.push(token);
        }
    }
}

fn is_blank(c: char) -> bool {
    match c {
        '\n' | '\t' | ' ' => true,
        _ => false,
    }
}

fn is_operator(c: char) -> bool {
    match c {
        ';' | ',' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | '?' | ':' | '$' | '=' | '!'
        | '~' | '&' | '|' | '+' | '*' | '/' | '^' | '%' => true,
        _ => false,
    }
}

fn is_strip(c: char) -> bool {
    Regex::new(r"[a-zA-Z0-9]").unwrap().is_match(&c.to_string())
}

fn is_block_start(c: char) -> bool {
    match c {
        '"' | '\'' => true,
        _ => false,
    }
}

fn is_block_end(c: char) -> bool {
    match c {
        '"' | '\'' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        // assert_eq!(SEMI, T![;]);
        // assert_eq!(COMMA, T![,]);
    }

    #[test]
    fn text_nfa() {
        let code = "const      a = 1+ 2;".to_string();
        let mut nfa = NFA::new(code);
        nfa.lexed();
        assert_eq!(
            vec!["const", "      ", "a", " ", "=", " ", "1", "+", " ", "2", ";"],
            nfa.lexical_stream
        )
    }

    #[test]
    fn text_nfa_2() {
        let code = "a+=2;".to_string();
        let mut nfa = NFA::new(code);
        nfa.lexed();
        assert_eq!(vec!["a", "+", "=", "2", ";"], nfa.lexical_stream);
    }

    #[test]
    fn text_nfa_1() {
        let code = "const a=1+2;\
        var b=(3+2)*3;"
            .to_string();
        let mut nfa = NFA::new(code);
        nfa.lexed();
        assert_eq!(
            vec![
                "const", " ", "a", "=", "1", "+", "2", ";", "var", " ", "b", "=", "(", "3", "+",
                "2", ")", "*", "3", ";",
            ],
            nfa.lexical_stream
        )
    }

    #[test]
    fn test_tokenize_1() {
        let code = "const   a  = 1 += 2;".to_string();
        let mut nfa = NFA::new(code);
        nfa.lexed();
        nfa.tokenize();
        assert_eq!(
            vec![
                (CONST, "const".to_string()),
                (WHITE_SPACE, "   ".to_string()),
                (ID, "a".to_string()),
                (WHITE_SPACE, "  ".to_string()),
                (EQ, "=".to_string()),
                (WHITE_SPACE, " ".to_string()),
                (NUMBER, "1".to_string()),
                (WHITE_SPACE, " ".to_string()),
                (PLUS, "+".to_string()),
                (EQ, "=".to_string()),
                (WHITE_SPACE, " ".to_string()),
                (NUMBER, "2".to_string()),
                (SEMI, ";".to_string())
            ],
            nfa.token_stream
        )
    }
}
