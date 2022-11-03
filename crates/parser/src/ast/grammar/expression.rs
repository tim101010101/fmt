use crate::ast::grammar::{
    bit_calc_op, comparison_op, list, literal,
    single_token, unary_prefix_op, unary_suffix_op,
};
use crate::ast::{
    AssignmentExpr, BinaryExpr, FunctionCallExpr, Id, Node,
    ReturnExpr, TernaryExpr, UnaryExpr, ValueAccessExpr,
};
use crate::lex::{LexedToken, TokenStream};
use crate::syntax_kind::{
    SyntaxKind, ASSIGNMENT_EXPR, BINARY_EXPR,
    FUNCTION_CALL_EXPR, ID, INSTANCE_OF_KW, IN_KW,
    RETURN_EXPR, RETURN_KW, TERNARY_EXPR, UNARY_EXPR,
    VALUE_ACCESS_EXPR,
};
use crate::T;
use shared::parser_combiner::{
    chainl, either, right, zero_or_more, zero_or_one,
    Parser,
};

/// Expr -> ReturnExpr | AssignmentExpr
pub fn expr() -> impl Parser<'static, TokenStream, Node> {
    either(return_expr(), assignment_expr())
}

/// ReturnExpr -> RETURN AssignmentExpr
pub fn return_expr(
) -> impl Parser<'static, TokenStream, Node> {
    single_token(RETURN_KW).and_then(|_| {
        assignment_expr().map(|expr| ReturnExpr {
            kind: RETURN_EXPR,
            expr: Box::new(expr),
        })
    })
}

/// AssignmentExpr -> TernaryExpr ("=" TernaryExpr)*
pub fn assignment_expr(
) -> impl Parser<'static, TokenStream, Node> {
    fn build_node(
        left: Node,
        right_list: Vec<Node>,
        cur: usize,
    ) -> Node {
        if let Some(right) = right_list.get(cur) {
            AssignmentExpr {
                kind: ASSIGNMENT_EXPR,
                left: Box::new(left),
                right: Box::new(build_node(
                    right.to_owned(),
                    right_list,
                    cur + 1,
                )),
            }
        } else {
            left
        }
    }

    ternary_expr().and_then(|left| {
        zero_or_more(right(
            single_token(T!["="]),
            ternary_expr(),
        ))
        .map(move |right_list| {
            build_node(left.to_owned(), right_list, 0)
        })
    })
}

/// TernaryExpr -> BinaryExpr ("?" TernaryExpr ":" TernaryExpr)?
pub fn ternary_expr(
) -> impl Parser<'static, TokenStream, Node> {
    binary_expr().and_then(|condition| {
        zero_or_one(
            right(single_token(T!["?"]), ternary_expr())
                .and_then(|then_expr| {
                    right(
                        single_token(T![":"]),
                        ternary_expr(),
                    )
                    .map(
                        move |else_expr| {
                            (
                                then_expr.to_owned(),
                                else_expr,
                            )
                        },
                    )
                }),
        )
        .map(move |res| match res {
            None => condition.to_owned(),
            Some((then_expr, else_expr)) => TernaryExpr {
                kind: TERNARY_EXPR,
                condition: Box::new(condition.to_owned()),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            },
        })
    })
}

fn build_binary_expr_node(
    expr: Node,
    mut node_list: Vec<(SyntaxKind, Node)>,
) -> Node {
    match node_list.len() {
        0 => expr,
        _ => {
            let (op, right) = node_list.pop().unwrap();
            BinaryExpr {
                kind: BINARY_EXPR,
                left: Box::new(build_binary_expr_node(
                    expr, node_list,
                )),
                op,
                right: Box::new(right),
            }
        }
    }
}

/// BinaryExpr -> BinaryExpr1 ( ( "==" | "===" | "<" | "<=" | ">" | ">=" ) BinaryExpr1 )*
pub fn binary_expr(
) -> impl Parser<'static, TokenStream, Node> {
    binary_expr1().and_then(|left| {
        zero_or_more(comparison_op().and_then(|(op, _)| {
            binary_expr1().map(move |right| (op, right))
        }))
        .map(move |node_list| {
            let len = node_list.len();
            match len {
                0 => left.to_owned(),
                _ => build_binary_expr_node(
                    left.to_owned(),
                    node_list,
                ),
            }
        })
    })
}

/// BinaryExpr1 -> BinaryExpr2 ( ( "+" | "-" ) BinaryExpr2 )*
pub fn binary_expr1(
) -> impl Parser<'static, TokenStream, Node> {
    binary_expr2().and_then(|left| {
        zero_or_more(
            either(
                single_token(T!["+"]),
                single_token(T!["-"]),
            )
            .and_then(|(op, _)| {
                binary_expr2().map(move |right| (op, right))
            }),
        )
        .map(move |node_list| {
            let len = node_list.len();
            match len {
                0 => left.to_owned(),
                _ => build_binary_expr_node(
                    left.to_owned(),
                    node_list,
                ),
            }
        })
    })
}

/// BinaryExpr2 -> BinaryExpr3 ( ( "*" | "/" ) BinaryExpr3 )*
pub fn binary_expr2(
) -> impl Parser<'static, TokenStream, Node> {
    binary_expr3().and_then(|left| {
        zero_or_more(
            either(
                single_token(T!["*"]),
                single_token(T!["/"]),
            )
            .and_then(|(op, _)| {
                binary_expr3().map(move |right| (op, right))
            }),
        )
        .map(move |node_list| {
            let len = node_list.len();
            match len {
                0 => left.to_owned(),
                _ => build_binary_expr_node(
                    left.to_owned(),
                    node_list,
                ),
            }
        })
    })
}

/// BianryExpr3 -> BinaryExpr4 ( ( "&" | "|" | "^" | "~" | "<<" | ">>" | ">>>" ) BinaryExpr4 )*
pub fn binary_expr3(
) -> impl Parser<'static, TokenStream, Node> {
    binary_expr4().and_then(|left| {
        zero_or_more(bit_calc_op().and_then(|(op, _)| {
            binary_expr4().map(move |right| (op, right))
        }))
        .map(move |node_list| {
            let len = node_list.len();
            match len {
                0 => left.to_owned(),
                _ => build_binary_expr_node(
                    left.to_owned(),
                    node_list,
                ),
            }
        })
    })
}

/// BianryExpr4 -> UnaryExpr ( ( INSTANCE_OF | IN ) UnaryExpr )*
pub fn binary_expr4(
) -> impl Parser<'static, TokenStream, Node> {
    unary_expr().and_then(|left| {
        zero_or_more(
            either(
                single_token(INSTANCE_OF_KW),
                single_token(IN_KW),
            )
            .and_then(|(op, _)| {
                unary_expr().map(move |right| (op, right))
            }),
        )
        .map(move |node_list| {
            let len = node_list.len();
            match len {
                0 => left.to_owned(),
                _ => build_binary_expr_node(
                    left.to_owned(),
                    node_list,
                ),
            }
        })
    })
}

/// UnaryExpr -> (("++" | "--" | "!" | TYPE_OF | DELETE) UnaryExpr) ("++" | "--")*
///            | FunctionCallExpr ("++" | "--")*
///            | ValueAccessExpr ("++" | "--")*
pub fn unary_expr(
) -> impl Parser<'static, TokenStream, Node> {
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
        unary_prefix_op().and_then(|op| {
            unary_expr()
                .map(move |expr| {
                    build_node(
                        expr,
                        vec![op.to_owned()],
                        0,
                        true,
                    )
                })
                .and_then(move |expr| {
                    zero_or_more(unary_suffix_op()).map(
                        move |op_list| {
                            build_node(
                                expr.to_owned(),
                                op_list,
                                0,
                                false,
                            )
                        },
                    )
                })
        }),
        either(function_call_expr(), value_access_expr())
            .and_then(move |expr| {
                zero_or_more(unary_suffix_op()).map(
                    move |op_list| {
                        build_node(
                            expr.to_owned(),
                            op_list,
                            0,
                            false,
                        )
                    },
                )
            }),
    )
}

/// FunctionCallExpr -> ValueAccessExpr ( "(" (TernaryExpr ("," TernaryExpr)*)? ")" )*
///                   | Factor ( "(" (TernaryExpr ("," TernaryExpr)*)? ")" )*
pub fn function_call_expr(
) -> impl Parser<'static, TokenStream, Node> {
    fn build_node(
        expr: Node,
        mut args: Vec<Vec<Node>>,
    ) -> Node {
        if let Some(args_list) = args.pop() {
            FunctionCallExpr {
                kind: FUNCTION_CALL_EXPR,
                callee: Box::new(build_node(expr, args)),
                args: args_list
                    .iter()
                    .map(|n| Box::new(n.to_owned()))
                    .collect(),
            }
        } else {
            expr
        }
    }

    either(value_access_expr(), factor()).and_then(|expr| {
        zero_or_more(list(
            single_token(T!["("]),
            ternary_expr(),
            single_token(T![")"]),
        ))
        .map(move |args| build_node(expr.to_owned(), args))
    })
}

/// ValueAccessExpr -> FunctionCallExpr ("." Factor)*
///                  | Factor ("." Factor)*
pub fn value_access_expr(
) -> impl Parser<'static, TokenStream, Node> {
    chainl(
        factor(),
        // TODO bug here
        // either(function_call_expr(), factor()),
        single_token(T!["."]),
    )
    .map(|path| match path.len() {
        1 => path.get(0).unwrap().to_owned(),
        _ => ValueAccessExpr {
            kind: VALUE_ACCESS_EXPR,
            path: path
                .iter()
                .map(|n| Box::new(n.to_owned()))
                .collect(),
        },
    })
}

/// Factor -> literal | ID | "(" AssignmentExpr ")"
pub fn factor() -> impl Parser<'static, TokenStream, Node> {
    either(
        either(
            literal(),
            single_token(ID)
                .map(|(_, name)| Id { kind: ID, name }),
        ),
        // TODO there's a bug in `between`
        single_token(T!["("])
            .and_then(|_| assignment_expr())
            .and_then(|node| {
                single_token(T![")"])
                    .map(move |_| node.to_owned())
            }),
    )
}

#[cfg(test)]
mod tests {
    use crate::ast::grammar::expression::*;
    use crate::ast::*;
    use crate::syntax_kind::*;
    use shared::parser_combiner::Parser;

    fn get_number(
    ) -> (Box<Node>, Box<Node>, Box<Node>, Box<Node>) {
        let one = Box::new(NumberLiteral {
            kind: NUMBER,
            value: 1,
            raw: "1".to_string(),
        });
        let two = Box::new(NumberLiteral {
            kind: NUMBER,
            value: 2,
            raw: "2".to_string(),
        });
        let three = Box::new(NumberLiteral {
            kind: NUMBER,
            value: 3,
            raw: "3".to_string(),
        });
        let four = Box::new(NumberLiteral {
            kind: NUMBER,
            value: 4,
            raw: "4".to_string(),
        });
        (one, two, three, four)
    }

    #[test]
    fn test_priority() {
        let (one, two, three, four) = get_number();
        let five = Box::new(NumberLiteral {
            kind: NUMBER,
            value: 5,
            raw: "5".to_string(),
        });
        let six = Box::new(NumberLiteral {
            kind: NUMBER,
            value: 6,
            raw: "6".to_string(),
        });
        let foo = Box::new(Id {
            kind: ID,
            name: "foo".to_string(),
        });
        let bar = Box::new(Id {
            kind: ID,
            name: "bar".to_string(),
        });
        let foo_str = Box::new(StringLiteral {
            kind: STRING,
            value: "foo".to_string(),
            raw: "foo".to_string(),
        });
        let bar_str = Box::new(StringLiteral {
            kind: STRING,
            value: "bar".to_string(),
            raw: "bar".to_string(),
        });

        // ++1 + 2
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
                        expr: one.clone()
                    }),
                    op: PLUS,
                    right: two.clone()
                }
            )),
            expr().parse(input)
        );

        // ++1 + (++2)
        let input = vec![
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "2".to_string()),
            (CLOSE_PAREN, ")".to_string()),
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
                        expr: one.clone()
                    }),
                    op: PLUS,
                    right: Box::new(UnaryExpr {
                        kind: UNARY_EXPR,
                        prefix: true,
                        op: PLUSPLUS,
                        expr: two.clone()
                    })
                }
            )),
            expr().parse(input)
        );

        // 1++ + (++2)
        let input = vec![
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "2".to_string()),
            (CLOSE_PAREN, ")".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                BinaryExpr {
                    kind: BINARY_EXPR,
                    left: Box::new(UnaryExpr {
                        kind: UNARY_EXPR,
                        prefix: false,
                        op: PLUSPLUS,
                        expr: one.clone()
                    }),
                    op: PLUS,
                    right: Box::new(UnaryExpr {
                        kind: UNARY_EXPR,
                        prefix: true,
                        op: PLUSPLUS,
                        expr: two.clone()
                    })
                }
            )),
            expr().parse(input)
        );

        // 1 << 2 + 3
        let input = vec![
            (NUMBER, "1".to_string()),
            (LT, "<".to_string()),
            (LT, "<".to_string()),
            (NUMBER, "2".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "3".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                BinaryExpr {
                    kind: BINARY_EXPR,
                    left: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: one.clone(),
                        op: LTLT,
                        right: two.clone()
                    }),
                    op: PLUS,
                    right: three.clone()
                }
            )),
            expr().parse(input)
        );

        // 1 << 2 * 3
        let input = vec![
            (NUMBER, "1".to_string()),
            (LT, "<".to_string()),
            (LT, "<".to_string()),
            (NUMBER, "2".to_string()),
            (STAR, "*".to_string()),
            (NUMBER, "3".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                BinaryExpr {
                    kind: BINARY_EXPR,
                    left: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: one.clone(),
                        op: LTLT,
                        right: two.clone()
                    }),
                    op: STAR,
                    right: three.clone()
                }
            )),
            expr().parse(input)
        );

        // 1 << ( 2 + 3 )
        let input = vec![
            (NUMBER, "1".to_string()),
            (LT, "<".to_string()),
            (LT, "<".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (NUMBER, "2".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "3".to_string()),
            (CLOSE_PAREN, ")".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                BinaryExpr {
                    kind: BINARY_EXPR,
                    left: one.clone(),
                    op: LTLT,
                    right: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: two.clone(),
                        op: PLUS,
                        right: three.clone()
                    })
                }
            )),
            expr().parse(input)
        );

        // 1 < 2 ? 3 + 4 : 5++
        let input = vec![
            (NUMBER, "1".to_string()),
            (LT, "<".to_string()),
            (NUMBER, "2".to_string()),
            (QUESTION, "?".to_string()),
            (NUMBER, "3".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "4".to_string()),
            (COLON, ":".to_string()),
            (NUMBER, "5".to_string()),
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                TernaryExpr {
                    kind: TERNARY_EXPR,
                    condition: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: one.clone(),
                        op: LT,
                        right: two.clone()
                    }),
                    then_expr: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: three.clone(),
                        op: PLUS,
                        right: four.clone()
                    }),
                    else_expr: Box::new(UnaryExpr {
                        kind: UNARY_EXPR,
                        prefix: false,
                        op: PLUSPLUS,
                        expr: five.clone()
                    })
                }
            )),
            expr().parse(input)
        );

        // foo.bar()
        let input = vec![
            (ID, "foo".to_string()),
            (DOT, ".".to_string()),
            (ID, "bar".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (CLOSE_PAREN, ")".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                FunctionCallExpr {
                    kind: FUNCTION_CALL_EXPR,
                    callee: Box::new(ValueAccessExpr {
                        kind: VALUE_ACCESS_EXPR,
                        path: vec![
                            foo.clone(),
                            bar.clone()
                        ]
                    }),
                    args: vec![]
                }
            )),
            expr().parse(input)
        );
    }

    #[test]
    fn test_return_expr() {
        let (one, two, _, _) = get_number();

        let input = vec![
            (RETURN_KW, "return".to_string()),
            (NUMBER, "1".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                ReturnExpr {
                    kind: RETURN_EXPR,
                    expr: one.clone()
                }
            )),
            return_expr().parse(input)
        );

        let input = vec![
            (RETURN_KW, "return".to_string()),
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "2".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                ReturnExpr {
                    kind: RETURN_EXPR,
                    expr: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: one.clone(),
                        op: PLUS,
                        right: two.clone()
                    })
                }
            )),
            return_expr().parse(input)
        );
    }

    #[test]
    fn test_assignment_expr() {
        let (one, two, three, _) = get_number();

        let input = vec![
            (NUMBER, "1".to_string()),
            (EQ, "=".to_string()),
            (NUMBER, "2".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                AssignmentExpr {
                    kind: ASSIGNMENT_EXPR,
                    left: one.clone(),
                    right: two.clone()
                }
            )),
            assignment_expr().parse(input)
        );

        let input = vec![
            (NUMBER, "1".to_string()),
            (EQ, "=".to_string()),
            (NUMBER, "2".to_string()),
            (EQ, "=".to_string()),
            (NUMBER, "3".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                AssignmentExpr {
                    kind: ASSIGNMENT_EXPR,
                    left: one.clone(),
                    right: Box::new(AssignmentExpr {
                        kind: ASSIGNMENT_EXPR,
                        left: two.clone(),
                        right: three.clone()
                    })
                }
            )),
            assignment_expr().parse(input)
        );
    }

    #[test]
    fn test_ternary_expr() {
        let (one, two, three, four) = get_number();
        let five = Box::new(NumberLiteral {
            kind: NUMBER,
            value: 5,
            raw: "5".to_string(),
        });
        let six = Box::new(NumberLiteral {
            kind: NUMBER,
            value: 6,
            raw: "6".to_string(),
        });
        let seven = Box::new(NumberLiteral {
            kind: NUMBER,
            value: 7,
            raw: "7".to_string(),
        });

        let input = vec![
            (NUMBER, "1".to_string()),
            (EQ, "=".to_string()),
            (EQ, "=".to_string()),
            (NUMBER, "1".to_string()),
            (QUESTION, "?".to_string()),
            (NUMBER, "2".to_string()),
            (COLON, ":".to_string()),
            (NUMBER, "3".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                TernaryExpr {
                    kind: TERNARY_EXPR,
                    condition: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: one.clone(),
                        op: EQEQ,
                        right: one.clone()
                    }),
                    then_expr: two.clone(),
                    else_expr: three.clone()
                }
            )),
            ternary_expr().parse(input)
        );

        let input = vec![
            (NUMBER, "1".to_string()),
            (EQ, "=".to_string()),
            (EQ, "=".to_string()),
            (NUMBER, "1".to_string()),
            (QUESTION, "?".to_string()),
            (NUMBER, "2".to_string()),
            (COLON, ":".to_string()),
            (NUMBER, "3".to_string()),
            (EQ, "=".to_string()),
            (EQ, "=".to_string()),
            (NUMBER, "3".to_string()),
            (QUESTION, "?".to_string()),
            (NUMBER, "4".to_string()),
            (COLON, ":".to_string()),
            (NUMBER, "5".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                TernaryExpr {
                    kind: TERNARY_EXPR,
                    condition: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: one.clone(),
                        op: EQEQ,
                        right: one.clone()
                    }),
                    then_expr: two.clone(),
                    else_expr: Box::new(TernaryExpr {
                        kind: TERNARY_EXPR,
                        condition: Box::new(BinaryExpr {
                            kind: BINARY_EXPR,
                            left: three.clone(),
                            op: EQEQ,
                            right: three.clone()
                        }),
                        then_expr: four.clone(),
                        else_expr: five.clone()
                    })
                }
            )),
            ternary_expr().parse(input)
        );

        let input = vec![
            (NUMBER, "1".to_string()),
            (EQ, "=".to_string()),
            (EQ, "=".to_string()),
            (NUMBER, "1".to_string()),
            (QUESTION, "?".to_string()),
            (NUMBER, "2".to_string()),
            (EQ, "=".to_string()),
            (EQ, "=".to_string()),
            (NUMBER, "2".to_string()),
            (QUESTION, "?".to_string()),
            (NUMBER, "3".to_string()),
            (COLON, ":".to_string()),
            (NUMBER, "4".to_string()),
            (COLON, ":".to_string()),
            (NUMBER, "5".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                TernaryExpr {
                    kind: TERNARY_EXPR,
                    condition: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: one.clone(),
                        op: EQEQ,
                        right: one.clone()
                    }),
                    then_expr: Box::new(TernaryExpr {
                        kind: TERNARY_EXPR,
                        condition: Box::new(BinaryExpr {
                            kind: BINARY_EXPR,
                            left: two.clone(),
                            op: EQEQ,
                            right: two.clone()
                        }),
                        then_expr: three.clone(),
                        else_expr: four.clone()
                    }),
                    else_expr: five.clone()
                }
            )),
            ternary_expr().parse(input)
        );

        let input = vec![
            (NUMBER, "1".to_string()),
            (EQ, "=".to_string()),
            (EQ, "=".to_string()),
            (NUMBER, "1".to_string()),
            (QUESTION, "?".to_string()),
            (NUMBER, "2".to_string()),
            (EQ, "=".to_string()),
            (EQ, "=".to_string()),
            (NUMBER, "2".to_string()),
            (QUESTION, "?".to_string()),
            (NUMBER, "3".to_string()),
            (COLON, ":".to_string()),
            (NUMBER, "4".to_string()),
            (COLON, ":".to_string()),
            (NUMBER, "5".to_string()),
            (EQ, "=".to_string()),
            (EQ, "=".to_string()),
            (NUMBER, "5".to_string()),
            (QUESTION, "?".to_string()),
            (NUMBER, "6".to_string()),
            (COLON, ":".to_string()),
            (NUMBER, "7".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                TernaryExpr {
                    kind: TERNARY_EXPR,
                    condition: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: one.clone(),
                        op: EQEQ,
                        right: one.clone()
                    }),
                    then_expr: Box::new(TernaryExpr {
                        kind: TERNARY_EXPR,
                        condition: Box::new(BinaryExpr {
                            kind: BINARY_EXPR,
                            left: two.clone(),
                            op: EQEQ,
                            right: two.clone()
                        }),
                        then_expr: three.clone(),
                        else_expr: four.clone()
                    }),
                    else_expr: Box::new(TernaryExpr {
                        kind: TERNARY_EXPR,
                        condition: Box::new(BinaryExpr {
                            kind: BINARY_EXPR,
                            left: five.clone(),
                            op: EQEQ,
                            right: five.clone()
                        }),
                        then_expr: six.clone(),
                        else_expr: seven.clone()
                    })
                }
            )),
            ternary_expr().parse(input)
        );
    }

    #[test]
    fn test_binary_expr() {
        let (one, two, three, _) = get_number();

        let input = vec![
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "2".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                BinaryExpr {
                    kind: BINARY_EXPR,
                    left: one.clone(),
                    op: PLUS,
                    right: two.clone()
                }
            )),
            binary_expr().parse(input)
        );

        let input = vec![
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "2".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "3".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                BinaryExpr {
                    kind: BINARY_EXPR,
                    left: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: one.clone(),
                        op: PLUS,
                        right: two.clone()
                    }),
                    op: PLUS,
                    right: three.clone()
                }
            )),
            binary_expr().parse(input)
        );

        let input = vec![
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "2".to_string()),
            (STAR, "*".to_string()),
            (NUMBER, "3".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                BinaryExpr {
                    kind: BINARY_EXPR,
                    left: one.clone(),
                    op: PLUS,
                    right: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: two.clone(),
                        op: STAR,
                        right: three.clone()
                    })
                }
            )),
            binary_expr().parse(input)
        );

        let input = vec![
            (NUMBER, "1".to_string()),
            (EQ, "=".to_string()),
            (EQ, "=".to_string()),
            (NUMBER, "2".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "3".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                BinaryExpr {
                    kind: BINARY_EXPR,
                    left: one.clone(),
                    op: EQEQ,
                    right: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: two.clone(),
                        op: PLUS,
                        right: three.clone()
                    })
                }
            )),
            binary_expr().parse(input)
        );

        let input = vec![
            (OPEN_PAREN, "(".to_string()),
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "2".to_string()),
            (CLOSE_PAREN, ")".to_string()),
            (STAR, "*".to_string()),
            (NUMBER, "3".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                BinaryExpr {
                    kind: BINARY_EXPR,
                    left: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: one.clone(),
                        op: PLUS,
                        right: two.clone()
                    }),
                    op: STAR,
                    right: three.clone()
                }
            )),
            binary_expr().parse(input)
        );
    }

    #[test]
    fn test_unary_expr() {
        let (one, _, _, _) = get_number();

        let input = vec![
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                UnaryExpr {
                    kind: UNARY_EXPR,
                    prefix: false,
                    op: PLUSPLUS,
                    expr: one.clone()
                }
            )),
            unary_expr().parse(input)
        );

        let input = vec![
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                UnaryExpr {
                    kind: UNARY_EXPR,
                    prefix: false,
                    op: PLUSPLUS,
                    expr: one.clone()
                }
            )),
            unary_expr().parse(input)
        );

        let input = vec![
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                UnaryExpr {
                    kind: UNARY_EXPR,
                    prefix: true,
                    op: PLUSPLUS,
                    expr: Box::new(UnaryExpr {
                        kind: UNARY_EXPR,
                        prefix: false,
                        op: PLUSPLUS,
                        expr: one.clone()
                    })
                }
            )),
            unary_expr().parse(input)
        );

        let input = vec![
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                UnaryExpr {
                    kind: UNARY_EXPR,
                    prefix: false,
                    op: PLUSPLUS,
                    expr: Box::new(UnaryExpr {
                        kind: UNARY_EXPR,
                        prefix: false,
                        op: PLUSPLUS,
                        expr: one.clone()
                    })
                }
            )),
            unary_expr().parse(input)
        );

        let input = vec![
            (PLUS, "+".to_string()),
            (PLUS, "+".to_string()),
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
                    expr: Box::new(UnaryExpr {
                        kind: UNARY_EXPR,
                        prefix: true,
                        op: PLUSPLUS,
                        expr: one.clone()
                    })
                }
            )),
            unary_expr().parse(input)
        );
    }

    #[test]
    fn test_function_call() {
        let input = vec![
            (ID, "foo".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (CLOSE_PAREN, ")".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                FunctionCallExpr {
                    kind: FUNCTION_CALL_EXPR,
                    callee: Box::new(Id {
                        kind: ID,
                        name: "foo".to_string()
                    }),
                    args: vec![]
                }
            )),
            function_call_expr().parse(input)
        );

        let input = vec![
            (ID, "foo".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (STRING, "bar".to_string()),
            (COMMA, ",".to_string()),
            (STRING, "baz".to_string()),
            (CLOSE_PAREN, ")".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                FunctionCallExpr {
                    kind: FUNCTION_CALL_EXPR,
                    callee: Box::new(Id {
                        kind: ID,
                        name: "foo".to_string()
                    }),
                    args: vec![
                        Box::new(StringLiteral {
                            kind: STRING,
                            value: "bar".to_string(),
                            raw: "bar".to_string()
                        }),
                        Box::new(StringLiteral {
                            kind: STRING,
                            value: "baz".to_string(),
                            raw: "baz".to_string()
                        })
                    ]
                }
            )),
            function_call_expr().parse(input)
        );

        let input = vec![
            (ID, "foo".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (CLOSE_PAREN, ")".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (CLOSE_PAREN, ")".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                FunctionCallExpr {
                    kind: FUNCTION_CALL_EXPR,
                    callee: Box::new(FunctionCallExpr {
                        kind: FUNCTION_CALL_EXPR,
                        callee: Box::new(Id {
                            kind: ID,
                            name: "foo".to_string()
                        }),
                        args: vec![]
                    }),
                    args: vec![]
                }
            )),
            function_call_expr().parse(input)
        );

        let input = vec![
            (ID, "foo".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (STRING, "bar".to_string()),
            (CLOSE_PAREN, ")".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (STRING, "baz".to_string()),
            (CLOSE_PAREN, ")".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                FunctionCallExpr {
                    kind: FUNCTION_CALL_EXPR,
                    callee: Box::new(FunctionCallExpr {
                        kind: FUNCTION_CALL_EXPR,
                        callee: Box::new(Id {
                            kind: ID,
                            name: "foo".to_string()
                        }),
                        args: vec![Box::new(
                            StringLiteral {
                                kind: STRING,
                                value: "bar".to_string(),
                                raw: "bar".to_string()
                            }
                        )]
                    }),
                    args: vec![Box::new(StringLiteral {
                        kind: STRING,
                        value: "baz".to_string(),
                        raw: "baz".to_string()
                    })]
                }
            )),
            function_call_expr().parse(input)
        );

        let input = vec![
            (ID, "foo".to_string()),
            (DOT, ".".to_string()),
            (ID, "bar".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (CLOSE_PAREN, ")".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                FunctionCallExpr {
                    kind: FUNCTION_CALL_EXPR,
                    callee: Box::new(ValueAccessExpr {
                        kind: VALUE_ACCESS_EXPR,
                        path: vec![
                            Box::new(Id {
                                kind: ID,
                                name: "foo".to_string()
                            }),
                            Box::new(Id {
                                kind: ID,
                                name: "bar".to_string()
                            })
                        ]
                    }),
                    args: vec![]
                }
            )),
            function_call_expr().parse(input)
        )
    }

    #[test]
    fn test_value_access() {
        let input = vec![
            (ID, "foo".to_string()),
            (DOT, ".".to_string()),
            (ID, "bar".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                ValueAccessExpr {
                    kind: VALUE_ACCESS_EXPR,
                    path: vec![
                        Box::new(Id {
                            kind: ID,
                            name: "foo".to_string()
                        }),
                        Box::new(Id {
                            kind: ID,
                            name: "bar".to_string()
                        })
                    ]
                }
            )),
            value_access_expr().parse(input)
        )
    }

    #[test]
    fn test_factor() {
        let input = vec![(STRING, "foo".to_string())];
        assert_eq!(
            Ok((
                vec![],
                StringLiteral {
                    kind: STRING,
                    value: "foo".to_string(),
                    raw: "foo".to_string()
                }
            )),
            factor().parse(input)
        );

        let input = vec![(ID, "foo".to_string())];
        assert_eq!(
            Ok((
                vec![],
                Id {
                    kind: ID,
                    name: "foo".to_string()
                }
            )),
            factor().parse(input)
        );

        let input = vec![
            (OPEN_PAREN, "(".to_string()),
            (ID, "foo".to_string()),
            (CLOSE_PAREN, ")".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                Id {
                    kind: ID,
                    name: "foo".to_string()
                }
            )),
            factor().parse(input)
        );
    }

    fn issue1() {
        let foo = Box::new(Id {
            kind: ID,
            name: "foo".to_string(),
        });
        let bar = Box::new(Id {
            kind: ID,
            name: "bar".to_string(),
        });

        // foo().bar
        let input = vec![
            (ID, "foo".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (CLOSE_PAREN, ")".to_string()),
            (DOT, ".".to_string()),
            (ID, "bar".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                ValueAccessExpr {
                    kind: VALUE_ACCESS_EXPR,
                    path: vec![
                        Box::new(FunctionCallExpr {
                            kind: FUNCTION_CALL_EXPR,
                            callee: foo.clone(),
                            args: vec![]
                        }),
                        bar.clone()
                    ]
                }
            )),
            expr().parse(input)
        );
    }
}
