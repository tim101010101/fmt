use crate::{
    parser::{
        ast::{Node, Node::*},
        LexedToken, TokenStream,
    },
    syntax_kind::*,
    T,
};
use shared::parser_combiner::{
    atom, between, choice, either, empty, judge, map,
    seq_by, zero_or_more, BoxedParser, Parser,
};

pub fn empty_node(
) -> impl Parser<'static, TokenStream, Node> {
    empty().map(|_| Empty)
}

pub fn empty_token(
) -> impl Parser<'static, TokenStream, SyntaxKind> {
    empty().map(|_| EMPTY)
}

pub fn single_token(
    expect: SyntaxKind,
) -> impl Parser<'static, TokenStream, LexedToken> {
    judge(
        atom::<TokenStream, LexedToken>(),
        move |(kind, _)| *kind == expect,
    )
}

pub fn white_space() -> impl Parser<'static, TokenStream, ()>
{
    map(single_token(WHITESPACE), |_| ())
}

pub fn eq_eq(
) -> impl Parser<'static, TokenStream, LexedToken> {
    single_token(T!["="])
        .and_then(|_| single_token(T!["="]))
        .map(|_| (EQEQ, "==".to_string()))
}

pub fn eq_eq_eq(
) -> impl Parser<'static, TokenStream, LexedToken> {
    eq_eq()
        .and_then(|_| single_token(T!["="]))
        .map(|_| (EQEQEQ, "===".to_string()))
}

pub fn lt_eq(
) -> impl Parser<'static, TokenStream, LexedToken> {
    single_token(T!["<"])
        .and_then(|_| single_token(T!["="]))
        .map(|_| (LTEQ, "<=".to_string()))
}

pub fn gt_eq(
) -> impl Parser<'static, TokenStream, LexedToken> {
    single_token(T![">"])
        .and_then(|_| single_token(T!["="]))
        .map(|_| (GTEQ, ">=".to_string()))
}

pub fn bang_eq(
) -> impl Parser<'static, TokenStream, LexedToken> {
    single_token(T!["!"])
        .and_then(|_| single_token(T!["="]))
        .map(|_| (BANGEQ, "!=".to_string()))
}

pub fn bang_eq_eq(
) -> impl Parser<'static, TokenStream, LexedToken> {
    single_token(T!["!"])
        .and_then(|_| single_token(T!["="]))
        .and_then(|_| single_token(T!["="]))
        .map(|_| (BANGEQEQ, "!==".to_string()))
}

pub fn plus_plus(
) -> impl Parser<'static, TokenStream, LexedToken> {
    single_token(T!["+"])
        .and_then(|_| single_token(T!["+"]))
        .map(|_| (PLUSPLUS, "++".to_string()))
}

pub fn minus_minus(
) -> impl Parser<'static, TokenStream, LexedToken> {
    single_token(T!["-"])
        .and_then(|_| single_token(T!["-"]))
        .map(|_| (MINUSMINUS, "--".to_string()))
}

pub fn lt_lt(
) -> impl Parser<'static, TokenStream, LexedToken> {
    single_token(T!["<"])
        .and_then(|_| single_token(T!["<"]))
        .map(|_| (LTLT, "<<".to_string()))
}

pub fn gt_gt(
) -> impl Parser<'static, TokenStream, LexedToken> {
    single_token(T![">"])
        .and_then(|_| single_token(T![">"]))
        .map(|_| (GTGT, ">>".to_string()))
}

pub fn gt_gt_gt(
) -> impl Parser<'static, TokenStream, LexedToken> {
    single_token(T![">"])
        .and_then(|_| single_token(T![">"]))
        .and_then(|_| single_token(T![">"]))
        .map(|_| (GTGTGT, ">>>".to_string()))
}

pub fn comparison_op(
) -> impl Parser<'static, TokenStream, LexedToken> {
    choice(vec![
        BoxedParser::new(eq_eq()),
        BoxedParser::new(bang_eq()),
        BoxedParser::new(eq_eq_eq()),
        BoxedParser::new(bang_eq_eq()),
        BoxedParser::new(single_token(T!["<"])),
        BoxedParser::new(lt_eq()),
        BoxedParser::new(single_token(T![">"])),
        BoxedParser::new(gt_eq()),
    ])
}

pub fn calc_op(
) -> impl Parser<'static, TokenStream, LexedToken> {
    choice(vec![
        BoxedParser::new(single_token(T!["+"])),
        BoxedParser::new(single_token(T!["-"])),
        BoxedParser::new(single_token(T!["*"])),
        BoxedParser::new(single_token(T!["/"])),
    ])
}

pub fn bit_calc_op(
) -> impl Parser<'static, TokenStream, LexedToken> {
    choice(vec![
        BoxedParser::new(single_token(T!["&"])),
        BoxedParser::new(single_token(T!["|"])),
        BoxedParser::new(single_token(T!["~"])),
        BoxedParser::new(single_token(T!["^"])),
        BoxedParser::new(lt_lt()),
        BoxedParser::new(gt_gt()),
        BoxedParser::new(gt_gt_gt()),
    ])
}

pub fn unary_prefix_op(
) -> impl Parser<'static, TokenStream, LexedToken> {
    choice(vec![
        BoxedParser::new(plus_plus()),
        BoxedParser::new(minus_minus()),
        BoxedParser::new(single_token(T!["!"])),
        BoxedParser::new(single_token(TYPE_OF_KW)),
        BoxedParser::new(single_token(DELETE_KW)),
    ])
}

pub fn unary_suffix_op(
) -> impl Parser<'static, TokenStream, LexedToken> {
    either(plus_plus(), minus_minus())
}

pub fn list<ItemParser, SurroundParser>(
    left: SurroundParser,
    item: ItemParser,
    right: SurroundParser,
) -> impl Parser<'static, TokenStream, Vec<Node>>
where
    SurroundParser:
        Parser<'static, TokenStream, LexedToken>,
    ItemParser: Parser<'static, TokenStream, Node>,
{
    between(
        left,
        seq_by(item, single_token(T![","])),
        right,
    )
}

pub fn block<P>(
    parser: P,
) -> impl Parser<'static, TokenStream, Vec<Node>>
where
    P: Parser<'static, TokenStream, Node>,
{
    between(
        single_token(T!["{"]),
        zero_or_more(parser),
        single_token(T!["}"]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::grammar::literal;
    use crate::parser::ast::node::literal_node;
    use crate::parser::ast::Literal::{
        NumberLiteral, StringLiteral,
    };

    #[test]
    fn test_single_token() {
        let input = vec![
            (WHITESPACE, " ".to_string()),
            (DEFINATOR, "const".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![(DEFINATOR, "const".to_string())],
                (WHITESPACE, " ".to_string())
            )),
            single_token(WHITESPACE).parse(input.clone())
        );
        assert_eq!(
            Err(vec![
                (WHITESPACE, " ".to_string()),
                (DEFINATOR, "const".to_string())
            ]),
            single_token(DEFINATOR).parse(input)
        )
    }

    #[test]
    fn test_ws() {
        let input = vec![
            (WHITESPACE, " ".to_string()),
            (DEFINATOR, "const".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![(DEFINATOR, "const".to_string())],
                ()
            )),
            white_space().parse(input)
        )
    }

    #[test]
    fn test_composite_operator() {
        assert_eq!(
            Ok((vec![], (EQEQ, "==".to_string()))),
            eq_eq().parse(vec![
                (EQ, "=".to_string()),
                (EQ, "=".to_string()),
            ])
        );

        assert_eq!(
            Ok((vec![], (EQEQEQ, "===".to_string()))),
            eq_eq_eq().parse(vec![
                (EQ, "=".to_string()),
                (EQ, "=".to_string()),
                (EQ, "=".to_string()),
            ])
        );

        assert_eq!(
            Ok((vec![], (LTEQ, "<=".to_string()))),
            lt_eq().parse(vec![
                (LT, "<".to_string()),
                (EQ, "=".to_string()),
            ])
        );

        assert_eq!(
            Ok((vec![], (GTEQ, ">=".to_string()))),
            gt_eq().parse(vec![
                (GT, ">".to_string()),
                (EQ, "=".to_string()),
            ])
        );

        assert_eq!(
            Ok((vec![], (BANGEQ, "!=".to_string()).into())),
            bang_eq().parse(vec![
                (BANG, "!".to_string()),
                (EQ, "=".to_string()),
            ])
        );

        assert_eq!(
            Ok((vec![], (BANGEQEQ, "!==".to_string()))),
            bang_eq_eq().parse(vec![
                (BANG, "!".to_string()),
                (EQ, "=".to_string()),
                (EQ, "=".to_string()),
            ])
        );
    }

    #[test]
    fn test_list() {
        let one = literal_node(NumberLiteral {
            kind: NUMBER,
            value: 1,
            raw: "1".to_string(),
        });

        let input = vec![
            (OPEN_PAREN, "(".to_string()),
            (CLOSE_PAREN, ")".to_string()),
        ];
        assert_eq!(
            Ok((vec![], vec![])),
            list(
                single_token(T!["("]),
                literal(),
                single_token(T![")"])
            )
            .parse(input)
        );

        let input = vec![
            (OPEN_PAREN, "(".to_string()),
            (NUMBER, "1".to_string()),
            (CLOSE_PAREN, ")".to_string()),
        ];
        assert_eq!(
            Ok((vec![], vec![one.clone()])),
            list(
                single_token(T!["("]),
                literal(),
                single_token(T![")"])
            )
            .parse(input)
        );

        let input = vec![
            (OPEN_PAREN, "(".to_string()),
            (NUMBER, "1".to_string()),
            (COMMA, ",".to_string()),
            (NUMBER, "1".to_string()),
            (COMMA, ",".to_string()),
            (NUMBER, "1".to_string()),
            (CLOSE_PAREN, ")".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                vec![one.clone(), one.clone(), one.clone()]
            )),
            list(
                single_token(T!["("]),
                literal(),
                single_token(T![")"])
            )
            .parse(input)
        );

        let input = vec![
            (OPEN_PAREN, "(".to_string()),
            (NUMBER, "1".to_string()),
            (COMMA, ",".to_string()),
            (NUMBER, "1".to_string()),
            (COMMA, ",".to_string()),
            (NUMBER, "1".to_string()),
            (COMMA, ",".to_string()),
            (CLOSE_PAREN, ")".to_string()),
        ];
        assert_eq!(
            Err(input.clone()),
            list(
                single_token(T!["("]),
                literal(),
                single_token(T![")"])
            )
            .parse(input)
        )
    }

    #[test]
    fn test_block() {
        let foo = literal_node(StringLiteral {
            kind: STRING,
            value: "foo".to_string(),
            raw: "\"foo\"".to_string(),
        });
        let bar = literal_node(StringLiteral {
            kind: STRING,
            value: "bar".to_string(),
            raw: "\"bar\"".to_string(),
        });

        let input = vec![
            (OPEN_BRACE, "{".to_string()),
            (CLOSE_BRACE, "}".to_string()),
        ];
        assert_eq!(
            Ok((vec![], vec![])),
            block(literal()).parse(input)
        );

        let input = vec![
            (OPEN_BRACE, "{".to_string()),
            (STRING, "\"foo\"".to_string()),
            (STRING, "\"bar\"".to_string()),
            (CLOSE_BRACE, "}".to_string()),
        ];
        assert_eq!(
            Ok((vec![], vec![foo, bar])),
            block(literal()).parse(input)
        );
    }
}
