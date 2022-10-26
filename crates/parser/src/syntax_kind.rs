#![allow(bad_style, unreachable_pub, dead_code)]

#[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
pub struct SyntaxKind(pub u16);

// node
pub const ROOT: SyntaxKind = SyntaxKind(0);
pub const ERROR: SyntaxKind = SyntaxKind(1);
pub const SINGLE: SyntaxKind = SyntaxKind(2);
pub const LIST: SyntaxKind = SyntaxKind(3);
pub const BLOCK: SyntaxKind = SyntaxKind(4);

pub const UNARY_EXPR: SyntaxKind = SyntaxKind(100);
pub const BINARY_EXPR: SyntaxKind = SyntaxKind(101);

// token
pub const IDENT: SyntaxKind = SyntaxKind(10000);
pub const WHITESPACE: SyntaxKind = SyntaxKind(10001);

pub const DEFINATOR: SyntaxKind = SyntaxKind(10002);
pub const ID: SyntaxKind = SyntaxKind(10003);
pub const NUMBER: SyntaxKind = SyntaxKind(10004);
pub const STRING: SyntaxKind = SyntaxKind(10005);

pub const OPEN_PAREN: SyntaxKind = SyntaxKind(10006);
pub const CLOSE_PAREN: SyntaxKind = SyntaxKind(10007);
pub const OPEN_BRACKET: SyntaxKind = SyntaxKind(10008);
pub const CLOSE_BRACKET: SyntaxKind = SyntaxKind(10009);
pub const OPEN_BRACE: SyntaxKind = SyntaxKind(10010);
pub const CLOSE_BRACE: SyntaxKind = SyntaxKind(10011);
pub const PLUS: SyntaxKind = SyntaxKind(10012);
pub const MINUS: SyntaxKind = SyntaxKind(10013);
pub const STAR: SyntaxKind = SyntaxKind(10014);
pub const SLASH: SyntaxKind = SyntaxKind(10015);
pub const EQ: SyntaxKind = SyntaxKind(10016);
pub const SEMI: SyntaxKind = SyntaxKind(10017);
pub const COMMA: SyntaxKind = SyntaxKind(10018);

pub const FUNCTION_KW: SyntaxKind = SyntaxKind(10050);

// other
pub const UNKNOW: SyntaxKind = SyntaxKind(65535);

impl SyntaxKind {
    pub fn from_keyword(str: &str) -> Option<SyntaxKind> {
        let kw = match str {
            "function" => FUNCTION_KW,
            "const" => DEFINATOR,
            "let" => DEFINATOR,
            "var" => DEFINATOR,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_operator(str: &str) -> Option<SyntaxKind> {
        let op = match str {
            "=" => EQ,
            ";" => SEMI,
            "(" => OPEN_PAREN,
            ")" => CLOSE_PAREN,
            "[" => OPEN_BRACKET,
            "]" => CLOSE_BRACKET,
            "{" => OPEN_BRACE,
            "}" => CLOSE_BRACE,
            "+" => PLUS,
            "*" => STAR,
            "," => COMMA,
            _ => return None,
        };
        Some(op)
    }
    pub fn to_str(&self) -> &str {
        match self {
            &ROOT => "ROOT",
            &SINGLE => "SINGLE",
            &LIST => "LIST",
            &BLOCK => "BLOCK",

            &WHITESPACE => "WHITESPACE",

            &DEFINATOR => "DEFINATOR",
            &ID => "ID",
            &NUMBER => "NUMBER",
            &STRING => "STRING",

            &OPEN_PAREN => "OPEN_PAREN",
            &CLOSE_PAREN => "CLOSE_PAREN",
            &OPEN_BRACKET => "OPEN_BRACKET",
            &CLOSE_BRACKET => "CLOSE_BRACKET",
            &OPEN_BRACE => "OPEN_BRACE",
            &CLOSE_BRACE => "CLOSE_BRACE",
            &PLUS => "PLUS",
            &MINUS => "MINUS",
            &STAR => "STAR",
            &SLASH => "SLASH",
            &EQ => "EQ",
            &SEMI => "SEMI",
            &COMMA => "COMMA",

            &FUNCTION_KW => "FUNCTION_KW",

            _ => return "Unknow",
        }
    }
}

#[macro_export]
macro_rules! T {
    [";"] => { $ crate::syntax_kind::SEMI };
    [","] => { $ crate::syntax_kind::COMMA };
    ["("] => { $ crate::syntax_kind::OPEN_PAREN };

    ["function"] => {$crate::syntax_kind::FUNCTION_KW};

}

#[cfg(test)]
mod tests {
    use crate::syntax_kind::{FUNCTION_KW, SEMI};

    #[test]
    fn test_macro() {
        assert_eq!(T![";"], SEMI);
        assert_eq!(T!["function"], FUNCTION_KW);
    }
}
