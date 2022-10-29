#![allow(bad_style, unreachable_pub, dead_code)]

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    /* operator */
    SEMI,          // ;
    COMMA,         // ,
    DOT,           // .
    OPEN_PAREN,    // (
    CLOSE_PAREN,   // )
    OPEN_BRACE,    // {
    CLOSE_BRACE,   // }
    OPEN_BRACKET,  // [
    CLOSE_BRACKET, // ]
    QUESTION,      // ?
    COLON,         // :
    DOLLAR,        // $
    EQ,            // =
    BANG,          // !
    LT,            // <
    GT,            // >
    MINUS,         // -
    AND,           // &
    OR,            // |
    PLUS,          // +
    STAR,          // *
    SLASH,         // /
    CARET,         // ^
    PERCENT,       // %

    /* literal */
    ID,
    NUMBER,
    STRING,
    OBJECT,
    ARRAY,
    WHITE_SPACE,

    /* keyword */
    IMPORT,
    EXPORT,
    DEFAULT,
    AS,
    ASYNC,
    AWAIT,
    CONST,
    LET,
    VAR,
    IF,
    ELSE,
    FOR,
    FUNCTION,
    STATIC,
    CLASS,
    SUPER,
    YIELD,

    /* other */
    EOF,
    UNKNOW,
}

use self::TokenKind::*;

impl TokenKind {
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            IMPORT
                | EXPORT
                | DEFAULT
                | AS
                | ASYNC
                | AWAIT
                | CONST
                | LET
                | VAR
                | IF
                | ELSE
                | FOR
                | FUNCTION
                | STATIC
                | CLASS
                | SUPER
                | YIELD
        )
    }

    pub fn is_literal(self) -> bool {
        matches!(self, NUMBER | STRING | OBJECT | ARRAY)
    }

    pub fn from_keyword(str: &str) -> Option<TokenKind> {
        let token = match str {
            "as" => AS,
            "async" => ASYNC,
            "await" => AWAIT,
            "import" => IMPORT,
            "export" => EXPORT,
            "default" => DEFAULT,
            "const" => CONST,
            "let" => LET,
            "var" => VAR,
            "if" => IF,
            "else" => ELSE,
            "for" => FOR,
            "function" => FUNCTION,
            "static" => STATIC,
            "class" => CLASS,
            "super" => SUPER,
            "yield" => YIELD,
            _ => return None,
        };
        Some(token)
    }

    pub fn from_str(s: &str) -> Option<TokenKind> {
        let token = match s {
            ";" => SEMI,
            "," => COMMA,
            "(" => OPEN_PAREN,
            ")" => CLOSE_PAREN,
            "[" => OPEN_BRACKET,
            "]" => CLOSE_BRACKET,
            "{" => OPEN_BRACE,
            "}" => CLOSE_BRACE,
            "<" => LT,
            ">" => GT,
            "?" => QUESTION,
            ":" => COLON,
            "$" => DOLLAR,
            "=" => EQ,
            "!" => BANG,
            "~" => MINUS,
            "&" => AND,
            "|" => OR,
            "+" => PLUS,
            "*" => STAR,
            "/" => SLASH,
            "^" => CARET,
            "%" => PERCENT,
            _ => return None,
        };
        Some(token)
    }
}

#[macro_export]
macro_rules! T {
    [;] => { $ crate::lib::token_kind::TokenKind::SEMI };
    [,] => { $ crate::lib::token_kind::TokenKind::COMMA };
    ['('] => { $ crate::lib::token_kind::TokenKind::OPEN_PAREN };
    [')'] => { $ crate::lib::token_kind::TokenKind::CLOSE_PAREN };
    ['['] => { $ crate::lib::token_kind::TokenKind::OPEN_BRACKET };
    [']'] => { $ crate::lib::token_kind::TokenKind::CLOSE_BRACKET };
    ['{'] => { $ crate::lib::token_kind::TokenKind::OPEN_BRACE };
    ['}'] => { $ crate::lib::token_kind::TokenKind::CLOSE_BRACE };
    [<] => { $ crate::lib::token_kind::TokenKind::LT };
    [>] => { $ crate::lib::token_kind::TokenKind::GT };
    [?] => { $ crate::lib::token_kind::TokenKind::QUESTION };
    [:] => { $ crate::lib::token_kind::TokenKind::COLON };
    ['$'] => { $ crate::lib::token_kind::TokenKind::DOLLAR };
    [=] => { $ crate::lib::token_kind::TokenKind::EQ };
    [!] => { $ crate::lib::token_kind::TokenKind::BANG };
    [~] => { $ crate::lib::token_kind::TokenKind::MINUS };
    [&] => { $ crate::lib::token_kind::TokenKind::AND };
    [|] => { $ crate::lib::token_kind::TokenKind::OR };
    [+] => { $ crate::lib::token_kind::TokenKind::PLUS };
    [*] => { $ crate::lib::token_kind::TokenKind::STAR };
    [/] => { $ crate::lib::token_kind::TokenKind::SLASH };
    [^] => { $ crate::lib::token_kind::TokenKind::CARET };
    [%] => { $ crate::lib::token_kind::TokenKind::PERCENT };

    [as] => { $ crate::lib::token_kind::TokenKind::AS };
    [async] => { $ crate::lib::token_kind::TokenKind::ASYNC };
    [await] => { $ crate::lib::token_kind::TokenKind::AWAIT };
    [import] => { $ crate::lib::token_kind::TokenKind::IMPORT };
    [export] => { $ crate::lib::token_kind::TokenKind::EXPORT };
    [default] => { $ crate::lib::token_kind::TokenKind::DEFAULT };
    [const] => { $ crate::lib::token_kind::TokenKind::CONST };
    [let] => { $ crate::lib::token_kind::TokenKind::LET };
    [var] => { $ crate::lib::token_kind::TokenKind::VAR };
    [if] => { $ crate::lib::token_kind::TokenKind::IF };
    [else] => { $ crate::lib::token_kind::TokenKind::ELSE };
    [for] => { $ crate::lib::token_kind::TokenKind::FOR };
    [function] => { $ crate::lib::token_kind::TokenKind::FUNCTION };
    [static] => { $ crate::lib::token_kind::TokenKind::STATIC };
    [class] => { $ crate::lib::token_kind::TokenKind::CLASS };
    [super] => { $ crate::lib::token_kind::TokenKind::SUPER };
    [yield] => { $ crate::lib::token_kind::TokenKind::YIELD };
}

#[cfg(test)]
mod tests {
    use crate::lib::token_kind::TokenKind::{OPEN_PAREN, SEMI, STATIC};

    #[test]
    fn test_macro() {
        assert_eq!(SEMI, T![;]);
        assert_eq!(OPEN_PAREN, T!['(']);
        assert_eq!(STATIC, T![static]);
    }
}
