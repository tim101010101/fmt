use crate::lex::{LexedToken, TokenStream};
use crate::syntax_kind::{
    SyntaxKind, BANGEQ, BANGEQEQ, EQEQ, EQEQEQ, GTEQ, GTGT,
    GTGTGT, LTEQ, LTLT, MINUSMINUS, PLUSPLUS, WHITESPACE,
};
use crate::T;
use shared::parser_combiner::{
    atom, judge, map, one_of, BoxedParser, Parser,
};

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
    one_of(vec![
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
    one_of(vec![
        BoxedParser::new(single_token(T!["+"])),
        BoxedParser::new(single_token(T!["-"])),
        BoxedParser::new(single_token(T!["*"])),
        BoxedParser::new(single_token(T!["/"])),
    ])
}

pub fn bit_calc_op(
) -> impl Parser<'static, TokenStream, LexedToken> {
    one_of(vec![
        BoxedParser::new(single_token(T!["&"])),
        BoxedParser::new(single_token(T!["|"])),
        BoxedParser::new(single_token(T!["~"])),
        BoxedParser::new(single_token(T!["^"])),
        BoxedParser::new(lt_lt()),
        BoxedParser::new(gt_gt()),
        BoxedParser::new(gt_gt_gt()),
    ])
}

#[cfg(test)]
mod tests {
    use crate::ast::grammar::basic::{
        bang_eq, bang_eq_eq, eq_eq, eq_eq_eq, gt_eq, lt_eq,
        single_token, white_space,
    };
    use crate::syntax_kind::{
        BANG, BANGEQ, BANGEQEQ, DEFINATOR, EQ, EQEQ,
        EQEQEQ, GT, GTEQ, LT, LTEQ, WHITESPACE,
    };
    use shared::parser_combiner::Parser;

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
}
