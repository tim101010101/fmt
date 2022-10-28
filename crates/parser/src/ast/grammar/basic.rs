use crate::ast::token::{Token, TokenData};
use crate::lex::{LexedToken, TokenStream};
use crate::syntax_kind::{
    SyntaxKind, BANGEQ, BANGEQEQ, EQEQ, EQEQEQ, GTEQ, ID,
    LTEQ, WHITESPACE,
};
use crate::T;
use shared::parser_combiner::{
    atom, judge, map, BoxedParser, Parser,
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

// pub fn series_tokens(
//     expects: Vec<SyntaxKind>,
// ) -> impl Parser<'static, TokenStream, ()> {
//     move |input: TokenStream| {
//         let iter = expects.clone().into_iter();
//
//         iter.fold(
//             BoxedParser::new(single_token(expects[0])),
//             |res, cur| {
//                 res.and_then(move |_| single_token(cur))
//             },
//         )
//     }
// }

pub fn eq_eq() -> impl Parser<'static, TokenStream, Token> {
    single_token(T!["="])
        .and_then(|_| single_token(T!["="]))
        .map(|_| {
            TokenData::new(EQEQ, "==".to_string()).into()
        })
}

pub fn eq_eq_eq() -> impl Parser<'static, TokenStream, Token>
{
    eq_eq().and_then(|_| single_token(T!["="])).map(|_| {
        TokenData::new(EQEQEQ, "===".to_string()).into()
    })
}

pub fn lt_eq() -> impl Parser<'static, TokenStream, Token> {
    single_token(T!["<"])
        .and_then(|_| single_token(T!["="]))
        .map(|_| {
            TokenData::new(LTEQ, "<=".to_string()).into()
        })
}

pub fn gt_eq() -> impl Parser<'static, TokenStream, Token> {
    single_token(T![">"])
        .and_then(|_| single_token(T!["="]))
        .map(|_| {
            TokenData::new(GTEQ, ">=".to_string()).into()
        })
}

pub fn bang_eq() -> impl Parser<'static, TokenStream, Token>
{
    single_token(T!["!"])
        .and_then(|_| single_token(T!["="]))
        .map(|_| {
            TokenData::new(BANGEQ, "!=".to_string()).into()
        })
}

pub fn bang_eq_eq(
) -> impl Parser<'static, TokenStream, Token> {
    single_token(T!["!"])
        .and_then(|_| single_token(T!["="]))
        .and_then(|_| single_token(T!["="]))
        .map(|_| {
            TokenData::new(BANGEQEQ, "!==".to_string())
                .into()
        })
}

#[cfg(test)]
mod tests {
    use crate::ast::grammar::basic::{
        eq_eq, single_token, white_space,
    };
    use crate::ast::token::TokenData;
    use crate::syntax_kind::{
        DEFINATOR, EQ, EQEQ, WHITESPACE,
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
            Ok((
                vec![],
                TokenData::new(EQEQ, "==".to_string())
                    .into()
            )),
            eq_eq().parse(vec![
                (EQ, "=".to_string()),
                (EQ, "=".to_string()),
            ])
        )

        // TODO
    }
}
