use crate::{error, lexer};

use super::ast::{self, Expr, ExprKind, Stmt, StmtKind};

pub struct Parser;

impl Parser {
    pub fn parser(content: String) -> ast::AST {
        let tokens = lexer::tokenize(content);
        let stmts = Self::parse_stmt(tokens);
        ast::AST(stmts)
    }

    fn parse_stmt(tokens: Vec<lexer::Token>) -> Vec<Stmt> {
        let kind = Self::parse_stmtkind(tokens);
        vec![Stmt { kind }]
    }

    fn parse_stmtkind(tokens: Vec<lexer::Token>) -> StmtKind {
        let expr = Self::parse_binary_expr(tokens);
        StmtKind::ExprStmt(expr)
    }

    fn check_RParen(token: lexer::Token) -> bool {
        return token._type == lexer::TokenType::RParen;
    }

    fn parse_binary_expr(tokens: Vec<lexer::Token>) -> Expr {
        let mut kind: Result<ExprKind, ()> = Ok(ExprKind::NumericalLiteral(0.0));
        for i in 0..tokens.len() {}
        let expr = Expr {
            kind: kind.unwrap(),
        };
        expr
    }
}
