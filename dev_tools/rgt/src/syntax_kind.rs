use crate::SyntaxKind;

// node
pub const FN: SyntaxKind = SyntaxKind(1);
pub const NAME: SyntaxKind = SyntaxKind(2);
pub const PARAM_LIST: SyntaxKind = SyntaxKind(3);
pub const BIN_EXR: SyntaxKind = SyntaxKind(4);

// token
pub const IDENT: SyntaxKind = SyntaxKind(100);
pub const WHITESPACE: SyntaxKind = SyntaxKind(101);
pub const FN_KW: SyntaxKind = SyntaxKind(102);
pub const INT: SyntaxKind = SyntaxKind(103);
pub const PLUS: SyntaxKind = SyntaxKind(104);
pub const STAR: SyntaxKind = SyntaxKind(105);
