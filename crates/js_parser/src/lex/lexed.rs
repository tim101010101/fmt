use crate::lex::type_judgument::{
    is_blank as is_blank_from_str, is_number, is_string,
};
use crate::syntax_kind::*;
use regex::Regex;

pub type LexedToken = (SyntaxKind, String);

pub struct DFA {
    code: String,
    pub token_stream: Vec<LexedToken>,
}

impl DFA {
    pub fn new(code: String) -> Self {
        DFA {
            code,
            token_stream: Vec::new(),
        }
    }
    pub fn lexed(&mut self, skip_blank: bool) {
        if self.code.len() == 0 {
            return;
        }

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
            } else if state == 3 && is_block_end(c) {
                state = state_table[state].2;
            } else if is_block_start(c) {
                state = state_table[state].1;
            } else if is_strip(c) {
                state = state_table[state].3;
            } else if is_blank(c) {
                state = state_table[state].4;
            } else {
                state = 0;
            }

            if prev_state == 2
                || end_state.contains(&prev_state)
                    && state != prev_state
            {
                self.push_token(&text_cache, skip_blank);
                text_cache.clear();
                text_cache.push_str(&c.to_string());
            } else {
                text_cache.push_str(&c.to_string());
            }

            idx += 1;
            prev_state = state;
        }
        self.push_token(&text_cache, skip_blank);
    }
    fn push_token(&mut self, text: &str, skip_blank: bool) {
        if skip_blank && is_blank_from_str(text) {
            return;
        }

        let text = text.to_string();
        let token: (SyntaxKind, String) =
            if let Some(token_kind) =
                SyntaxKind::from_operator(&text)
            {
                (token_kind, text)
            } else if let Some(token_kind) =
                SyntaxKind::from_keyword(&text)
            {
                (token_kind, text)
            } else {
                if is_blank_from_str(&text) {
                    (WHITESPACE, text)
                } else if is_number(&text) {
                    (NUMBER, text)
                } else if is_string(&text) {
                    (STRING, text)
                } else {
                    (ID, text)
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
        ';' | ',' | '(' | ')' | '[' | ']' | '{' | '}'
        | '<' | '>' | '?' | ':' | '$' | '=' | '!' | '~'
        | '&' | '|' | '+' | '*' | '/' | '^' | '%' | '.' => {
            true
        }
        _ => false,
    }
}

fn is_strip(c: char) -> bool {
    Regex::new(r"[a-zA-Z0-9]")
        .unwrap()
        .is_match(&c.to_string())
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
        let mut dfa = DFA::new(code);
        dfa.lexed(false);
        assert_eq!(
            vec![
                (DEFINATOR, "const".to_string()),
                (WHITESPACE, "      ".to_string()),
                (ID, "a".to_string()),
                (WHITESPACE, " ".to_string()),
                (EQ, "=".to_string()),
                (WHITESPACE, " ".to_string()),
                (NUMBER, "1".to_string()),
                (PLUS, "+".to_string()),
                (WHITESPACE, " ".to_string()),
                (NUMBER, "2".to_string()),
                (SEMI, ";".to_string())
            ],
            dfa.token_stream
        )
    }

    #[test]
    fn text_nfa_2() {
        let code = "a+=2;".to_string();
        let mut dfa = DFA::new(code);
        dfa.lexed(false);
        assert_eq!(
            vec![
                (ID, "a".to_string()),
                (PLUS, "+".to_string()),
                (EQ, "=".to_string()),
                (NUMBER, "2".to_string()),
                (SEMI, ";".to_string())
            ],
            dfa.token_stream
        )
    }

    #[test]
    fn text_nfa_1() {
        let code = "const a=1+2;\
        var b=(3+2)*3;"
            .to_string();
        let mut dfa = DFA::new(code);
        dfa.lexed(true);
        assert_eq!(
            vec![
                (DEFINATOR, "const".to_string()),
                (ID, "a".to_string()),
                (EQ, "=".to_string()),
                (NUMBER, "1".to_string()),
                (PLUS, "+".to_string()),
                (NUMBER, "2".to_string()),
                (SEMI, ";".to_string()),
                (DEFINATOR, "var".to_string()),
                (ID, "b".to_string()),
                (EQ, "=".to_string()),
                (OPEN_PAREN, "(".to_string()),
                (NUMBER, "3".to_string()),
                (PLUS, "+".to_string()),
                (NUMBER, "2".to_string()),
                (CLOSE_PAREN, ")".to_string()),
                (STAR, "*".to_string()),
                (NUMBER, "3".to_string()),
                (SEMI, ";".to_string())
            ],
            dfa.token_stream
        )
    }

    #[test]
    fn test_tokenize_1() {
        let code = "const   a  = 1 += 2;".to_string();
        let mut dfa = DFA::new(code);
        dfa.lexed(true);
        assert_eq!(
            vec![
                (DEFINATOR, "const".to_string()),
                (ID, "a".to_string()),
                (EQ, "=".to_string()),
                (NUMBER, "1".to_string()),
                (PLUS, "+".to_string()),
                (EQ, "=".to_string()),
                (NUMBER, "2".to_string()),
                (SEMI, ";".to_string())
            ],
            dfa.token_stream
        )
    }

    #[test]
    fn test_block_parse() {
        let code = "const a = \"Hello World\";".to_string();
        let mut dfa = DFA::new(code);
        dfa.lexed(true);
        println!("{:?}", dfa.token_stream)
    }
}
