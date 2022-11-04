use crate::ast::grammar::stat_node;
use crate::ast::{Node, Root};
use crate::lex::TokenStream;
use crate::syntax_kind::ROOT;
use shared::parser_combiner::{zero_or_more, Parser};

/// Root -> stat*
pub fn root() -> impl Parser<'static, TokenStream, Node> {
    zero_or_more(stat_node()).map(|statements| Root {
        kind: ROOT,
        statements,
    })
}

#[cfg(test)]
mod tests {
    use crate::ast::grammar::program::root;
    use crate::ast::{
        FunctionDeclaStatement, Id, NumberLiteral, Root,
        VariableDeclaStatement,
    };
    use crate::lex;
    use crate::syntax_kind::{
        FUNCTION_DECLA_STAT, ID, NUMBER, ROOT,
        VARIABLE_DECLA_STAT,
    };
    use shared::parser_combiner::Parser;

    #[test]
    fn test_root() {
        let input = lex("");
        println!("{:?}", input);
        assert_eq!(
            Ok((
                vec![],
                Root {
                    kind: ROOT,
                    statements: vec![]
                }
            )),
            root().parse(input)
        );

        let input = lex("let a = 1;\nfunction b(){}");
        assert_eq!(
            Ok((
                vec![],
                Root {
                    kind: ROOT,
                    statements: vec![
                        Box::new(VariableDeclaStatement {
                            kind: VARIABLE_DECLA_STAT,
                            definator: "let".to_string(),
                            name: Box::new(Id {
                                kind: ID,
                                name: "a".to_string()
                            }),
                            init: Box::new(NumberLiteral {
                                kind: NUMBER,
                                value: 1,
                                raw: "1".to_string()
                            })
                        }),
                        Box::new(FunctionDeclaStatement {
                            kind: FUNCTION_DECLA_STAT,
                            name: Box::new(Id {
                                kind: ID,
                                name: "b".to_string()
                            }),
                            args: vec![],
                            body: vec![]
                        })
                    ]
                }
            )),
            root().parse(input)
        )
    }
}
