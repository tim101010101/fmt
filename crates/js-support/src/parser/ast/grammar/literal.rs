use crate::{
    parser::{
        ast::{grammar::*, node::literal_node, Literal::*, Node},
        TokenStream,
    },
    syntax_kind::*,
    T,
};
use shared::parser_combiner::{between, choice, either, left, seq_by, BoxedParser, Parser};

/// StringLiteral -> STRING
pub fn string_literal() -> impl Parser<'static, TokenStream, Node> {
    single_token(STRING).map(|(_, text)| {
        literal_node(StringLiteral {
            kind: STRING,
            value: text[1..text.len() - 1].to_string(),
            raw: text,
        })
    })
}

/// NumberLiteral -> NUMBER
pub fn number_literal() -> impl Parser<'static, TokenStream, Node> {
    single_token(NUMBER).map(|(_, value)| {
        literal_node(NumberLiteral {
            kind: NUMBER,
            value: value.parse().unwrap(),
            raw: value,
        })
    })
}

/// ObjectLiteral -> "{" (Attribute)* "}"
pub fn object_literal() -> impl Parser<'static, TokenStream, Node> {
    between(single_token(T!["{"]), attributes(), single_token(T!["}"])).map(|attributes| {
        literal_node(ObjectLiteral {
            kind: OBJECT,
            attributes,
        })
    })
}

/// Attribute -> KeyValue ("," KeyValue)*
pub fn attributes() -> impl Parser<'static, TokenStream, Vec<(String, Box<Node>)>> {
    seq_by(key_value(), single_token(T![","]))
}

/// KeyValue -> ID ":" (Literal | Expr)
pub fn key_value() -> impl Parser<'static, TokenStream, (String, Box<Node>)> {
    left(single_token(ID), single_token(T![":"])).and_then(|(_, key)| {
        either(literal(), expr()).map(move |value| (key.to_owned(), Box::new(value)))
    })
}

/// ArrayLiteral -> "[" (ArrayItem)* "]"
pub fn array_literal() -> impl Parser<'static, TokenStream, Node> {
    single_token(T!["["])
        .and_then(|_| left(array_item(), single_token(T!["]"])))
        .map(|items| literal_node(ArrayLiteral { kind: ARRAY, items }))
}

/// ArrayItem -> (Literal | Expr) ("," Literal | Expr) *
pub fn array_item() -> impl Parser<'static, TokenStream, Vec<Box<Node>>> {
    seq_by(
        either(literal(), expr()).map(|item| Box::new(item)),
        single_token(T![","]),
    )
}

/// Literal -> StringLiteral
///          | NumberLiteral
///          | ObjectLiteral
///          | ArrayLiteral
pub fn literal() -> impl Parser<'static, TokenStream, Node> {
    choice(vec![
        BoxedParser::new(string_literal()),
        BoxedParser::new(number_literal()),
        BoxedParser::new(object_literal()),
        BoxedParser::new(array_literal()),
    ])
}

pub fn id() -> impl Parser<'static, TokenStream, Box<Node>> {
    single_token(ID).map(|(_, name)| Box::new(literal_node(Id { kind: ID, name })))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_number() -> (Node, Node) {
        (
            literal_node(NumberLiteral {
                kind: NUMBER,
                value: 1,
                raw: "1".to_string(),
            }),
            literal_node(NumberLiteral {
                kind: NUMBER,
                value: 2,
                raw: "2".to_string(),
            }),
        )
    }
    fn get_string() -> (Node, Node) {
        (
            literal_node(StringLiteral {
                kind: STRING,
                value: "foo".to_string(),
                raw: "\"foo\"".to_string(),
            }),
            literal_node(StringLiteral {
                kind: STRING,
                value: "bar".to_string(),
                raw: "\"bar\"".to_string(),
            }),
        )
    }
    fn get_id() -> (Node, Node) {
        (
            literal_node(Id {
                kind: ID,
                name: "foo".to_string(),
            }),
            literal_node(Id {
                kind: ID,
                name: "bar".to_string(),
            }),
        )
    }

    #[test]
    fn test_string_literal() {
        let (foo, _) = get_string();

        let input = vec![(STRING, "\"foo\"".to_string())];
        assert_eq!(Ok((vec![], foo.clone())), string_literal().parse(input))
    }

    #[test]
    fn test_number_literal() {
        let (one, _) = get_number();

        let input = vec![(NUMBER, "1".to_string())];
        assert_eq!(Ok((vec![], one.clone())), number_literal().parse(input))
    }

    #[test]
    fn test_object_literal() {
        let (_, bar) = get_string();

        let input = vec![
            (OPEN_BRACE, "{".to_string()),
            (ID, "foo".to_string()),
            (COLON, ":".to_string()),
            (STRING, "\"bar\"".to_string()),
            (CLOSE_BRACE, "}".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                literal_node(ObjectLiteral {
                    kind: OBJECT,
                    attributes: vec![("foo".to_string(), Box::new(bar.clone()))]
                })
            )),
            object_literal().parse(input)
        );

        let input = vec![
            (OPEN_BRACE, "{".to_string()),
            (CLOSE_BRACE, "}".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                literal_node(ObjectLiteral {
                    kind: OBJECT,
                    attributes: vec![],
                })
            )),
            object_literal().parse(input)
        );

        let input = vec![
            (OPEN_BRACE, "{".to_string()),
            (ID, "foo".to_string()),
            (COLON, ":".to_string()),
            (OPEN_BRACE, "{".to_string()),
            (ID, "bar".to_string()),
            (COLON, ":".to_string()),
            (STRING, "\"foo\"".to_string()),
            (CLOSE_BRACE, "}".to_string()),
            (CLOSE_BRACE, "}".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                literal_node(ObjectLiteral {
                    kind: OBJECT,
                    attributes: vec![(
                        "foo".to_string(),
                        Box::new(literal_node(ObjectLiteral {
                            kind: OBJECT,
                            attributes: vec![(
                                "bar".to_string(),
                                Box::new(literal_node(StringLiteral {
                                    kind: STRING,
                                    value: "foo".to_string(),
                                    raw: "\"foo\"".to_string()
                                }))
                            )],
                        }))
                    )],
                })
            )),
            object_literal().parse(input)
        );
    }

    #[test]
    fn test_array_literal() {
        let (foo, bar) = get_string();

        let input = vec![
            (OPEN_BRACKET, "[".to_string()),
            (STRING, "\"foo\"".to_string()),
            (COMMA, ",".to_string()),
            (STRING, "\"bar\"".to_string()),
            (CLOSE_BRACKET, "]".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                literal_node(ArrayLiteral {
                    kind: ARRAY,
                    items: vec![Box::new(foo.clone()), Box::new(bar.clone())]
                })
            )),
            array_literal().parse(input)
        );

        let input = vec![
            (OPEN_BRACKET, "[".to_string()),
            (CLOSE_BRACKET, "]".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                literal_node(ArrayLiteral {
                    kind: ARRAY,
                    items: vec![]
                })
            )),
            array_literal().parse(input)
        );

        let input = vec![
            (OPEN_BRACKET, "[".to_string()),
            (STRING, "\"foo\"".to_string()),
            (COMMA, ",".to_string()),
            (OPEN_BRACKET, "[".to_string()),
            (STRING, "\"bar\"".to_string()),
            (CLOSE_BRACKET, "]".to_string()),
            (CLOSE_BRACKET, "]".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                literal_node(ArrayLiteral {
                    kind: ARRAY,
                    items: vec![
                        Box::new(foo.clone()),
                        Box::new(literal_node(ArrayLiteral {
                            kind: ARRAY,
                            items: vec![Box::new(bar.clone())]
                        }))
                    ]
                })
            )),
            array_literal().parse(input)
        )
    }
}
