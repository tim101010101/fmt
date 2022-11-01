use crate::ast::grammar::literal;
use crate::ast::Node;
use crate::lex::TokenStream;
use shared::parser_combiner::{
    choice, BoxedParser, Parser,
};

/// Expr -> Expr + Expr1 | Expr1
/// Expr1 -> Expr ++ | ++ Expr | li
///
/// Expr -> Binary | Expr1
/// Binary -> Expr + Expr1
/// Expr1 -> Unary | li
/// Unary -> Expr++ | ++Expr
///
///
pub fn expr() -> impl Parser<'static, TokenStream, Node> {
    choice(vec![
        BoxedParser::new(literal()),
        // BoxedParser::new(unary_expr()),
        // BoxedParser::new(binary_expr()),
    ])
}

#[cfg(test)]
mod tests {
    use crate::ast::grammar::expr;
    use crate::ast::{
        BinaryExpr, NumberLiteral, UnaryExpr,
    };
    use crate::syntax_kind::{
        BINARY_EXPR, NUMBER, PLUS, PLUSPLUS, UNARY_EXPR,
    };

    #[test]
    fn issue_1() {
        let input = vec![
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "1".to_string()),
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
                        value: 1,
                        raw: "1".to_string()
                    })
                }
            )),
            expr().parse(input)
        );
    }
}
