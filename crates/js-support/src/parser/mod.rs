mod ast;
mod lex;

pub(crate) use ast::{
    expr_node, literal_node, stat_node, visitor,
};
pub(crate) use lex::{
    type_judgument, LexedToken, TokenStream,
};

pub use ast::{syntax, Expr, Literal, Node, Stat};
pub use lex::lex;

#[cfg(test)]
mod tests {
    use crate::{
        parser::{
            expr_node, lex, literal_node, stat_node,
            syntax, Expr::*, Literal::*, Node::*, Stat::*,
        },
        syntax_kind::*,
    };

    #[test]
    fn smoke_test() {
        let input = lex(r#"
        function sayHello(value) {
            console.log(value + " 2022.11.4");
        }
        
        const msg = "Hello FMT";
        
        sayHello(msg);
        "#);
        let function_decla =
            Box::new(stat_node(FunctionDeclaStatement {
                kind: FUNCTION_DECLA_STAT,
                name: Box::new(literal_node(Id {
                    kind: ID,
                    name: "sayHello".to_string(),
                })),
                args: vec![Box::new(literal_node(Id {
                    kind: ID,
                    name: "value".to_string(),
                }))],
                body: vec![Box::new(expr_node(
                    FunctionCallExpr {
                        kind: FUNCTION_CALL_EXPR,
                        callee: Box::new(expr_node(
                            ValueAccessExpr {
                                kind: VALUE_ACCESS_EXPR,
                                path: vec![
                                    Box::new(literal_node(
                                        Id {
                                            kind: ID,
                                            name: "console"
                                                .to_string(
                                                ),
                                        },
                                    )),
                                    Box::new(literal_node(
                                        Id {
                                            kind: ID,
                                            name: "log"
                                                .to_string(
                                                ),
                                        },
                                    )),
                                ],
                            },
                        )),
                        args: vec![Box::new(expr_node(BinaryExpr {
                            kind: BINARY_EXPR,
                            left: Box::new(literal_node(
                                Id {
                                    kind: ID,
                                    name: "value"
                                        .to_string(),
                                },
                            )),
                            op: PLUS,
                            right: Box::new(literal_node(
                                StringLiteral {
                                    kind: STRING,
                                    value: " 2022.11.4"
                                        .to_string(),
                                    raw: "\" 2022.11.4\""
                                        .to_string(),
                                },
                            )),
                        }))],
                    },
                ))],
            }));
        let variable_decla =
            Box::new(stat_node(VariableDeclaStatement {
                kind: VARIABLE_DECLA_STAT,
                definator: "const".to_string(),
                name: Box::new(literal_node(Id {
                    kind: ID,
                    name: "msg".to_string(),
                })),
                init: Box::new(literal_node(
                    StringLiteral {
                        kind: STRING,
                        value: "Hello FMT".to_string(),
                        raw: "\"Hello FMT\"".to_string(),
                    },
                )),
            }));
        let function_call =
            Box::new(expr_node(FunctionCallExpr {
                kind: FUNCTION_CALL_EXPR,
                callee: Box::new(literal_node(Id {
                    kind: ID,
                    name: "sayHello".to_string(),
                })),
                args: vec![Box::new(literal_node(Id {
                    kind: ID,
                    name: "msg".to_string(),
                }))],
            }));

        assert_eq!(
            syntax(input),
            Ok(Root {
                kind: ROOT,
                statements: vec![
                    function_decla,
                    variable_decla,
                    function_call
                ]
            })
        );
    }
}
