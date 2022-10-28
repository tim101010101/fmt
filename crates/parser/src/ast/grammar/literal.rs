use crate::ast::grammar::basic::single_token;
use crate::ast::node::NodeData;
use crate::ast::token::{Token, TokenData};
use crate::ast::tree::Element;
use crate::lex::TokenStream;
use crate::syntax_kind::*;
use crate::T;
use shared::parser_combiner::{
    either, left, one_of, pair, right, zero_or_more,
    zero_or_one, BoxedParser, Parser,
};

/// STRING_LITERAL -> DOUBLE_QUOTE [^DOUBLE_QUOTE]* DOUBLE_QUOTE
pub fn string_literal(
) -> impl Parser<'static, TokenStream, Element> {
    single_token(STRING).map(|(kind, text)| {
        TokenData::new(kind, text).into()
    })
}

/// NUMBER_LITERAL -> \d+
pub fn number_literal(
) -> impl Parser<'static, TokenStream, Element> {
    single_token(NUMBER).map(|(kind, text)| {
        TokenData::new(kind, text).into()
    })
}

/// OBJECT_LITERAL -> OPEN_BRACKET [ID COLON [STRING | NUMBER]] CLOSE_BRACKET
pub fn object_literal(
) -> impl Parser<'static, TokenStream, Element> {
    right(
        single_token(T!["{"]),
        left(attributes(), single_token(T!["}"])),
    )
    .map(|children| {
        NodeData::new(OBJECT_LITERAL, children).into()
    })
}

pub fn key_value(
) -> impl Parser<'static, TokenStream, Element> {
    pair(
        left(single_token(ID), single_token(T![":"])),
        either(single_token(NUMBER), single_token(STRING)),
    )
    .map(|(key, value)| {
        let (key_kind, key_text) = key;
        let (value_kind, value_text) = value;
        NodeData::new(
            ATTRIBUTE,
            vec![
                TokenData::new(key_kind, key_text).into(),
                TokenData::new(value_kind, value_text)
                    .into(),
            ],
        )
        .into()
    })
}

pub fn attributes(
) -> impl Parser<'static, TokenStream, Vec<Element>> {
    zero_or_more(left(
        key_value(),
        zero_or_one(single_token(T![","])),
    ))
}

/// Literal -> STRING | NUMBER | OBJECT
pub fn literal(
) -> impl Parser<'static, TokenStream, Element> {
    one_of(vec![
        BoxedParser::new(string_literal()),
        BoxedParser::new(number_literal()),
        BoxedParser::new(object_literal()),
    ])
}

/// CompareOp -> == | != | === | !==
///            | < | <= | > | >=
// pub fn compare_op(
// ) -> impl Parser<'static, TokenStream, Element> {
//     todo!()
// }

/// CalcOp -> + | - | * | /
pub fn calc_op() {
    todo!()
}

/// BitCalcOp -> & | | | ^ | ~ | >> | >>> | <<
pub fn bit_calc_op() {
    todo!()
}

/// KeywordOp -> INSTANCE_OF | IN
pub fn keyword_op() {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::ast::grammar::literal::{
        attributes, number_literal, object_literal,
        string_literal,
    };
    use crate::ast::node::NodeData;
    use crate::ast::token::TokenData;
    use crate::syntax_kind::{
        ATTRIBUTE, COMMA, ID, NUMBER, OBJECT_LITERAL,
        STRING,
    };
    use crate::T;
    use shared::parser_combiner::Parser;

    #[test]
    fn test_string_literal() {
        let input = vec![
            (STRING, "Hello World".to_string()),
            (NUMBER, "1".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![(NUMBER, "1".to_string())],
                TokenData::new(
                    STRING,
                    "Hello World".to_string()
                )
                .into()
            )),
            string_literal().parse(input)
        )
    }

    #[test]
    fn test_number_literal() {
        let input = vec![
            (NUMBER, "1".to_string()),
            (STRING, "Hello World".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![(STRING, "Hello World".to_string())],
                TokenData::new(NUMBER, "1".to_string())
                    .into()
            )),
            number_literal().parse(input)
        )
    }

    #[test]
    fn test_attribute_literal() {
        let input = vec![
            (ID, "hello".to_string()),
            (T![":"], ":".to_string()),
            (STRING, "world1".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                vec![NodeData::new(
                    ATTRIBUTE,
                    vec![
                        TokenData::new(
                            ID,
                            "hello".to_string()
                        )
                        .into(),
                        TokenData::new(
                            STRING,
                            "world1".to_string()
                        )
                        .into()
                    ]
                )
                .into()]
            )),
            attributes().parse(input)
        )
    }

    #[test]
    fn test_object_literal() {
        let input = vec![
            (T!["{"], "{".to_string()),
            (ID, "hello".to_string()),
            (T![":"], ":".to_string()),
            (STRING, "world".to_string()),
            (COMMA, ",".to_string()),
            (ID, "foo".to_string()),
            (T![":"], ":".to_string()),
            (STRING, "bar".to_string()),
            (T!["}"], "}".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                NodeData::new(
                    OBJECT_LITERAL,
                    vec![
                        NodeData::new(
                            ATTRIBUTE,
                            vec![
                                TokenData::new(
                                    ID,
                                    "hello".to_string()
                                )
                                .into(),
                                TokenData::new(
                                    STRING,
                                    "world".to_string()
                                )
                                .into(),
                            ]
                        )
                        .into(),
                        NodeData::new(
                            ATTRIBUTE,
                            vec![
                                TokenData::new(
                                    ID,
                                    "foo".to_string()
                                )
                                .into(),
                                TokenData::new(
                                    STRING,
                                    "bar".to_string()
                                )
                                .into(),
                            ]
                        )
                        .into()
                    ]
                )
                .into()
            )),
            object_literal().parse(input)
        )
    }
}
