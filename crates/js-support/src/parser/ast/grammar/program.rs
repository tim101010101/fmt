use crate::{
    parser::{
        ast::{grammar::boxed_stat_node, Node, Node::Root},
        TokenStream,
    },
    syntax_kind::ROOT,
};
use shared::parser_combiner::{zero_or_more, Parser};

/// Root -> stat*
pub fn root() -> impl Parser<'static, TokenStream, Node> {
    zero_or_more(boxed_stat_node()).map(|statements| Root {
        kind: ROOT,
        statements,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::node::{literal_node, stat_node};
    use crate::parser::ast::Literal::{Id, NumberLiteral};
    use crate::parser::ast::Stat::{FunctionDeclaStatement, VariableDeclaStatement};
    use crate::parser::lex;
    use crate::syntax_kind::*;

    #[test]
    fn test_root() {
        let input = lex("");
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
                        Box::new(stat_node(VariableDeclaStatement {
                            kind: VARIABLE_DECLA_STAT,
                            definator: "let".to_string(),
                            name: Box::new(literal_node(Id {
                                kind: ID,
                                name: "a".to_string()
                            })),
                            init: Box::new(literal_node(NumberLiteral {
                                kind: NUMBER,
                                value: 1,
                                raw: "1".to_string()
                            }))
                        })),
                        Box::new(stat_node(FunctionDeclaStatement {
                            kind: FUNCTION_DECLA_STAT,
                            name: Box::new(literal_node(Id {
                                kind: ID,
                                name: "b".to_string()
                            })),
                            args: vec![],
                            body: vec![]
                        }))
                    ]
                }
            )),
            root().parse(input)
        )
    }
}
