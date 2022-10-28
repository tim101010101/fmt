use crate::ast::grammar::basic::single_token;
use crate::ast::grammar::literal::literal;
use crate::ast::node::NodeData;
use crate::ast::token::TokenData;
use crate::ast::tree::Element;
use crate::lex::TokenStream;
use crate::syntax_kind::{
    DEFINATOR, FUNCTION_CALL, ID, SEQUENCE_EXPR,
    VALUE_ACCESS_EXPR,
};
use crate::T;
use shared::parser_combiner::{
    either, left, pair, right, zero_or_more, zero_or_one,
    Parser,
};

pub fn sequence_item(
) -> impl Parser<'static, TokenStream, Element> {
    either(
        single_token(ID).map(|(kind, text)| {
            TokenData::new(kind, text).into()
        }),
        literal(),
    )
}

/// SequenceExpr -> (ID | Literal) [, (ID | Literal)]*
pub fn sequence_expr(
) -> impl Parser<'static, TokenStream, Element> {
    sequence_item().and_then(|first_item| {
        zero_or_more(right(
            single_token(T![","]),
            sequence_item(),
        ))
        .map(move |mut children| {
            children.insert(0, first_item.to_owned());
            NodeData::new(SEQUENCE_EXPR, children).into()
        })
    })
}

/// BlockExpr -> OPEN_BRACE [Expr]* CLOSE_BRACE
// pub fn block_expr(
// ) -> impl Parser<'static, TokenStream, Element> {
//     right(
//         single_token(T!["{"]),
//         left(zero_or_more(expr()), single_token(T!["}"])),
//     )
// }

/// DeclarationExpr -> FunctionDeclaration | VariableDeclaration
pub fn declaration_expr() {
    todo!()
}

/// FunctionDeclaration -> FUNCTION ID OPEN_PAREN SequenceExpr CLOSE_PAREN BlockExpr
pub fn function_declaration() {
    todo!()
}

/// VariableDeclaration ->
///         ( CONST | LET | VAR )
///         ID
///         EQ
///         ( Literal | Expr )
pub fn variable_declaration(
) -> impl Parser<'static, TokenStream, Element> {
    pair(single_token(DEFINATOR), single_token(ID))
        .and_then(|(definator, id)| {
            right(
                single_token(T!["="]),
                left(
                    // literal for this time being
                    literal(),
                    zero_or_one(single_token(T![";"])),
                ),
            )
        })
}

/// ValueAccessExpr -> ID [. ID]*
pub fn value_access_expr(
) -> impl Parser<'static, TokenStream, Element> {
    single_token(ID)
        .map(|(kind, text)| {
            TokenData::new(kind, text).into()
        })
        .and_then(|first_item: Element| {
            zero_or_more(right(
                single_token(T!["."]),
                single_token(ID).map(|(kind, text)| {
                    TokenData::new(kind, text).into()
                }),
            ))
            .map(move |mut children| {
                children.insert(0, first_item.to_owned());
                NodeData::new(VALUE_ACCESS_EXPR, children)
                    .into()
            })
        })
}

/// FunctionCallExpr -> ValueAccessExpr OPEN_PAREN SequenceExpr CLOSE_PAREN
pub fn function_call_expr(
) -> impl Parser<'static, TokenStream, Element> {
    left(value_access_expr(), single_token(T!["("]))
        .and_then(|callee| {
            left(sequence_expr(), single_token(T![")"]))
                .map(move |args| {
                    NodeData::new(
                        FUNCTION_CALL,
                        vec![callee.to_owned(), args],
                    )
                    .into()
                })
        })
}

/// AssignmentExpr -> Expr = Expr
pub fn assignment_expr() {
    todo!()
}

/// UnaryExpr -> ++ | --
///            | !
///            | TYPE_OF | DELETE
///            Expr
pub fn unary_expr() {
    todo!()
}

/// BinaryExpr -> Expr
///             | == | != | === | !==
///             | < | <= | > | >=
///             | + | - | * | /
///             | & | | | ^ | ~ | >> | >>> | <<
///             | INSTANCE_OF | IN
///             Expr
pub fn binary_expr() {
    todo!()
}

/// TernaryExpr -> BinaryExpr QUESTION Expr COLON Expr
pub fn ternary_expr() {
    todo!()
}

pub fn if_expr() {
    todo!()
}

pub fn else_expr() {
    todo!()
}

pub fn for_expr() {
    todo!()
}

pub fn while_expr() {
    todo!()
}

pub fn switch_expr() {
    todo!()
}

pub fn case_expr() {
    todo!()
}

pub fn default_expr() {
    todo!()
}

/// Expr -> Literal
///       | AssignmentExpr
///       | UnaryExpr | BinaryExpr | TernaryExpr
///       | ConditionExpr | CirculateExpr
// pub fn expr() -> impl Parser<'static, TokenStream, Element>
// {
//     todo!()
// }

#[cfg(test)]
mod tests {

    #[test]
    fn test_variable_declaration() {
        todo!()
    }
}
