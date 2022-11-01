use crate::ast::grammar::{
    bit_calc_op, calc_op, comparison_op, empty_node,
    literal, minus_minus, plus_plus, single_token,
};
use crate::ast::{
    AssignmentExpr, BinaryExpr, Empty, FunctionCallExpr,
    FunctionDeclaExpr, Id, Node, ReturnExpr, TernaryExpr,
    UnaryExpr, ValueAccessExpr, VariableDeclaExpr,
};
use crate::lex::TokenStream;
use crate::syntax_kind::{
    SyntaxKind, ASSIGNMENT_EXPR, BINARY_EXPR, DEFINATOR,
    DELETE_KW, EMPTY, FUNCTION_CALL_EXPR, FUNCTION_DECLA,
    FUNCTION_KW, ID, INSTANCE_OF_KW, IN_KW, RETURN_EXPR,
    RETURN_KW, TERNARY_EXPR, TYPE_OF_KW, UNARY_EXPR,
    VALUE_ACCESS_EXPR, VARIABLE_DECLA,
};
use crate::T;
use shared::parser_combiner::{
    between, chainl1, choice, either, left, one_or_more,
    pair, right, seq_by, zero_or_more, BoxedParser, Parser,
};

/// Expr -> Literal
///       | UnaryExpr
///       | BinaryExpr
///       | TernaryExpr
///       | ValueAccessExpr
///       | FunctionCallExpr
///       | AssignmentExpr
///       | ReturnExpr
pub fn expr() -> impl Parser<'static, TokenStream, Node> {
    choice(vec![
        BoxedParser::new(literal()),
        BoxedParser::new(unary_expr()),
        BoxedParser::new(binary_expr()),
        // BoxedParser::new(assignment_expr()),
    ])
}

/// UnaryExpr -> Literal UnaryExpr1
///            | ("++" | "--" | "!" | TYPE_OF | DELETE) Expr
pub fn unary_expr(
) -> impl Parser<'static, TokenStream, Node> {
    fn build_node(
        expr: Node,
        op_list: Vec<SyntaxKind>,
        cur: usize,
    ) -> Node {
        if let Some(op) = op_list.get(cur) {
            UnaryExpr {
                kind: UNARY_EXPR,
                prefix: false,
                op: *op,
                expr: Box::new(build_node(
                    expr,
                    op_list,
                    cur + 1,
                )),
            }
        } else {
            expr
        }
    }

    either(
        literal().and_then(|expr| {
            unary_expr1().map(move |op_list| {
                build_node(expr.to_owned(), op_list, 0)
            })
        }),
        choice(vec![
            BoxedParser::new(plus_plus()),
            BoxedParser::new(minus_minus()),
            BoxedParser::new(single_token(T!["!"])),
            BoxedParser::new(single_token(TYPE_OF_KW)),
            BoxedParser::new(single_token(DELETE_KW)),
        ])
        .and_then(|(op, _)| {
            expr().map(move |expr| UnaryExpr {
                kind: UNARY_EXPR,
                prefix: true,
                op,
                expr: Box::new(expr.to_owned()),
            })
        }),
    )
}

/// UnaryExpr1 -> ("++" | "--") UnaryExpr1 | <empty>
/// UnaryExpr1 -> ("++" | "--")*
pub fn unary_expr1(
) -> impl Parser<'static, TokenStream, Vec<SyntaxKind>> {
    zero_or_more(
        either(plus_plus(), minus_minus())
            .map(|(kind, _)| kind),
    )
}

/// BinaryExpr -> Literal BinaryExpr1
pub fn binary_expr(
) -> impl Parser<'static, TokenStream, Node> {
    literal().and_then(|left| {
        binary_expr1().map(move |(op, right)| match right {
            Empty => left.to_owned(),
            _ => BinaryExpr {
                kind: BINARY_EXPR,
                left: Box::new(left.to_owned()),
                op,
                right: Box::new(right),
            },
        })
    })
}

/// BinaryExpr1 -> ( "+" | "-" | "/" | "*"
///                | "&" | "|" | "^" | "~" | "<<" | ">>" | ">>>"
///                | ">" | ">=" | "<" | "<=" | "==" | "==="
///                | INSTANCE_OF | IN
///                  Expr BinaryExpr1 )
///                | <empty>
pub fn binary_expr1(
) -> impl Parser<'static, TokenStream, (SyntaxKind, Node)> {
    either(
        choice(vec![
            BoxedParser::new(calc_op()),
            BoxedParser::new(bit_calc_op()),
            BoxedParser::new(comparison_op()),
            BoxedParser::new(single_token(INSTANCE_OF_KW)),
            BoxedParser::new(single_token(IN_KW)),
        ])
        .and_then(|(op, _)| {
            expr().map(move |left| (op, left))
        })
        .and_then(move |(op, left)| {
            binary_expr1().map(move |right| match right {
                (EMPTY, Empty) => (op, left.to_owned()),
                _ => right,
            })
        }),
        empty_node().map(|n| (EMPTY, Empty)),
    )
}

/// TernaryExpr -> BianryExpr "?" Expr ":" Expr
pub fn ternary_expr(
) -> impl Parser<'static, TokenStream, Node> {
    left(binary_expr(), single_token(T!["?"]))
        .and_then(|condition| {
            expr().map(move |then_expr| {
                (condition.to_owned(), then_expr)
            })
        })
        .and_then(|(condition, then_expr)| {
            right(single_token(T![":"]), expr()).map(
                move |else_expr| TernaryExpr {
                    kind: TERNARY_EXPR,
                    condition: Box::new(
                        condition.to_owned(),
                    ),
                    then_expr: Box::new(
                        then_expr.to_owned(),
                    ),
                    else_expr: Box::new(else_expr),
                },
            )
        })
}

/// ValueAccessExpr -> ID ("." ID)+
pub fn value_access_expr(
) -> impl Parser<'static, TokenStream, Node> {
    chainl1(single_token(ID), single_token(T!["."]))
        .map(|res| {
            res.iter()
                .map(|(_, path)| path.to_string())
                .collect::<Vec<String>>()
        })
        .map(|path| ValueAccessExpr {
            kind: VALUE_ACCESS_EXPR,
            path,
        })
}

/// AssignmentExpr -> Literal AssignmentExpr1
pub fn assignment_expr(
) -> impl Parser<'static, TokenStream, Node> {
    literal().and_then(|left| {
        assignment_expr1().map(move |right| match right {
            Empty => left.to_owned(),
            _ => AssignmentExpr {
                kind: ASSIGNMENT_EXPR,
                left: Box::new(left.to_owned()),
                right: Box::new(right),
            },
        })
    })
}

/// AssignmentExpr1 -> ("=" AssignmentExpr AssignmentExpr1) | <empty>
pub fn assignment_expr1(
) -> impl Parser<'static, TokenStream, Node> {
    either(
        single_token(T!["="])
            .and_then(|_| assignment_expr())
            .and_then(|left| {
                assignment_expr1().map(move |right| {
                    match right {
                        Empty => left.to_owned(),
                        _ => AssignmentExpr {
                            kind: ASSIGNMENT_EXPR,
                            left: Box::new(left.to_owned()),
                            right: Box::new(right),
                        },
                    }
                })
            }),
        empty_node(),
    )
}

/// FunctionCallExpr -> (ID | ValueAccessExpr) (FunctionCallExpr1)+
pub fn function_call_expr(
) -> impl Parser<'static, TokenStream, Node> {
    fn build_node(
        callee: Node,
        args_list: Vec<Vec<Box<Node>>>,
        cur: usize,
    ) -> Node {
        if let Some(args) = args_list.get(cur) {
            FunctionCallExpr {
                kind: FUNCTION_CALL_EXPR,
                callee: Box::new(build_node(
                    callee,
                    args_list.to_owned(),
                    cur + 1,
                )),
                args: args.to_owned(),
            }
        } else {
            callee
        }
    }

    either(
        single_token(ID)
            .map(|(_, name)| Id { kind: ID, name }),
        value_access_expr(),
    )
    .and_then(|callee| {
        one_or_more(function_call_expr1())
            .map(move |args| (callee.to_owned(), args))
    })
    .map(|(callee, args)| {
        build_node(callee, args.to_owned(), 0)
    })
}

/// FunctionCallExpr1 -> "(" (Expr, (, Expr)*)? ")" FunctionCallExpr1 | <empty>
/// FunctionCallExpr1 -> "(" (Expr, (, Expr)*)? ")"
pub fn function_call_expr1(
) -> impl Parser<'static, TokenStream, Vec<Box<Node>>> {
    between(
        single_token(T!["("]),
        seq_by(expr(), single_token(T![","])),
        single_token(T![")"]),
    )
    .map(|args| {
        args.iter()
            .map(|arg| Box::new(arg.to_owned()))
            .collect()
    })
}

/// ReturnExpr -> "return" Expr
pub fn return_decla(
) -> impl Parser<'static, TokenStream, Node> {
    right(single_token(RETURN_KW), expr()).map(|expr| {
        ReturnExpr {
            kind: RETURN_EXPR,
            expr: Box::new(expr),
        }
    })
}

/// Declaration -> VariableDecla | FunctionDecla
pub fn declaration(
) -> impl Parser<'static, TokenStream, Node> {
    either(variable_decla(), function_decla())
}

/// VariableDecla -> DEFINTOR ID "=" Expr
pub fn variable_decla(
) -> impl Parser<'static, TokenStream, Node> {
    left(
        pair(single_token(DEFINATOR), single_token(ID)),
        single_token(T!["="]),
    )
    .and_then(|(defintor, name)| {
        expr().map(move |init| {
            let defintor = defintor.to_owned().1;
            let name = name.to_owned().1;
            VariableDeclaExpr {
                kind: VARIABLE_DECLA,
                defintor: defintor.to_owned(),
                name: name.to_owned(),
                init: Box::new(init),
            }
        })
    })
}

/// FunctionDecla -> FUNCTION ID "(" (ID ("," ID)*)? ")" "{" Stat* "}"
pub fn function_decla(
) -> impl Parser<'static, TokenStream, Node> {
    right(single_token(FUNCTION_KW), single_token(ID))
        .and_then(|(_, name)| {
            between(
                single_token(T!["("]),
                seq_by(
                    single_token(ID),
                    single_token(T![","]),
                ),
                single_token(T![")"]),
            )
            .map(move |args| {
                (
                    name.to_owned(),
                    args.iter()
                        .map(|(_, text)| text.to_string())
                        .collect::<Vec<String>>(),
                )
            })
        })
        .and_then(|(name, args)| {
            between(
                single_token(T!["{"]),
                // TODO stat
                zero_or_more(expr()),
                single_token(T!["}"]),
            )
            .map(move |body| {
                FunctionDeclaExpr {
                    kind: FUNCTION_DECLA,
                    name: name.to_owned(),
                    args: args.to_owned(),
                    body: body
                        .iter()
                        .map(|n| Box::new(n.to_owned()))
                        .collect(),
                }
            })
        })
}

#[cfg(test)]
mod tests {
    use crate::ast::grammar::expression::assignment_expr;
    use crate::ast::grammar::{
        binary_expr, expr, function_call_expr,
        function_call_expr1, function_decla, ternary_expr,
        unary_expr, value_access_expr, variable_decla,
    };
    use crate::ast::{
        AssignmentExpr, BinaryExpr, FunctionCallExpr,
        FunctionDeclaExpr, Id, NumberLiteral,
        StringLiteral, TernaryExpr, UnaryExpr,
        ValueAccessExpr, VariableDeclaExpr,
    };
    use crate::syntax_kind::{
        ASSIGNMENT_EXPR, BINARY_EXPR, CLOSE_BRACE,
        CLOSE_PAREN, COLON, COMMA, DEFINATOR, DOT, EQ,
        EQEQ, FUNCTION_CALL_EXPR, FUNCTION_DECLA,
        FUNCTION_KW, ID, NUMBER, OPEN_BRACE, OPEN_PAREN,
        PLUS, PLUSPLUS, QUESTION, STRING, TERNARY_EXPR,
        UNARY_EXPR, VALUE_ACCESS_EXPR, VARIABLE_DECLA,
    };
    use shared::parser_combiner::Parser;

    #[test]
    fn test_unary_expr() {
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
                    expr: Box::new(NumberLiteral {
                        kind: NUMBER,
                        value: 1,
                        raw: "1".to_string()
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
                        expr: Box::new(NumberLiteral {
                            kind: NUMBER,
                            value: 1,
                            raw: "1".to_string()
                        })
                    })
                }
            )),
            unary_expr().parse(input)
        );

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
            unary_expr().parse(input)
        )
    }

    #[test]
    fn test_binary_expr() {
        let input = vec![
            (NUMBER, "1".to_string()),
            (PLUS, "+".to_string()),
            (NUMBER, "1".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                BinaryExpr {
                    kind: BINARY_EXPR,
                    left: Box::new(NumberLiteral {
                        kind: NUMBER,
                        value: 1,
                        raw: "1".to_string()
                    }),
                    op: PLUS,
                    right: Box::new(NumberLiteral {
                        kind: NUMBER,
                        value: 1,
                        raw: "1".to_string()
                    })
                }
            )),
            binary_expr().parse(input)
        );

        let input = vec![
            (NUMBER, "1".to_string()),
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
                    left: Box::new(NumberLiteral {
                        kind: NUMBER,
                        value: 1,
                        raw: "1".to_string()
                    }),
                    op: PLUS,
                    right: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: Box::new(NumberLiteral {
                            kind: NUMBER,
                            value: 1,
                            raw: "1".to_string()
                        }),
                        op: PLUS,
                        right: Box::new(NumberLiteral {
                            kind: NUMBER,
                            value: 1,
                            raw: "1".to_string()
                        })
                    })
                }
            )),
            binary_expr().parse(input)
        );
    }

    #[test]
    fn test_ternary_expr() {
        let input = vec![
            (NUMBER, "1".to_string()),
            (EQ, "=".to_string()),
            (EQ, "=".to_string()),
            (NUMBER, "1".to_string()),
            (QUESTION, "?".to_string()),
            (STRING, "foo".to_string()),
            (COLON, ":".to_string()),
            (STRING, "bar".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                TernaryExpr {
                    kind: TERNARY_EXPR,
                    condition: Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: Box::new(NumberLiteral {
                            kind: NUMBER,
                            value: 1,
                            raw: "1".to_string()
                        }),
                        op: EQEQ,
                        right: Box::new(NumberLiteral {
                            kind: NUMBER,
                            value: 1,
                            raw: "1".to_string()
                        })
                    }),
                    then_expr: Box::new(StringLiteral {
                        kind: STRING,
                        value: "foo".to_string(),
                        raw: "foo".to_string()
                    }),
                    else_expr: Box::new(StringLiteral {
                        kind: STRING,
                        value: "bar".to_string(),
                        raw: "bar".to_string()
                    })
                }
            )),
            ternary_expr().parse(input)
        )
    }

    #[test]
    fn test_assignment_expr() {
        let input = vec![
            (STRING, "foo".to_string()),
            (EQ, "=".to_string()),
            (STRING, "bar".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                AssignmentExpr {
                    kind: ASSIGNMENT_EXPR,
                    left: Box::new(StringLiteral {
                        kind: STRING,
                        value: "foo".to_string(),
                        raw: "foo".to_string()
                    }),
                    right: Box::new(StringLiteral {
                        kind: STRING,
                        value: "bar".to_string(),
                        raw: "bar".to_string()
                    })
                }
            )),
            assignment_expr().parse(input)
        );

        let input = vec![
            (STRING, "1".to_string()),
            (EQ, "=".to_string()),
            (STRING, "2".to_string()),
            (EQ, "=".to_string()),
            (STRING, "3".to_string()),
            (EQ, "=".to_string()),
            (STRING, "4".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                AssignmentExpr {
                    kind: ASSIGNMENT_EXPR,
                    left: Box::new(StringLiteral {
                        kind: STRING,
                        value: "1".to_string(),
                        raw: "1".to_string()
                    }),
                    right: Box::new(AssignmentExpr {
                        kind: ASSIGNMENT_EXPR,
                        left: Box::new(StringLiteral {
                            kind: STRING,
                            value: "2".to_string(),
                            raw: "2".to_string()
                        }),
                        right: Box::new(AssignmentExpr {
                            kind: ASSIGNMENT_EXPR,
                            left: Box::new(StringLiteral {
                                kind: STRING,
                                value: "3".to_string(),
                                raw: "3".to_string()
                            }),
                            right: Box::new(
                                StringLiteral {
                                    kind: STRING,
                                    value: "4".to_string(),
                                    raw: "4".to_string()
                                }
                            )
                        })
                    })
                }
            )),
            assignment_expr().parse(input)
        );

        let input = vec![
            (EQ, "=".to_string()),
            (STRING, "world".to_string()),
        ];
        assert_eq!(
            Err(vec![
                (EQ, "=".to_string()),
                (STRING, "world".to_string()),
            ]),
            assignment_expr().parse(input)
        )
    }

    #[test]
    fn test_value_access_expr() {
        let input = vec![
            (ID, "foo".to_string()),
            (DOT, ".".to_string()),
            (ID, "bar".to_string()),
            (DOT, ".".to_string()),
            (ID, "baz".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                ValueAccessExpr {
                    kind: VALUE_ACCESS_EXPR,
                    path: vec![
                        "foo".to_string(),
                        "bar".to_string(),
                        "baz".to_string()
                    ]
                }
            )),
            value_access_expr().parse(input)
        );

        let input = vec![(ID, "foo".to_string())];
        assert_eq!(
            Err(vec![(ID, "foo".to_string())]),
            value_access_expr().parse(input)
        );
    }

    #[test]
    fn test_function_call_expr() {
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
            (NUMBER, "1".to_string()),
            (COMMA, ",".to_string()),
            (STRING, "1".to_string()),
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
                        Box::new(NumberLiteral {
                            kind: NUMBER,
                            value: 1,
                            raw: "1".to_string()
                        }),
                        Box::new(StringLiteral {
                            kind: STRING,
                            value: "1".to_string(),
                            raw: "1".to_string()
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
    }

    #[test]
    fn test_value_declaration() {
        let input = vec![
            (DEFINATOR, "const".to_string()),
            (ID, "foo".to_string()),
            (EQ, "=".to_string()),
            (STRING, "bar".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                VariableDeclaExpr {
                    kind: VARIABLE_DECLA,
                    defintor: "const".to_string(),
                    name: "foo".to_string(),
                    init: Box::new(StringLiteral {
                        kind: STRING,
                        value: "bar".to_string(),
                        raw: "bar".to_string()
                    })
                }
            )),
            variable_decla().parse(input)
        );
    }

    #[test]
    fn test_functon_delcaration() {
        let input = vec![
            (FUNCTION_KW, "function".to_string()),
            (ID, "foo".to_string()),
            (OPEN_PAREN, "(".to_string()),
            (CLOSE_PAREN, ")".to_string()),
            (OPEN_BRACE, "{".to_string()),
            (CLOSE_BRACE, "}".to_string()),
        ];
        assert_eq!(
            Ok((
                vec![],
                FunctionDeclaExpr {
                    kind: FUNCTION_DECLA,
                    name: "foo".to_string(),
                    args: vec![],
                    body: vec![]
                }
            )),
            function_decla().parse(input)
        )
    }

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

    #[test]
    fn issue_2() {
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
                            "foo".to_string(),
                            "bar".to_string()
                        ]
                    }),
                    args: vec![]
                }
            )),
            function_call_expr().parse(input)
        )
    }
}
