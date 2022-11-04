use crate::ast::grammar::basic::single_token;
use crate::ast::grammar::expr;
use crate::ast::{
    ArrayLiteral, Id, Node, NumberLiteral, ObjectLiteral,
    StringLiteral,
};
use crate::lex::TokenStream;
use crate::syntax_kind::{
    ARRAY, ID, NUMBER, OBJECT, STRING,
};
use crate::T;
use shared::parser_combiner::{
    between, choice, either, left, seq_by, BoxedParser,
    Parser,
};

/// StringLiteral -> STRING
pub fn string_literal(
) -> impl Parser<'static, TokenStream, Node> {
    single_token(STRING).map(|(_, text)| StringLiteral {
        kind: STRING,
        value: text[1..text.len() - 1].to_string(),
        raw: text,
    })
}

/// NumberLiteral -> NUMBER
pub fn number_literal(
) -> impl Parser<'static, TokenStream, Node> {
    single_token(NUMBER).map(|(_, value)| NumberLiteral {
        kind: NUMBER,
        value: value.parse().unwrap(),
        raw: value,
    })
}

/// ObjectLiteral -> "{" (Attribute)* "}"
pub fn object_literal(
) -> impl Parser<'static, TokenStream, Node> {
    between(
        single_token(T!["{"]),
        attributes(),
        single_token(T!["}"]),
    )
    .map(|attributes| ObjectLiteral {
        kind: OBJECT,
        attributes,
    })
}

/// Attribute -> KeyValue ("," KeyValue)*
pub fn attributes(
) -> impl Parser<'static, TokenStream, Vec<(String, Box<Node>)>>
{
    seq_by(key_value(), single_token(T![","]))
}

/// KeyValue -> ID ":" (Literal | Expr)
pub fn key_value(
) -> impl Parser<'static, TokenStream, (String, Box<Node>)>
{
    left(single_token(ID), single_token(T![":"])).and_then(
        |(_, key)| {
            either(literal(), expr()).map(move |value| {
                (key.to_owned(), Box::new(value))
            })
        },
    )
}

/// ArrayLiteral -> "[" (ArrayItem)* "]"
pub fn array_literal(
) -> impl Parser<'static, TokenStream, Node> {
    single_token(T!["["])
        .and_then(|_| {
            left(array_item(), single_token(T!["]"]))
        })
        .map(|items| ArrayLiteral { kind: ARRAY, items })
}

/// ArrayItem -> (Literal | Expr) ("," Literal | Expr) *
pub fn array_item(
) -> impl Parser<'static, TokenStream, Vec<Box<Node>>> {
    seq_by(
        either(literal(), expr())
            .map(|item| Box::new(item)),
        single_token(T![","]),
    )
}

/// Literal -> StringLiteral
///          | NumberLiteral
///          | ObjectLiteral
///          | ArrayLiteral
pub fn literal() -> impl Parser<'static, TokenStream, Node>
{
    choice(vec![
        BoxedParser::new(string_literal()),
        BoxedParser::new(number_literal()),
        BoxedParser::new(object_literal()),
        BoxedParser::new(array_literal()),
    ])
}

pub fn id() -> impl Parser<'static, TokenStream, Box<Node>>
{
    single_token(ID)
        .map(|(_, name)| Box::new(Id { kind: ID, name }))
}

#[cfg(test)]
mod tests {
    use crate::ast::grammar::literal::string_literal;
    use crate::ast::grammar::{
        array_literal, number_literal, object_literal,
    };
    use crate::ast::{
        ArrayLiteral, NumberLiteral, ObjectLiteral,
        StringLiteral,
    };
    use crate::syntax_kind::{
        ARRAY, CLOSE_BRACE, CLOSE_BRACKET, COLON, COMMA,
        ID, NUMBER, OBJECT, OPEN_BRACE, OPEN_BRACKET,
        STRING,
    };
    use shared::parser_combiner::Parser;

    #[test]
    fn test_string_literal() {
        let input = vec![(STRING, "\"hello\"".to_string())];
        assert_eq!(
            Ok((
                vec![],
                StringLiteral {
                    kind: STRING,
                    value: "hello".to_string(),
                    raw: "\"hello\"".to_string()
                }
            )),
            string_literal().parse(input)
        )
    }

    #[test]
    fn test_number_literal() {
        let input = vec![(NUMBER, "1".to_string())];
        assert_eq!(
            Ok((
                vec![],
                NumberLiteral {
                    kind: NUMBER,
                    value: 1,
                    raw: "1".to_string()
                }
            )),
            number_literal().parse(input)
        )
    }

    #[test]
    fn test_object_literal() {
        let input = vec![
            (OPEN_BRACE, "{".to_string()),
            (ID, "hello".to_string()),
            (COLON, ":".to_string()),
            (STRING, "\"world\"".to_string()),
            (CLOSE_BRACE, "}".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                ObjectLiteral {
                    kind: OBJECT,
                    attributes: vec![(
                        "hello".to_string(),
                        Box::new(StringLiteral {
                            kind: STRING,
                            value: "world".to_string(),
                            raw: "\"world\"".to_string(),
                        })
                    )]
                }
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
                ObjectLiteral {
                    kind: OBJECT,
                    attributes: vec![],
                }
            )),
            object_literal().parse(input)
        );

        let input = vec![
            (OPEN_BRACE, "{".to_string()),
            (ID, "hello".to_string()),
            (COLON, ":".to_string()),
            (OPEN_BRACE, "{".to_string()),
            (ID, "hello".to_string()),
            (COLON, ":".to_string()),
            (STRING, "\"world\"".to_string()),
            (CLOSE_BRACE, "}".to_string()),
            (CLOSE_BRACE, "}".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                ObjectLiteral {
                    kind: OBJECT,
                    attributes: vec![(
                        "hello".to_string(),
                        Box::new(ObjectLiteral {
                            kind: OBJECT,
                            attributes: vec![(
                                "hello".to_string(),
                                Box::new(StringLiteral {
                                    kind: STRING,
                                    value: "world"
                                        .to_string(),
                                    raw: "\"world\""
                                        .to_string(),
                                })
                            )],
                        })
                    )],
                }
            )),
            object_literal().parse(input)
        );
    }

    #[test]
    fn test_array_literal() {
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
                ArrayLiteral {
                    kind: ARRAY,
                    items: vec![
                        Box::new(StringLiteral {
                            kind: STRING,
                            value: "foo".to_string(),
                            raw: "\"foo\"".to_string()
                        }),
                        Box::new(StringLiteral {
                            kind: STRING,
                            value: "bar".to_string(),
                            raw: "\"bar\"".to_string()
                        })
                    ]
                }
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
                ArrayLiteral {
                    kind: ARRAY,
                    items: vec![]
                }
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
                ArrayLiteral {
                    kind: ARRAY,
                    items: vec![
                        Box::new(StringLiteral {
                            kind: STRING,
                            value: "foo".to_string(),
                            raw: "\"foo\"".to_string()
                        }),
                        Box::new(ArrayLiteral {
                            kind: ARRAY,
                            items: vec![Box::new(
                                StringLiteral {
                                    kind: STRING,
                                    value: "bar"
                                        .to_string(),
                                    raw: "\"bar\""
                                        .to_string()
                                }
                            )]
                        })
                    ]
                }
            )),
            array_literal().parse(input)
        )
    }
}
