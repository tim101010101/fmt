use crate::demo::store::NodeStore;
use crate::demo::tree::Node;
use crate::lex::{LexedToken, TokenStream};
use crate::syntax_kind::{SyntaxKind, STRING};
use shared::parser_combiner::{atom, judge, Parser};

pub fn single_token(
    expect: SyntaxKind,
) -> impl Parser<'static, (TokenStream, NodeStore), LexedToken>
{
    judge(
        atom::<(TokenStream, NodeStore), LexedToken>(),
        move |(kind, _)| *kind == expect,
    )
}

pub fn string_literal(
) -> impl Parser<(TokenStream, NodeStore), Node> {
    todo!()
}

fn expr() -> impl Parser<(TokenStream, NodeStore), Node> {
    todo!()
}

fn expr1() -> impl Parser<(TokenStream, NodeStore), Node> {
    todo!()
}
