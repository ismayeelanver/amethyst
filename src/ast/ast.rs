use crate::lexer;

// ---------------
// Exprs and Stmts
// ---------------
#[derive(Debug)]
pub struct AST(pub Vec<Stmt>);
#[derive(Debug)]
pub enum StmtKind {
    ExprStmt(Expr),
}
#[derive(Debug)]
pub enum ExprKind {
    NumericalLiteral(f32),
    BinaryExpr(Box<ExprKind>, lexer::TokenType, Box<ExprKind>),
}
#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
}
#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
}
