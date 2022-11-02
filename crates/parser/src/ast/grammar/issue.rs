use crate::ast::grammar::{
    literal, plus_plus, single_token,
};
use crate::ast::{BinaryExpr, Node, UnaryExpr};
use crate::lex::{LexedToken, TokenStream};
use crate::syntax_kind::{
    SyntaxKind, BINARY_EXPR, UNARY_EXPR,
};
use crate::T;
use shared::parser_combiner::{
    either, left, zero_or_more, Parser,
};

pub fn b() -> impl Parser<'static, TokenStream, Node> {
    fn build_node(
        expr: Node,
        mut node_list: Vec<(SyntaxKind, Node)>,
    ) -> Node {
        match node_list.len() {
            0 => expr,
            _ => {
                let (op, right) = node_list.pop().unwrap();
                BinaryExpr {
                    kind: BINARY_EXPR,
                    left: Box::new(build_node(
                        expr, node_list,
                    )),
                    op,
                    right: Box::new(right),
                }
            }
        }
    }

    u().and_then(|left| {
        zero_or_more(single_token(T!["+"]).and_then(
            |(op, _)| u().map(move |right| (op, right)),
        ))
        .map(move |node_list| {
            let len = node_list.len();
            match len {
                0 => left.to_owned(),
                _ => build_node(left.to_owned(), node_list),
            }
        })
    })
}

pub fn u() -> impl Parser<'static, TokenStream, Node> {
    fn build_node(
        expr: Node,
        op_list: Vec<LexedToken>,
        cur: usize,
        prefix: bool,
    ) -> Node {
        if let Some((op, _)) = op_list.get(cur) {
            UnaryExpr {
                kind: UNARY_EXPR,
                prefix,
                op: op.to_owned(),
                expr: Box::new(build_node(
                    expr,
                    op_list,
                    cur + 1,
                    prefix,
                )),
            }
        } else {
            expr
        }
    }

    either(
        p().and_then(|expr| {
            zero_or_more(plus_plus()).map(move |op_list| {
                build_node(
                    expr.to_owned(),
                    op_list,
                    0,
                    false,
                )
            })
        }),
        plus_plus().and_then(|op| {
            u().map(move |expr| {
                build_node(
                    expr,
                    vec![op.to_owned()],
                    0,
                    true,
                )
            })
        }),
    )
}

pub fn p() -> impl Parser<'static, TokenStream, Node> {
    either(
        literal(),
        single_token(T!["("])
            .and_then(|_| left(b(), single_token(T![")"]))),
    )
}

#[cfg(test)]
mod tests {
    use crate::ast::grammar::issue::{b, u};
    use crate::ast::{
        BinaryExpr, NumberLiteral, UnaryExpr,
    };
    use crate::syntax_kind::{
        BINARY_EXPR, CLOSE_PAREN, NUMBER, OPEN_PAREN, PLUS,
        PLUSPLUS, UNARY_EXPR,
    };
    use shared::parser_combiner::Parser;

    #[test]
    fn t1() {
        let input = vec![
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "1".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                UnaryExpr {
                    kind: UNARY_EXPR,
                    prefix: true,
                    op: PLUSPLUS,
                    expr: Box::new(NumberLiteral {
                        kind: NUMBER,
                        value: 1,
                        raw: "1".to_string()
                    })
                }
            )),
            u().parse(input)
        )
    }

    #[test]
    fn t2() {
        let input = vec![
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "2".to_string()),
            (CLOSE_PAREN, ")".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                UnaryExpr {
                    kind: UNARY_EXPR,
                    prefix: true,
                    op: PLUSPLUS,
                    expr: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: Box::new(NumberLiteral {
                            kind: NUMBER,
                            value: 1,
                            raw: "1".to_string()
                        }),
                        op: PLUS,
                        right: Box::new(NumberLiteral {
                            kind: NUMBER,
                            value: 2,
                            raw: "2".to_string()
                        })
                    })
                }
            )),
            b().parse(input)
        )
    }

    #[test]
    fn issue_1() {
        let input = vec![
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "2".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                BinaryExpr {
                    kind: BINARY_EXPR,
                    left: Box::new(UnaryExpr {
                        kind: UNARY_EXPR,
                        prefix: true,
                        op: PLUSPLUS,
                        expr: Box::new(NumberLiteral {
                            kind: NUMBER,
                            value: 1,
                            raw: "1".to_string()
                        })
                    }),
                    op: PLUS,
                    right: Box::new(NumberLiteral {
                        kind: NUMBER,
                        value: 2,
                        raw: "2".to_string()
                    })
                }
            )),
            b().parse(input)
        );
    }
}
