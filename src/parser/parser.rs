use crate::{
    ast::ast::{ASTNode, AST},
    error::{self, Token, UnexpectedTokenError},
    lexer::TokenType,
};
#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current_index: 0,
        }
    }

    pub fn parse(&mut self) -> AST {
        self.parse_expr()
    }

    pub fn parse_expr(&mut self) -> AST {
        let mut ast = AST::new(Vec::new());
        let mut nodes: Vec<ASTNode> = Vec::new();
        let mut operators: Vec<TokenType> = Vec::new();

        while self.peek(None)._type != TokenType::EOF {
            match self.peek(None)._type {
                TokenType::Float
                | TokenType::IntegerLiteral
                | TokenType::Stringliteral
                | TokenType::Identifier => {
                    nodes.push(self.parse_primary().unwrap());
                }
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Mul
                | TokenType::Div
                | TokenType::Mod => {
                    self.parse_operator(&mut nodes, &mut operators);
                }
                _ => error::UnexpectedTokenError(self.peek(None)),
            }
            self.consume();
        }

        while !operators.is_empty() {
            self.apply_operator(&mut nodes, &mut operators);
        }

        ast.program = nodes;
        ast
    }

    fn parse_primary(&mut self) -> Option<ASTNode> {
        if self.peek(None)._type == TokenType::Stringliteral {
            let string = self.peek(None).value;
            Some(ASTNode::StringExpr(string))
        } else if self.peek(None)._type == TokenType::IntegerLiteral {
            let number = self.peek(None).value.parse::<i32>().unwrap();
            Some(ASTNode::NumberExpr(number))
        } else if self.peek(None)._type == TokenType::Identifier {
            let id = self.peek(None).value;
            Some(ASTNode::IdentifierExpr(id))
        } else {
            None
        }
    }

    fn parse_operator(&mut self, nodes: &mut Vec<ASTNode>, operators: &mut Vec<TokenType>) {
        while !operators.is_empty()
            && Self::precedence(&operators.last().unwrap())
                >= Self::precedence(&self.peek(None)._type)
        {
            self.apply_operator(nodes, operators);
        }
        operators.push(self.peek(None)._type);
    }

    fn apply_operator(&mut self, nodes: &mut Vec<ASTNode>, operators: &mut Vec<TokenType>) {
        let right = nodes.pop().unwrap();
        let left = nodes.pop().unwrap();
        let op = operators.pop().unwrap();
        let binary_expr = ASTNode::BinaryExpr(Box::new(left), op, Box::new(right));
        nodes.push(binary_expr);
    }

    fn precedence(op: &TokenType) -> i32 {
        match op {
            TokenType::Plus | TokenType::Minus => 1,
            TokenType::Mul | TokenType::Div | TokenType::Mod => 2,
            _ => 0,
        }
    }

    pub fn peek(&self, offset: Option<usize>) -> Token {
        let index = self.current_index + offset.unwrap_or(0);
        if index < self.tokens.len() {
            self.tokens[index].clone()
        } else {
            Token {
                _type: TokenType::EOF,
                value: String::new(),
            }
        }
    }

    pub fn consume(&mut self) {
        if self.current_index < self.tokens.len() {
            self.current_index += 1;
        }
    }
}
