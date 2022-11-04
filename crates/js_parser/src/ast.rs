mod grammar;
mod node;
mod tree;

use crate::lex::TokenStream;
pub use grammar::root;
pub use node::{BoxedNode, Node, Node::*};
use shared::parser_combiner::Parser;

pub fn syntax(
    token_stream: TokenStream,
) -> Result<Node, String> {
    match root().parse(token_stream) {
        Ok((_, node)) => Ok(node),
        Err(_) => Err("Parsing failed".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        BinaryExpr, FunctionCallExpr,
        FunctionDeclaStatement, Id, Root, StringLiteral,
        ValueAccessExpr, VariableDeclaStatement,
    };
    use crate::syntax_kind::{
        BINARY_EXPR, FUNCTION_CALL_EXPR,
        FUNCTION_DECLA_STAT, ID, PLUS, ROOT, STRING,
        VALUE_ACCESS_EXPR, VARIABLE_DECLA_STAT,
    };
    use crate::{lex, syntax};

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
            Box::new(FunctionDeclaStatement {
                kind: FUNCTION_DECLA_STAT,
                name: Box::new(Id {
                    kind: ID,
                    name: "sayHello".to_string(),
                }),
                args: vec![Box::new(Id {
                    kind: ID,
                    name: "value".to_string(),
                })],
                body: vec![Box::new(FunctionCallExpr {
                    kind: FUNCTION_CALL_EXPR,
                    callee: Box::new(ValueAccessExpr {
                        kind: VALUE_ACCESS_EXPR,
                        path: vec![
                            Box::new(Id {
                                kind: ID,
                                name: "console".to_string(),
                            }),
                            Box::new(Id {
                                kind: ID,
                                name: "log".to_string(),
                            }),
                        ],
                    }),
                    args: vec![Box::new(BinaryExpr {
                        kind: BINARY_EXPR,
                        left: Box::new(Id {
                            kind: ID,
                            name: "value".to_string(),
                        }),
                        op: PLUS,
                        right: Box::new(StringLiteral {
                            kind: STRING,
                            value: " 2022.11.4".to_string(),
                            raw: "\" 2022.11.4\""
                                .to_string(),
                        }),
                    })],
                })],
            });
        let variable_decla =
            Box::new(VariableDeclaStatement {
                kind: VARIABLE_DECLA_STAT,
                definator: "const".to_string(),
                name: Box::new(Id {
                    kind: ID,
                    name: "msg".to_string(),
                }),
                init: Box::new(StringLiteral {
                    kind: STRING,
                    value: "Hello FMT".to_string(),
                    raw: "\"Hello FMT\"".to_string(),
                }),
            });
        let function_call = Box::new(FunctionCallExpr {
            kind: FUNCTION_CALL_EXPR,
            callee: Box::new(Id {
                kind: ID,
                name: "sayHello".to_string(),
            }),
            args: vec![Box::new(Id {
                kind: ID,
                name: "msg".to_string(),
            })],
        });

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
