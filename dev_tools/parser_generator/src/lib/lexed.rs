use crate::lib::{
    token_kind::TokenKind::{self, *},
    type_judgument::{is_blank as is_blank_from_str, is_number},
};
use regex::Regex;

pub struct NFA {
    code: String,
    token_stream: Vec<(TokenKind, String, (usize, usize))>,
}

impl NFA {
    pub fn new(code: String) -> Self {
        NFA {
            code,
            token_stream: Vec::new(),
        }
    }

    pub fn lexed(&mut self, skip_blank: bool) {
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
                self.push_token(&text_cache, &idx, skip_blank);
                text_cache.clear();
                text_cache.push_str(&c.to_string());
            } else {
                text_cache.push_str(&c.to_string());
            }

            idx += 1;
            prev_state = state;
        }
        self.push_token(&text_cache, &idx, skip_blank);
    }

    fn push_token(&mut self, text: &str, idx: &usize, skip_blank: bool) {
        if skip_blank && is_blank_from_str(text) {
            return;
        }

        let text = text.to_string();
        let loc = (idx - &text.len(), *idx);
        let token: (TokenKind, String, (usize, usize)) =
            if let Some(token_kind) = TokenKind::from_str(&text) {
                (token_kind, text, loc)
            } else if let Some(token_kind) = TokenKind::from_keyword(&text) {
                (token_kind, text, loc)
            } else {
                if is_blank_from_str(&text) {
                    (WHITE_SPACE, text, loc)
                } else if is_number(&text) {
                    (NUMBER, text, loc)
                } else {
                    (ID, text, loc)
                }
            };
        self.token_stream.push(token);
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
    fn text_nfa() {
        let code = "const      a = 1+ 2;".to_string();
        let mut nfa = NFA::new(code);
        nfa.lexed(false);
        assert_eq!(
            vec![
                (CONST, "const".to_string(), (0, 5)),
                (WHITE_SPACE, "      ".to_string(), (5, 11)),
                (ID, "a".to_string(), (11, 12)),
                (WHITE_SPACE, " ".to_string(), (12, 13)),
                (EQ, "=".to_string(), (13, 14)),
                (WHITE_SPACE, " ".to_string(), (14, 15)),
                (NUMBER, "1".to_string(), (15, 16)),
                (PLUS, "+".to_string(), (16, 17)),
                (WHITE_SPACE, " ".to_string(), (17, 18)),
                (NUMBER, "2".to_string(), (18, 19)),
                (SEMI, ";".to_string(), (19, 20))
            ],
            nfa.token_stream
        )
    }

    #[test]
    fn text_nfa_2() {
        let code = "a+=2;".to_string();
        let mut nfa = NFA::new(code);
        nfa.lexed(false);
        assert_eq!(
            vec![
                (ID, "a".to_string(), (0, 1)),
                (PLUS, "+".to_string(), (1, 2)),
                (EQ, "=".to_string(), (2, 3)),
                (NUMBER, "2".to_string(), (3, 4)),
                (SEMI, ";".to_string(), (4, 5))
            ],
            nfa.token_stream
        )
    }

    #[test]
    fn text_nfa_1() {
        let code = "const a=1+2;\
        var b=(3+2)*3;"
            .to_string();
        let mut nfa = NFA::new(code);
        nfa.lexed(true);
        assert_eq!(
            vec![
                (CONST, "const".to_string(), (0, 5)),
                (ID, "a".to_string(), (6, 7)),
                (EQ, "=".to_string(), (7, 8)),
                (NUMBER, "1".to_string(), (8, 9)),
                (PLUS, "+".to_string(), (9, 10)),
                (NUMBER, "2".to_string(), (10, 11)),
                (SEMI, ";".to_string(), (11, 12)),
                (VAR, "var".to_string(), (12, 15)),
                (ID, "b".to_string(), (16, 17)),
                (EQ, "=".to_string(), (17, 18)),
                (OPEN_PAREN, "(".to_string(), (18, 19)),
                (NUMBER, "3".to_string(), (19, 20)),
                (PLUS, "+".to_string(), (20, 21)),
                (NUMBER, "2".to_string(), (21, 22)),
                (CLOSE_PAREN, ")".to_string(), (22, 23)),
                (STAR, "*".to_string(), (23, 24)),
                (NUMBER, "3".to_string(), (24, 25)),
                (SEMI, ";".to_string(), (25, 26))
            ],
            nfa.token_stream
        )
    }

    #[test]
    fn test_tokenize_1() {
        let code = "const   a  = 1 += 2;".to_string();
        let mut nfa = NFA::new(code);
        nfa.lexed(true);
        assert_eq!(
            vec![
                (CONST, "const".to_string(), (0, 5)),
                (ID, "a".to_string(), (8, 9)),
                (EQ, "=".to_string(), (11, 12)),
                (NUMBER, "1".to_string(), (13, 14)),
                (PLUS, "+".to_string(), (15, 16)),
                (EQ, "=".to_string(), (16, 17)),
                (NUMBER, "2".to_string(), (18, 19)),
                (SEMI, ";".to_string(), (19, 20))
            ],
            nfa.token_stream
        )
    }
}
