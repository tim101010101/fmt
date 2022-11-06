use crate::parser::ast::node::{Node, Node::*};

// mod visitor {
//     use super::*;
//
//     pub(crate) trait Visitor {
//         fn visit(&mut self, n: &Node) {
//             match n {
//                 Root { .. } => self.visit_root(n),
//
//                 Id { .. }
//                 | StringLiteral { .. }
//                 | NumberLiteral { .. }
//                 | ObjectLiteral { .. }
//                 | ArrayLiteral { .. } => {
//                     self.visit_literal(n)
//                 }
//
//                 UnaryExpr { .. }
//                 | BinaryExpr { .. }
//                 | TernaryExpr { .. }
//                 | AssignmentExpr { .. }
//                 | ValueAccessExpr { .. }
//                 | FunctionCallExpr { .. }
//                 | ReturnExpr { .. } => self.visit_expr(n),
//
//                 VariableDeclaStatement { .. }
//                 | FunctionDeclaStatement { .. }
//                 | IfStatement { .. }
//                 | SwitchStatement { .. }
//                 | CaseStatement { .. }
//                 | DefaultStatement { .. }
//                 | ForStatement { .. }
//                 | WhileStatement { .. } => {
//                     self.visit_stat(n)
//                 }
//
//                 // TODO
//                 _ => panic!(),
//             }
//         }
//
//         fn visit_root(&mut self, n: &Node) {
//             match n {
//                 Root { statements, .. } => statements
//                     .iter()
//                     .for_each(|node| self.visit(node)),
//                 // TODO
//                 _ => {}
//             }
//         }
//         fn visit_literal(&mut self, n: &Node);
//         fn visit_expr(&mut self, n: &Node);
//         fn visit_stat(&mut self, n: &Node);
//     }
// }
//
mod visiable {}
