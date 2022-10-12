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
