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
            _ => return None,
        };
        Some(op)
    }
}

// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
// pub enum NodeKind {
//     ROOT,
//     DECLARATION,
// }
//
// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
// pub enum TokenKind {
//     /* operator */
//     SEMI,          // ;
//     COMMA,         // ,
//     DOT,           // .
//     OPEN_PAREN,    // (
//     CLOSE_PAREN,   // )
//     OPEN_BRACE,    // {
//     CLOSE_BRACE,   // }
//     OPEN_BRACKET,  // [
//     CLOSE_BRACKET, // ]
//     QUESTION,      // ?
//     COLON,         // :
//     DOLLAR,        // $
//     EQ,            // =
//     BANG,          // !
//     LT,            // <
//     GT,            // >
//     MINUS,         // -
//     AND,           // &
//     OR,            // |
//     PLUS,          // +
//     STAR,          // *
//     SLASH,         // /
//     CARET,         // ^
//     PERCENT,       // %
//
//     /* literal */
//     ID,
//     NUMBER,
//     STRING,
//     OBJECT,
//     ARRAY,
//     WHITE_SPACE,
//
//     /* keyword */
//     IMPORT,
//     EXPORT,
//     DEFAULT,
//     AS,
//     ASYNC,
//     AWAIT,
//     CONST,
//     LET,
//     VAR,
//     IF,
//     ELSE,
//     FOR,
//     FUNCTION,
//     STATIC,
//     CLASS,
//     SUPER,
//     YIELD,
//
//     /* other */
//     EOF,
//     UNKNOW,
// }
//
// use self::TokenKind::*;
//
// impl TokenKind {
//     pub fn is_keyword(self) -> bool {
//         matches!(
//             self,
//             IMPORT
//                 | EXPORT
//                 | DEFAULT
//                 | AS
//                 | ASYNC
//                 | AWAIT
//                 | CONST
//                 | LET
//                 | VAR
//                 | IF
//                 | ELSE
//                 | FOR
//                 | FUNCTION
//                 | STATIC
//                 | CLASS
//                 | SUPER
//                 | YIELD
//         )
//     }
//
//     pub fn is_literal(self) -> bool {
//         matches!(self, NUMBER | STRING | OBJECT | ARRAY)
//     }
//
//     pub fn from_keyword(str: &str) -> Option<TokenKind> {
//         let token = match str {
//             "as" => AS,
//             "async" => ASYNC,
//             "await" => AWAIT,
//             "import" => IMPORT,
//             "export" => EXPORT,
//             "default" => DEFAULT,
//             "const" => CONST,
//             "let" => LET,
//             "var" => VAR,
//             "if" => IF,
//             "else" => ELSE,
//             "for" => FOR,
//             "function" => FUNCTION,
//             "static" => STATIC,
//             "class" => CLASS,
//             "super" => SUPER,
//             "yield" => YIELD,
//             _ => return None,
//         };
//         Some(token)
//     }
//
//     pub fn from_str(s: &str) -> Option<TokenKind> {
//         let token = match s {
//             ";" => SEMI,
//             "," => COMMA,
//             "(" => OPEN_PAREN,
//             ")" => CLOSE_PAREN,
//             "[" => OPEN_BRACKET,
//             "]" => CLOSE_BRACKET,
//             "{" => OPEN_BRACE,
//             "}" => CLOSE_BRACE,
//             "<" => LT,
//             ">" => GT,
//             "?" => QUESTION,
//             ":" => COLON,
//             "$" => DOLLAR,
//             "=" => EQ,
//             "!" => BANG,
//             "~" => MINUS,
//             "&" => AND,
//             "|" => OR,
//             "+" => PLUS,
//             "*" => STAR,
//             "/" => SLASH,
//             "^" => CARET,
//             "%" => PERCENT,
//             _ => return None,
//         };
//         Some(token)
//     }
// }
//
// #[macro_export]
// macro_rules! T {
//     [;] => { $ crate::lex::token_kind::TokenKind::SEMI };
//     [,] => { $ crate::lex::token_kind::TokenKind::COMMA };
//     ['('] => { $ crate::lex::token_kind::TokenKind::OPEN_PAREN };
//     [')'] => { $ crate::lex::token_kind::TokenKind::CLOSE_PAREN };
//     ['['] => { $ crate::lex::token_kind::TokenKind::OPEN_BRACKET };
//     [']'] => { $ crate::lex::token_kind::TokenKind::CLOSE_BRACKET };
//     ['{'] => { $ crate::lex::token_kind::TokenKind::OPEN_BRACE };
//     ['}'] => { $ crate::lex::token_kind::TokenKind::CLOSE_BRACE };
//     [<] => { $ crate::lex::token_kind::TokenKind::LT };
//     [>] => { $ crate::lex::token_kind::TokenKind::GT };
//     [?] => { $ crate::lex::token_kind::TokenKind::QUESTION };
//     [:] => { $ crate::lex::token_kind::TokenKind::COLON };
//     ['$'] => { $ crate::lex::token_kind::TokenKind::DOLLAR };
//     [=] => { $ crate::lex::token_kind::TokenKind::EQ };
//     [!] => { $ crate::lex::token_kind::TokenKind::BANG };
//     [~] => { $ crate::lex::token_kind::TokenKind::MINUS };
//     [&] => { $ crate::lex::token_kind::TokenKind::AND };
//     [|] => { $ crate::lex::token_kind::TokenKind::OR };
//     [+] => { $ crate::lex::token_kind::TokenKind::PLUS };
//     [*] => { $ crate::lex::token_kind::TokenKind::STAR };
//     [/] => { $ crate::lex::token_kind::TokenKind::SLASH };
//     [^] => { $ crate::lex::token_kind::TokenKind::CARET };
//     [%] => { $ crate::lex::token_kind::TokenKind::PERCENT };
//
//     [as] => { $ crate::lex::token_kind::TokenKind::AS };
//     [async] => { $ crate::lex::token_kind::TokenKind::ASYNC };
//     [await] => { $ crate::lex::token_kind::TokenKind::AWAIT };
//     [import] => { $ crate::lex::token_kind::TokenKind::IMPORT };
//     [export] => { $ crate::lex::token_kind::TokenKind::EXPORT };
//     [default] => { $ crate::lex::token_kind::TokenKind::DEFAULT };
//     [const] => { $ crate::lex::token_kind::TokenKind::CONST };
//     [let] => { $ crate::lex::token_kind::TokenKind::LET };
//     [var] => { $ crate::lex::token_kind::TokenKind::VAR };
//     [if] => { $ crate::lex::token_kind::TokenKind::IF };
//     [else] => { $ crate::lex::token_kind::TokenKind::ELSE };
//     [for] => { $ crate::lex::token_kind::TokenKind::FOR };
//     [function] => { $ crate::lex::token_kind::TokenKind::FUNCTION };
//     [static] => { $ crate::lex::token_kind::TokenKind::STATIC };
//     [class] => { $ crate::lex::token_kind::TokenKind::CLASS };
//     [super] => { $ crate::lex::token_kind::TokenKind::SUPER };
//     [yield] => { $ crate::lex::token_kind::TokenKind::YIELD };
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::lex::token_kind::TokenKind::{OPEN_PAREN, SEMI, STATIC};
//
//     #[test]
//     fn test_macro() {
//         assert_eq!(SEMI, T![;]);
//         assert_eq!(OPEN_PAREN, T!['(']);
//         assert_eq!(STATIC, T![static]);
//     }
// }
