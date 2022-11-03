/// Stat -> Expr (";")?
///       | DeclarationStat
///       | ConditionalStat
///       | CycleStat
pub fn stat() {
    todo!()
}

/// ExprStat -> Expr (";")?
pub fn expr_stat() {}

/// DeclarationStat -> FunctionDeclara
///                  | VariableDeclara
pub fn declaration_state() {}

/// FunctionDecla -> FUNCTION ID "(" (ID ("," ID)*)? ")" Block
pub fn function_decla() {}

/// VariableDecla -> DEFINTOR ID "=" Expr ";"
pub fn variable_decla() {}

/// ConditionStat -> IfStat | SwitchStat
pub fn condition_stat() {}

/// IfStat -> IF "(" Expr ")" Block
pub fn if_stat() {}

/// ElseIfStat -> ELSE IfStat
pub fn else_if_stat() {}

/// ElseStat -> ELSE Block
pub fn else_stat() {}

/// SwitchStat -> SWITH "(" Expr ")" Block
pub fn switch_stat() {}

/// CaseStat -> CASE Expr ":" Stat* BREAK? ";"?
pub fn case_stat() {}

/// DefaultStat -> DEFAULT Expr ":" Stat* BREAK? ";"?
pub fn default_stat() {}

/// CycleStat -> ForStat | While Stat
pub fn cycle_stat() {}

/// ForStat -> FOR "(" VariableDecla? ";" BinaryExpr ";" Expr ")" Block
pub fn for_stat() {}

/// WhileStat -> WHILE "(" Expr ")" Block
pub fn while_stat() {}
