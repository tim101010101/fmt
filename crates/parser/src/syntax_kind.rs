#![allow(bad_style, unreachable_pub, dead_code)]

#[derive(
    Debug, PartialEq, PartialOrd, Eq, Hash, Copy, Clone,
)]
pub struct SyntaxKind(pub u16);

// node
pub const ROOT: SyntaxKind = SyntaxKind(0);
pub const ERROR: SyntaxKind = SyntaxKind(1);

// expression
pub const UNARY_EXPR: SyntaxKind = SyntaxKind(100);
pub const BINARY_EXPR: SyntaxKind = SyntaxKind(101);
pub const TERNARY_EXPR: SyntaxKind = SyntaxKind(102);
pub const SEQUENCE_EXPR: SyntaxKind = SyntaxKind(103);
pub const VALUE_ACCESS_EXPR: SyntaxKind = SyntaxKind(104);
pub const FUNCTION_CALL_EXPR: SyntaxKind = SyntaxKind(105);
pub const ASSIGNMENT_EXPR: SyntaxKind = SyntaxKind(106);
pub const VARIABLE_DECLA: SyntaxKind = SyntaxKind(107);
pub const FUNCTION_DECLA: SyntaxKind = SyntaxKind(108);

// token
pub const IDENT: SyntaxKind = SyntaxKind(10000);
pub const WHITESPACE: SyntaxKind = SyntaxKind(10001);

pub const DEFINATOR: SyntaxKind = SyntaxKind(10002);
pub const ID: SyntaxKind = SyntaxKind(10003);
pub const NUMBER: SyntaxKind = SyntaxKind(10004);
pub const STRING: SyntaxKind = SyntaxKind(10005);
pub const OBJECT: SyntaxKind = SyntaxKind(9999);
pub const ARRAY: SyntaxKind = SyntaxKind(9999);

// operator
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
pub const DOUBLE_QUOTE: SyntaxKind = SyntaxKind(10019);
pub const SINGLE_QUOTE: SyntaxKind = SyntaxKind(10020);
pub const COLON: SyntaxKind = SyntaxKind(10021);
pub const DOT: SyntaxKind = SyntaxKind(10022);
pub const LT: SyntaxKind = SyntaxKind(10023);
pub const GT: SyntaxKind = SyntaxKind(10024);
pub const BANG: SyntaxKind = SyntaxKind(10025);
pub const AMP: SyntaxKind = SyntaxKind(10026);
pub const PIPE: SyntaxKind = SyntaxKind(10027);
pub const CARET: SyntaxKind = SyntaxKind(10028);
pub const TILDE: SyntaxKind = SyntaxKind(10029);
pub const QUESTION: SyntaxKind = SyntaxKind(10030);

// keyword
pub const FUNCTION_KW: SyntaxKind = SyntaxKind(10050);
pub const IF_KW: SyntaxKind = SyntaxKind(10051);
pub const ELSE_KW: SyntaxKind = SyntaxKind(10052);
pub const FOR_WK: SyntaxKind = SyntaxKind(10053);
pub const while_WK: SyntaxKind = SyntaxKind(10054);
pub const SWITCH_KW: SyntaxKind = SyntaxKind(10055);
pub const CASE_KW: SyntaxKind = SyntaxKind(10056);
pub const DEFAULT_KW: SyntaxKind = SyntaxKind(10057);
pub const TYPE_OF_KW: SyntaxKind = SyntaxKind(10058);
pub const DELETE_KW: SyntaxKind = SyntaxKind(10059);
pub const INSTANCE_OF_KW: SyntaxKind = SyntaxKind(10060);
pub const IN_KW: SyntaxKind = SyntaxKind(10061);

// composite operator
pub const EQEQ: SyntaxKind = SyntaxKind(10100);
pub const EQEQEQ: SyntaxKind = SyntaxKind(10101);
pub const LTEQ: SyntaxKind = SyntaxKind(10102);
pub const GTEQ: SyntaxKind = SyntaxKind(10103);
pub const BANGEQ: SyntaxKind = SyntaxKind(10104);
pub const BANGEQEQ: SyntaxKind = SyntaxKind(10105);
pub const PLUSPLUS: SyntaxKind = SyntaxKind(10106);
pub const MINUSMINUS: SyntaxKind = SyntaxKind(10107);
pub const LTLT: SyntaxKind = SyntaxKind(10108);
pub const GTGT: SyntaxKind = SyntaxKind(10109);
pub const GTGTGT: SyntaxKind = SyntaxKind(10110);

// other
pub const EMPTY: SyntaxKind = SyntaxKind(65534);
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
            "\"" => DOUBLE_QUOTE,
            "'" => SINGLE_QUOTE,
            ":" => COLON,
            "." => DOT,
            "<" => LT,
            ">" => GT,
            _ => return None,
        };
        Some(op)
    }
    pub fn to_str(&self) -> &str {
        match self {
            &ROOT => "ROOT",

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
    [";"] => { $crate::syntax_kind::SEMI };
    [":"] => { $crate::syntax_kind::COLON };
    [","] => { $crate::syntax_kind::COMMA };
    ["="] => { $crate::syntax_kind::EQ };
    ["("] => { $crate::syntax_kind::OPEN_PAREN };
    [")"] => { $crate::syntax_kind::CLOSE_PAREN };
    ["["] => { $crate::syntax_kind::OPEN_BRACKET };
    ["]"] => { $crate::syntax_kind::CLOSE_BRACKET };
    ["{"] => { $crate::syntax_kind::OPEN_BRACE };
    ["}"] => { $crate::syntax_kind::CLOSE_BRACE };
    ["."] => { $crate::syntax_kind::DOT };
    ["<"] => { $crate::syntax_kind::LT };
    [">"] => { $crate::syntax_kind::GT };
    ["!"] => { $crate::syntax_kind::BANG };
    ["-"] => { $crate::syntax_kind::MINUS };
    ["+"] => { $crate::syntax_kind::PLUS };
    ["*"] => { $crate::syntax_kind::STAR };
    ["/"] => { $crate::syntax_kind::SLASH };
    ["&"] => { $crate::syntax_kind::AMP };
    ["|"] => { $crate::syntax_kind::PIPE };
    ["^"] => { $crate::syntax_kind::CARET };
    ["~"] => { $crate::syntax_kind::TILDE };
    ["?"] => { $crate::syntax_kind::QUESTION };

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
