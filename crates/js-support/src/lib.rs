mod parser;
mod syntax_kind;

#[cfg(test)]
mod tests {
    use crate::{
        parser::{lex, syntax, Node::*},
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