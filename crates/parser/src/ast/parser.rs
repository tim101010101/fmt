use crate::ast::builder::Builder;
use crate::ast::node::Node;
use crate::lex::{Token, TokenStream};
use crate::syntax_kind::*;

enum ParseResult {
    Ok,
    Eof,
    CloseList,
    CloseBlock,
}

pub struct Parser {
    token_stream: TokenStream,
    builder: Builder,
    errors: Vec<String>,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Parser {
            token_stream: TokenStream::new(tokens),
            builder: Builder::new(),
            errors: Vec::new(),
        }
    }
    fn current(&self) -> Option<SyntaxKind> {
        self.token_stream.look_ahead(0)
    }
    fn bump(&mut self) {
        let (kind, value) = self.token_stream.next().unwrap();
        self.builder.token(kind, value.as_str());
    }
    fn skip_whitespace(&mut self) {
        while self.current() == Some(WHITESPACE) {
            self.bump();
        }
    }
    fn process_error(&mut self, msg: &str) {
        self.builder.start_node(ERROR);
        self.errors.push(format!("[Error]: {}", msg));
        self.builder.finish_node();
    }

    pub(crate) fn parse(&mut self) -> Option<Node> {
        self.builder.start_node(ROOT);

        loop {
            match self.parse_something() {
                ParseResult::Ok => (),
                ParseResult::Eof => break,
                ParseResult::CloseList => {
                    self.process_error("unmatched `)`");
                }
                ParseResult::CloseBlock => {
                    self.process_error("unmatched `}`");
                }
            }
        }
        self.skip_whitespace();
        self.builder.finish_node();

        self.builder.syntax()
    }
    fn parse_something(&mut self) -> ParseResult {
        self.skip_whitespace();

        let t = match self.current() {
            None => return ParseResult::Eof,
            Some(CLOSE_PAREN) => return ParseResult::CloseList,
            Some(CLOSE_BRACE) => return ParseResult::CloseBlock,
            Some(token) => token,
        };
        self.builder.start_node(BLOCK);
        match t {
            OPEN_PAREN => self.parse_list(),
            OPEN_BRACE => self.parse_block(),
            ERROR => self.bump(),
            _ => self.bump(),
        };
        self.builder.finish_node();

        ParseResult::Ok
    }
    fn parse_list(&mut self) {
        self.builder.start_node(LIST);
        self.bump();
        loop {
            match self.parse_something() {
                ParseResult::Ok => (),
                ParseResult::Eof => {
                    self.process_error("expected `)`");
                    break;
                }
                ParseResult::CloseList => {
                    self.bump();
                    break;
                }
                ParseResult::CloseBlock => {
                    self.bump();
                    break;
                }
            }
        }
        self.builder.finish_node();
    }
    fn parse_block(&mut self) {
        self.builder.start_node(BLOCK);
        self.bump();
        loop {
            match self.parse_something() {
                ParseResult::Ok => (),
                ParseResult::Eof => {
                    self.process_error("expected `}`");
                    break;
                }
                ParseResult::CloseList => {
                    self.bump();
                    break;
                }
                ParseResult::CloseBlock => {
                    self.bump();
                    break;
                }
            }
        }
        self.builder.finish_node();
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::node::Node;
    use crate::ast::parser::Parser;
    use crate::lex::Token;
    use crate::syntax_kind::*;

    fn get_line_token_stream() -> Vec<Token> {
        vec![
            (DEFINATOR, "const".to_string()),
            (WHITESPACE, " ".to_string()),
            (ID, "foo".to_string()),
            (WHITESPACE, " ".to_string()),
            (EQ, "=".to_string()),
            (WHITESPACE, " ".to_string()),
            (NUMBER, "1".to_string()),
            (SEMI, ";".to_string()),
        ]
    }
    fn get_block_token_stream() -> Vec<Token> {
        vec![
            (OPEN_BRACE, "{".to_string()),
            (DEFINATOR, "const".to_string()),
            (WHITESPACE, " ".to_string()),
            (ID, "foo".to_string()),
            (WHITESPACE, " ".to_string()),
            (EQ, "=".to_string()),
            (WHITESPACE, " ".to_string()),
            (NUMBER, "1".to_string()),
            (SEMI, ";".to_string()),
            (CLOSE_BRACE, "}".to_string()),
        ]
    }
    fn get_function_declaration_token_stream() -> Vec<Token> {
        vec![
            (FUNCTION_KW, "function".to_string()),
            (WHITESPACE, " ".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (CLOSE_PAREN, ")".to_string()),
            (WHITESPACE, " ".to_string()),
            (OPEN_BRACE, "{".to_string()),
            (DEFINATOR, "const".to_string()),
            (WHITESPACE, " ".to_string()),
            (ID, "foo".to_string()),
            (WHITESPACE, " ".to_string()),
            (EQ, "=".to_string()),
            (WHITESPACE, " ".to_string()),
            (NUMBER, "1".to_string()),
            (SEMI, ";".to_string()),
            (CLOSE_BRACE, "}".to_string()),
        ]
    }
    fn parser_stub(tokens: Vec<Token>) -> Node {
        Parser::new(tokens).parse().unwrap()
    }

    #[test]
    fn test_parse_line() {
        println!("{}", parser_stub(get_line_token_stream()));
    }

    #[test]
    fn test_parse_block() {
        println!("{}", parser_stub(get_block_token_stream()));
    }

    #[test]
    fn test_parse_function_declaration() {
        // println!("{:?}", parser_stub(get_function_declaration_token_stream()));
        println!("{}", parser_stub(get_function_declaration_token_stream()));
    }

    fn tttt() {
        // NodeData {
        //     kind: SyntaxKind(0),
        //     len: 16,
        //     children: [Node(NodeData {
        //         kind: SyntaxKind(4),
        //         len: 16,
        //         children: [Node(NodeData {
        //             kind: SyntaxKind(4),
        //             len: 16,
        //             children: [
        //                 Token(TokenData {
        //                     kind: SyntaxKind(10010),
        //                     text: "{",
        //                 }),
        //                 Node(NodeData {
        //                     kind: SyntaxKind(4),
        //                     len: 5,
        //                     children: [Token(TokenData {
        //                         kind: SyntaxKind(10002),
        //                         text: "const",
        //                     })],
        //                 }),
        //                 Token(TokenData {
        //                     kind: SyntaxKind(10001),
        //                     text: " ",
        //                 }),
        //                 Node(NodeData {
        //                     kind: SyntaxKind(4),
        //                     len: 3,
        //                     children: [Token(TokenData {
        //                         kind: SyntaxKind(10003),
        //                         text: "foo",
        //                     })],
        //                 }),
        //                 Token(TokenData {
        //                     kind: SyntaxKind(10001),
        //                     text: " ",
        //                 }),
        //                 Node(NodeData {
        //                     kind: SyntaxKind(4),
        //                     len: 1,
        //                     children: [Token(TokenData {
        //                         kind: SyntaxKind(10016),
        //                         text: "=",
        //                     })],
        //                 }),
        //                 Token(TokenData {
        //                     kind: SyntaxKind(10001),
        //                     text: " ",
        //                 }),
        //                 Node(NodeData {
        //                     kind: SyntaxKind(4),
        //                     len: 1,
        //                     children: [Token(TokenData {
        //                         kind: SyntaxKind(10004),
        //                         text: "1",
        //                     })],
        //                 }),
        //                 Node(NodeData {
        //                     kind: SyntaxKind(4),
        //                     len: 1,
        //                     children: [Token(TokenData {
        //                         kind: SyntaxKind(10017),
        //                         text: ";",
        //                     })],
        //                 }),
        //                 Token(TokenData {
        //                     kind: SyntaxKind(10011),
        //                     text: "}",
        //                 }),
        //             ],
        //         })],
        //     })],
        // }
    }
}
