// --------------------------
// Expressions and Statements
// --------------------------

use core::fmt;
use std::fmt::Display;

use crate::lexer::TokenType;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ASTNode {
    CallExpr(Vec<ASTNode>),
    StringExpr(String),
    IdentifierExpr(String),
    NumberExpr(i32),
    BinaryExpr(Box<ASTNode>, TokenType, Box<ASTNode>),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AST {
    pub(crate) program: Vec<ASTNode>,
}

impl AST {
    pub(crate) fn new(program: Vec<ASTNode>) -> Self {
        Self { program }
    }
}
