use crate::ast::grammar::{
    binary_expr, block, empty_node, expr, expr_node, id,
    list, single_token,
};
use crate::ast::{
    CaseStatement, DefaultStatement, Empty, ForStatement,
    FunctionDeclaStatement, Id, IfStatement, Node,
    SwitchStatement, VariableDeclaStatement,
    WhileStatement,
};
use crate::lex::TokenStream;
use crate::syntax_kind::{
    BREAK_KW, CASE_KW, CASE_STAT, DEFAULT_CASE_STAT,
    DEFAULT_KW, DEFINATOR, ELSE_KW, FOR_KW, FOR_STAT,
    FUNCTION_DECLA_STAT, FUNCTION_KW, ID, IF_KW, IF_STAT,
    SWITCH_KW, SWITCH_STAT, VARIABLE_DECLA_STAT, WHILE_KW,
    WHILE_STAT,
};
use crate::T;
use shared::parser_combiner::{
    between, choice, either, left, one_or_more, pair,
    right, seq_by, series, zero_or_more, zero_or_one,
    BoxedParser, Parser,
};

pub fn stat_node(
) -> impl Parser<'static, TokenStream, Box<Node>> {
    stat().map(|n| Box::new(n))
}

/// Stat -> Expr (";")?
///       | DeclarationStat
///       | ConditionalStat
///       | CycleStat
pub fn stat() -> impl Parser<'static, TokenStream, Node> {
    choice(vec![
        BoxedParser::new(expr().and_then(|expr| {
            zero_or_one(single_token(T![";"]))
                .map(move |_| expr.to_owned())
        })),
        BoxedParser::new(declaration_stat()),
        BoxedParser::new(condition_stat()),
        BoxedParser::new(cycle_stat()),
    ])
}

/// DeclarationStat -> FunctionDeclara
///                  | VariableDeclara (";")?
pub fn declaration_stat(
) -> impl Parser<'static, TokenStream, Node> {
    either(
        function_decla(),
        left(
            variable_decla(),
            zero_or_one(single_token(T![";"])),
        ),
    )
}

/// FunctionDecla -> FUNCTION ID "(" (ID ("," ID)*)? ")" Block
pub fn function_decla(
) -> impl Parser<'static, TokenStream, Node> {
    right(single_token(FUNCTION_KW), id())
        .and_then(|name| {
            between(
                single_token(T!["("]),
                seq_by(
                    single_token(ID).map(|(_, name)| {
                        Box::new(Id { kind: ID, name })
                    }),
                    single_token(T![","]),
                ),
                single_token(T![")"]),
            )
            .map(move |args| (name.to_owned(), args))
        })
        .and_then(|(name, args)| {
            stat_block().map(move |body| {
                FunctionDeclaStatement {
                    kind: FUNCTION_DECLA_STAT,
                    name: name.to_owned(),
                    args: args.to_owned(),
                    body,
                }
            })
        })
}

/// VariableDecla -> DEFINTOR ID "=" Expr
pub fn variable_decla(
) -> impl Parser<'static, TokenStream, Node> {
    pair(single_token(DEFINATOR), id()).and_then(
        |(definator, name)| {
            right(single_token(T!["="]), expr()).map(
                move |init| {
                    let (_, definator) =
                        definator.to_owned();

                    VariableDeclaStatement {
                        kind: VARIABLE_DECLA_STAT,
                        definator,
                        name: name.to_owned(),
                        init: Box::new(init),
                    }
                },
            )
        },
    )
}

/// ConditionStat -> IfStat | SwitchStat
pub fn condition_stat(
) -> impl Parser<'static, TokenStream, Node> {
    either(if_stat(), switch_stat())
}

/// IfStat -> IF IfStat1 (ElseIfStat)* ElseStat?
pub fn if_stat() -> impl Parser<'static, TokenStream, Node>
{
    fn build_node(
        mut case_list: Vec<(Box<Node>, Vec<Box<Node>>)>,
        else_stat: Node,
    ) -> Node {
        if let Some((expr, then_block)) = case_list.pop() {
            IfStatement {
                kind: IF_STAT,
                expr,
                then_block,
                else_block: Box::new(build_node(
                    case_list, else_stat,
                )),
            }
        } else {
            else_stat
        }
    }

    right(single_token(IF_KW), if_stat1())
        .and_then(|expr| {
            zero_or_more(else_if_stat()).map(
                move |case_list| {
                    let mut case_list = case_list;
                    case_list.insert(0, expr.to_owned());
                    case_list
                },
            )
        })
        .and_then(|case_list| {
            let mut case_list = case_list.to_owned();
            case_list.reverse();
            zero_or_one(else_stat()).map(move |else_stat| {
                build_node(
                    case_list.to_owned(),
                    match else_stat {
                        None => Empty,
                        Some(n) => n,
                    },
                )
            })
        })
}

/// IfStat1 -> "(" Expr ")" Block
pub fn if_stat1(
) -> impl Parser<'static, TokenStream, (Box<Node>, Vec<Box<Node>>)>
{
    between(
        single_token(T!["("]),
        expr_node(),
        single_token(T![")"]),
    )
    .and_then(|expr| {
        stat_block().map(move |then_block| {
            (expr.to_owned(), then_block)
        })
    })
}

/// ElseIfStat -> ELSE IF IfStat1
pub fn else_if_stat(
) -> impl Parser<'static, TokenStream, (Box<Node>, Vec<Box<Node>>)>
{
    right(
        single_token(ELSE_KW)
            .and_then(|_| single_token(IF_KW)),
        if_stat1(),
    )
}

/// ElseStat -> ELSE Block
pub fn else_stat() -> impl Parser<'static, TokenStream, Node>
{
    right(single_token(ELSE_KW), stat_block()).map(
        |then_block| IfStatement {
            kind: IF_STAT,
            expr: Box::new(Empty),
            then_block,
            else_block: Box::new(Empty),
        },
    )
}

/// SwitchStat -> SWITH "(" Expr ")" SwitchBlock
pub fn switch_stat(
) -> impl Parser<'static, TokenStream, Node> {
    right(
        single_token(SWITCH_KW),
        between(
            single_token(T!["("]),
            expr_node(),
            single_token(T![")"]),
        ),
    )
    .and_then(|expr| {
        switch_block().map(move |then_block| {
            SwitchStatement {
                kind: SWITCH_STAT,
                expr: expr.to_owned(),
                then_block,
            }
        })
    })
}

/// SwitchBlock -> "{" ((CaseStat)* DefaultStat? )? "}"
pub fn switch_block(
) -> impl Parser<'static, TokenStream, Vec<Box<Node>>> {
    between(
        single_token(T!["{"]),
        zero_or_one(
            one_or_more(
                case_stat().map(|n| Box::new(n.to_owned())),
            )
            .and_then(|case_list| {
                zero_or_one(
                    default_stat()
                        .map(|n| Box::new(n.to_owned())),
                )
                .map(move |default_case| {
                    match default_case {
                        None => case_list.to_owned(),
                        Some(default_case_node) => {
                            let mut case_list =
                                case_list.to_owned();
                            case_list
                                .push(default_case_node);
                            case_list
                        }
                    }
                })
            }),
        ),
        single_token(T!["}"]),
    )
    .map(|case_list| match case_list {
        None => vec![],
        Some(case_list) => case_list,
    })
}

/// SwitchStat1 -> ":" Stat* BREAK? ";"?
pub fn switch_stat1(
) -> impl Parser<'static, TokenStream, (bool, Vec<Box<Node>>)>
{
    single_token(T![":"])
        .and_then(|_| {
            zero_or_more(stat().map(|n| Box::new(n)))
        })
        .and_then(|then_block| {
            zero_or_one(single_token(BREAK_KW)).map(
                move |has_break| {
                    (
                        has_break.is_some(),
                        then_block.to_owned(),
                    )
                },
            )
        })
        .and_then(|res| {
            zero_or_one(single_token(T![";"]))
                .map(move |_| res.to_owned())
        })
}

/// CaseStat -> CASE Expr SwitchStat1
pub fn case_stat() -> impl Parser<'static, TokenStream, Node>
{
    right(single_token(CASE_KW), expr_node()).and_then(
        |expr| {
            switch_stat1().map(
                move |(has_break, then_block)| {
                    CaseStatement {
                        kind: CASE_STAT,
                        expr: expr.to_owned(),
                        has_break,
                        then_block,
                    }
                },
            )
        },
    )
}

/// DefaultStat -> DEFAULT SwitchStat1
pub fn default_stat(
) -> impl Parser<'static, TokenStream, Node> {
    right(single_token(DEFAULT_KW), switch_stat1()).map(
        |(has_break, then_block)| DefaultStatement {
            kind: DEFAULT_CASE_STAT,
            has_break,
            then_block,
        },
    )
}

/// CycleStat -> ForStat | While Stat
pub fn cycle_stat(
) -> impl Parser<'static, TokenStream, Node> {
    either(for_stat(), while_stat())
}

/// ForStat -> FOR "(" ForStatArgs ")" Block
pub fn for_stat() -> impl Parser<'static, TokenStream, Node>
{
    right(
        single_token(FOR_KW),
        between(
            single_token(T!["("]),
            for_stat_args(),
            single_token(T![")"]),
        ),
    )
    .and_then(|(init, condition, step)| {
        stat_block().map(move |then_block| ForStatement {
            kind: FOR_STAT,
            init: init.to_owned(),
            condition: condition.to_owned(),
            step: step.to_owned(),
            then_block,
        })
    })
}

/// ForStatArgs -> VariableDecla? ";" BinaryExpr? ";" Expr?
pub fn for_stat_args() -> impl Parser<
    'static,
    TokenStream,
    (Box<Node>, Box<Node>, Box<Node>),
> {
    series(vec![
        BoxedParser::new(left(
            zero_or_one(variable_decla()),
            single_token(T![";"]),
        )),
        BoxedParser::new(left(
            zero_or_one(binary_expr()),
            single_token(T![";"]),
        )),
        BoxedParser::new(zero_or_one(expr())),
    ])
    .map(|args| {
        let unwrap_node =
            |maybe_node: &Option<Node>| match maybe_node {
                None => Box::new(Empty),
                Some(n) => Box::new(n.to_owned()),
            };
        (
            unwrap_node(args.get(0).unwrap()),
            unwrap_node(args.get(1).unwrap()),
            unwrap_node(args.get(2).unwrap()),
        )
    })
}

/// WhileStat -> WHILE "(" Expr ")" Block
pub fn while_stat(
) -> impl Parser<'static, TokenStream, Node> {
    single_token(WHILE_KW).and_then(|_| {
        between(
            single_token(T!["("]),
            expr_node(),
            single_token(T![")"]),
        )
        .and_then(|expr| {
            stat_block().map(move |then_block| {
                WhileStatement {
                    kind: WHILE_STAT,
                    condition: expr.to_owned(),
                    then_block,
                }
            })
        })
    })
}

pub fn stat_block(
) -> impl Parser<'static, TokenStream, Vec<Box<Node>>> {
    block(stat()).map(|node_list| {
        node_list
            .iter()
            .map(|n| Box::new(n.to_owned()))
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use crate::ast::grammar::statement::{
        declaration_stat, for_stat, for_stat_args, if_stat,
        stat, switch_stat, while_stat,
    };
    use crate::ast::{
        AssignmentExpr, BinaryExpr, CaseStatement,
        DefaultStatement, Empty, ForStatement,
        FunctionDeclaStatement, Id, IfStatement, Node,
        NumberLiteral, SwitchStatement, UnaryExpr,
        VariableDeclaStatement, WhileStatement,
    };
    use crate::lex;
    use crate::syntax_kind::{
        ASSIGNMENT_EXPR, BINARY_EXPR, CASE_STAT,
        DEFAULT_CASE_STAT, EQEQ, EQEQEQ, FOR_STAT,
        FUNCTION_DECLA_STAT, ID, IF_STAT, LTEQ, NUMBER,
        PLUSPLUS, SWITCH_STAT, UNARY_EXPR,
        VARIABLE_DECLA_STAT, WHILE_STAT,
    };
    use shared::parser_combiner::Parser;

    fn get_id() -> (Box<Node>, Box<Node>, Box<Node>) {
        let foo = Box::new(Id {
            kind: ID,
            name: "foo".to_string(),
        });
        let bar = Box::new(Id {
            kind: ID,
            name: "bar".to_string(),
        });
        let baz = Box::new(Id {
            kind: ID,
            name: "baz".to_string(),
        });
        (foo, bar, baz)
    }
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
    fn get_unary_expr() -> Box<Node> {
        let (one, _, _, _) = get_number();
        Box::new(UnaryExpr {
            kind: UNARY_EXPR,
            prefix: false,
            op: PLUSPLUS,
            expr: one.clone(),
        })
    }
    fn get_binary_expr() -> Box<Node> {
        let (one, _, _, _) = get_number();
        Box::new(BinaryExpr {
            kind: BINARY_EXPR,
            left: one.clone(),
            op: EQEQ,
            right: one.clone(),
        })
    }

    #[test]
    fn test_stat() {
        let empty_node = Box::new(Empty);
        let (foo, bar, baz) = get_id();
        let (one, two, three, _) = get_number();
        let a_id = Box::new(Id {
            kind: ID,
            name: "a".to_string(),
        });
        let b_id = Box::new(Id {
            kind: ID,
            name: "b".to_string(),
        });
        let c_id = Box::new(Id {
            kind: ID,
            name: "c".to_string(),
        });
        let bar_eqeq_one = Box::new(BinaryExpr {
            kind: BINARY_EXPR,
            left: bar.clone(),
            op: EQEQ,
            right: one.clone(),
        });
        let baz_eqeqeq_two = Box::new(BinaryExpr {
            kind: BINARY_EXPR,
            left: baz.clone(),
            op: EQEQEQ,
            right: two.clone(),
        });
        let let_a_one = Box::new(VariableDeclaStatement {
            kind: VARIABLE_DECLA_STAT,
            definator: "let".to_string(),
            name: a_id.clone(),
            init: one.clone(),
        });
        let const_b_two =
            Box::new(VariableDeclaStatement {
                kind: VARIABLE_DECLA_STAT,
                definator: "const".to_string(),
                name: b_id.clone(),
                init: two.clone(),
            });
        let var_c_three =
            Box::new(VariableDeclaStatement {
                kind: VARIABLE_DECLA_STAT,
                definator: "var".to_string(),
                name: c_id.clone(),
                init: three.clone(),
            });
        let a_eqeq_one = Box::new(BinaryExpr {
            kind: BINARY_EXPR,
            left: a_id.clone(),
            op: EQEQ,
            right: one.clone(),
        });
        let a_lteq_one = Box::new(BinaryExpr {
            kind: BINARY_EXPR,
            left: a_id.clone(),
            op: LTEQ,
            right: one.clone(),
        });
        let a_plusplus = Box::new(UnaryExpr {
            kind: UNARY_EXPR,
            prefix: false,
            op: PLUSPLUS,
            expr: a_id.clone(),
        });

        let input = lex(r#"
        function foo( bar, baz ) {
            if (bar == 1) {
                let a = 1;
            } else if (baz === 2) {
                const b = 2; 
            } else {
                var c = 3;
            }
            
            switch (bar) {
                case 1:
                    let a = 1;
                break;
                
                case 2:
                    const b = 2;
                break;
                
                default:
                    var c = 3;
                break;
            } 
            
            while(a == 1) {
                let a = 1; 
            }
            
            for(let a = 1; a <= 1; a++) {
                let a = 1; 
            }
        }
        "#);
        assert_eq!(
            Ok((
                vec![],
                FunctionDeclaStatement {
                    kind: FUNCTION_DECLA_STAT,
                    name: foo.clone(),
                    args: vec![bar.clone(), baz.clone()],
                    body: vec![
                        Box::new(IfStatement {
                            kind: IF_STAT,
                            expr: bar_eqeq_one.clone(),
                            then_block: vec![
                                let_a_one.clone()
                            ],
                            else_block: Box::new(
                                IfStatement {
                                    kind: IF_STAT,
                                    expr: baz_eqeqeq_two
                                        .clone(),
                                    then_block: vec![
                                        const_b_two.clone()
                                    ],
                                    else_block: Box::new(
                                        IfStatement {
                                            kind: IF_STAT,
                                            expr:
                                                empty_node
                                                    .clone(),
                                            then_block: vec![
                                                var_c_three
                                                    .clone(
                                                    )
                                            ],
                                            else_block:
                                                empty_node
                                                    .clone()
                                        }
                                    )
                                }
                            )
                        }),
                        Box::new(SwitchStatement {
                            kind: SWITCH_STAT,
                            expr: bar.clone(),
                            then_block: vec![
                                Box::new(CaseStatement {
                                    kind: CASE_STAT,
                                    expr: one.clone(),
                                    has_break: true,
                                    then_block: vec![
                                        let_a_one.clone()
                                    ]
                                }),
                                Box::new(CaseStatement {
                                    kind: CASE_STAT,
                                    expr: two.clone(),
                                    has_break: true,
                                    then_block: vec![
                                        const_b_two.clone()
                                    ]
                                }),
                                Box::new(DefaultStatement {
                                    kind: DEFAULT_CASE_STAT,
                                    has_break: true,
                                    then_block: vec![
                                        var_c_three.clone()
                                    ]
                                })
                            ]
                        }),
                        Box::new(WhileStatement {
                            kind: WHILE_STAT,
                            condition: a_eqeq_one.clone(),
                            then_block: vec![
                                let_a_one.clone()
                            ]
                        }),
                        Box::new(ForStatement {
                            kind: FOR_STAT,
                            init: let_a_one.clone(),
                            condition: a_lteq_one.clone(),
                            step: a_plusplus.clone(),
                            then_block: vec![
                                let_a_one.clone()
                            ]
                        })
                    ]
                }
            )),
            stat().parse(input)
        )
    }

    #[test]
    fn test_declaration() {
        let (foo, bar, baz) = get_id();
        let (one, _, _, _) = get_number();
        let one_eqeq_one = get_binary_expr();

        let input = lex("const foo = 1");
        assert_eq!(
            Ok((
                vec![],
                VariableDeclaStatement {
                    kind: VARIABLE_DECLA_STAT,
                    definator: "const".to_string(),
                    name: foo.clone(),
                    init: one.clone()
                }
            )),
            declaration_stat().parse(input)
        );

        let input = lex("const foo = 1 == 1");
        assert_eq!(
            Ok((
                vec![],
                VariableDeclaStatement {
                    kind: VARIABLE_DECLA_STAT,
                    definator: "const".to_string(),
                    name: foo.clone(),
                    init: one_eqeq_one.clone()
                }
            )),
            declaration_stat().parse(input)
        );

        let input = lex("function foo() {}");
        assert_eq!(
            Ok((
                vec![],
                FunctionDeclaStatement {
                    kind: FUNCTION_DECLA_STAT,
                    name: foo.clone(),
                    args: vec![],
                    body: vec![]
                }
            )),
            declaration_stat().parse(input)
        );

        let input = lex("function foo( bar, baz ) {}");
        assert_eq!(
            Ok((
                vec![],
                FunctionDeclaStatement {
                    kind: FUNCTION_DECLA_STAT,
                    name: foo.clone(),
                    args: vec![bar.clone(), baz.clone()],
                    body: vec![]
                }
            )),
            declaration_stat().parse(input)
        )
    }

    #[test]
    fn test_if_stat() {
        let (foo, bar, _) = get_id();
        let (one, two, _, _) = get_number();
        let one_plusplus = get_unary_expr();
        let empty_node = Box::new(Empty);
        let foo_eqeq_one = Box::new(BinaryExpr {
            kind: BINARY_EXPR,
            left: foo.clone(),
            op: EQEQ,
            right: one.clone(),
        });
        let bar_eqeq_two = Box::new(BinaryExpr {
            kind: BINARY_EXPR,
            left: bar.clone(),
            op: EQEQ,
            right: two.clone(),
        });

        let input = lex(r#"
        if (foo == 1) {
        } else if (bar == 2) {
        } else {
        }
        "#);
        assert_eq!(
            Ok((
                vec![],
                IfStatement {
                    kind: IF_STAT,
                    expr: foo_eqeq_one.clone(),
                    then_block: vec![],
                    else_block: Box::new(IfStatement {
                        kind: IF_STAT,
                        expr: bar_eqeq_two.clone(),
                        then_block: vec![],
                        else_block: Box::new(IfStatement {
                            kind: IF_STAT,
                            expr: empty_node.clone(),
                            then_block: vec![],
                            else_block: empty_node.clone()
                        })
                    })
                }
            )),
            if_stat().parse(input)
        );

        let input = lex(r#"
        if (foo == 1) {
        } else {
        }
        "#);
        assert_eq!(
            Ok((
                vec![],
                IfStatement {
                    kind: IF_STAT,
                    expr: foo_eqeq_one.clone(),
                    then_block: vec![],
                    else_block: Box::new(IfStatement {
                        kind: IF_STAT,
                        expr: empty_node.clone(),
                        then_block: vec![],
                        else_block: empty_node.clone()
                    })
                }
            )),
            if_stat().parse(input)
        );

        let input = lex(r#"
        if (foo == 1) {
        } else if (bar == 2) {
        }
        "#);
        assert_eq!(
            Ok((
                vec![],
                IfStatement {
                    kind: IF_STAT,
                    expr: foo_eqeq_one.clone(),
                    then_block: vec![],
                    else_block: Box::new(IfStatement {
                        kind: IF_STAT,
                        expr: bar_eqeq_two.clone(),
                        then_block: vec![],
                        else_block: empty_node.clone()
                    })
                }
            )),
            if_stat().parse(input)
        );

        let input = lex(r#"
        if (foo == 1) {
            1++;    
        } else if (bar == 2) {
            1++
        } else {
            1++
        }
        "#);
        assert_eq!(
            Ok((
                vec![],
                IfStatement {
                    kind: IF_STAT,
                    expr: foo_eqeq_one.clone(),
                    then_block: vec![one_plusplus.clone()],
                    else_block: Box::new(IfStatement {
                        kind: IF_STAT,
                        expr: bar_eqeq_two.clone(),
                        then_block: vec![
                            one_plusplus.clone()
                        ],
                        else_block: Box::new(IfStatement {
                            kind: IF_STAT,
                            expr: empty_node.clone(),
                            then_block: vec![
                                one_plusplus.clone()
                            ],
                            else_block: empty_node.clone()
                        })
                    })
                }
            )),
            if_stat().parse(input)
        );
    }

    #[test]
    fn test_switch_stat() {
        let (one, two, three, _) = get_number();
        let (foo, bar, _) = get_id();

        let input = lex(r#"
        switch (foo) {
            case 1:
            break;
            default: 
            break;
        }"#);
        assert_eq!(
            Ok((
                vec![],
                SwitchStatement {
                    kind: SWITCH_STAT,
                    expr: foo.clone(),
                    then_block: vec![
                        Box::new(CaseStatement {
                            kind: CASE_STAT,
                            expr: one.clone(),
                            has_break: true,
                            then_block: vec![]
                        }),
                        Box::new(DefaultStatement {
                            kind: DEFAULT_CASE_STAT,
                            has_break: true,
                            then_block: vec![]
                        })
                    ]
                }
            )),
            switch_stat().parse(input)
        );

        let input = lex(r#"
        switch (foo) {
            case 1:
                bar = 1;
            break;
            case 2:
                bar = 2;
            break;
            default: 
                bar = 3;
            break;
        }"#);
        assert_eq!(
            Ok((
                vec![],
                SwitchStatement {
                    kind: SWITCH_STAT,
                    expr: foo.clone(),
                    then_block: vec![
                        Box::new(CaseStatement {
                            kind: CASE_STAT,
                            expr: one.clone(),
                            has_break: true,
                            then_block: vec![Box::new(
                                AssignmentExpr {
                                    kind: ASSIGNMENT_EXPR,
                                    left: bar.clone(),
                                    right: one.clone()
                                }
                            )]
                        }),
                        Box::new(CaseStatement {
                            kind: CASE_STAT,
                            expr: two.clone(),
                            has_break: true,
                            then_block: vec![Box::new(
                                AssignmentExpr {
                                    kind: ASSIGNMENT_EXPR,
                                    left: bar.clone(),
                                    right: two.clone()
                                }
                            )]
                        }),
                        Box::new(DefaultStatement {
                            kind: DEFAULT_CASE_STAT,
                            has_break: true,
                            then_block: vec![Box::new(
                                AssignmentExpr {
                                    kind: ASSIGNMENT_EXPR,
                                    left: bar.clone(),
                                    right: three.clone()
                                }
                            )]
                        })
                    ]
                }
            )),
            switch_stat().parse(input)
        );

        let input = lex(r#"
        switch (foo) {
            case 1:
                bar = 1;
            case 2:
                bar = 2;
            default: 
                bar = 3;
        }"#);
        assert_eq!(
            Ok((
                vec![],
                SwitchStatement {
                    kind: SWITCH_STAT,
                    expr: foo.clone(),
                    then_block: vec![
                        Box::new(CaseStatement {
                            kind: CASE_STAT,
                            expr: one.clone(),
                            has_break: false,
                            then_block: vec![Box::new(
                                AssignmentExpr {
                                    kind: ASSIGNMENT_EXPR,
                                    left: bar.clone(),
                                    right: one.clone()
                                }
                            )]
                        }),
                        Box::new(CaseStatement {
                            kind: CASE_STAT,
                            expr: two.clone(),
                            has_break: false,
                            then_block: vec![Box::new(
                                AssignmentExpr {
                                    kind: ASSIGNMENT_EXPR,
                                    left: bar.clone(),
                                    right: two.clone()
                                }
                            )]
                        }),
                        Box::new(DefaultStatement {
                            kind: DEFAULT_CASE_STAT,
                            has_break: false,
                            then_block: vec![Box::new(
                                AssignmentExpr {
                                    kind: ASSIGNMENT_EXPR,
                                    left: bar.clone(),
                                    right: three.clone()
                                }
                            )]
                        })
                    ]
                }
            )),
            switch_stat().parse(input)
        );
    }

    #[test]
    fn test_for_args_stat() {
        let (one, _, _, _) = get_number();
        let (foo, _, _) = get_id();
        let one_eqeq_one = get_binary_expr();
        let one_plusplus = get_unary_expr();

        let input = lex("let foo = 1; 1==1; 1++");
        assert_eq!(
            Ok((
                vec![],
                (
                    Box::new(VariableDeclaStatement {
                        kind: VARIABLE_DECLA_STAT,
                        definator: "let".to_string(),
                        name: foo.clone(),
                        init: one.clone()
                    }),
                    one_eqeq_one.clone(),
                    one_plusplus.clone()
                )
            )),
            for_stat_args().parse(input)
        )
    }

    #[test]
    fn test_for_stat() {
        let (one, _, _, _) = get_number();
        let (foo, _, _) = get_id();
        let one_eqeq_one = get_binary_expr();
        let one_plusplus = get_unary_expr();
        let empty = Box::new(Empty);

        let input = lex("for(let foo = 1; 1==1; 1++){}");
        assert_eq!(
            Ok((
                vec![],
                ForStatement {
                    kind: FOR_STAT,
                    init: Box::new(
                        VariableDeclaStatement {
                            kind: VARIABLE_DECLA_STAT,
                            definator: "let".to_string(),
                            name: foo.clone(),
                            init: one.clone()
                        }
                    ),
                    condition: one_eqeq_one.clone(),
                    step: one_plusplus.clone(),
                    then_block: vec![]
                }
            )),
            for_stat().parse(input)
        );

        let input = lex("for (;;) {}");
        assert_eq!(
            Ok((
                vec![],
                ForStatement {
                    kind: FOR_STAT,
                    init: empty.clone(),
                    condition: empty.clone(),
                    step: empty.clone(),
                    then_block: vec![]
                }
            )),
            for_stat().parse(input)
        )
    }

    #[test]
    fn test_while_stat() {
        let one_eqeq_one = get_binary_expr();

        let input = lex("while( 1 == 1 ) {}");
        assert_eq!(
            Ok((
                vec![],
                WhileStatement {
                    kind: WHILE_STAT,
                    condition: one_eqeq_one.clone(),
                    then_block: vec![]
                }
            )),
            while_stat().parse(input)
        )
    }
}
