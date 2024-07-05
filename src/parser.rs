use crate::ast::*;
use crate::lexer::{tokenize, Token, TokenType};

pub struct Parser {}

impl Parser {
    fn parsable_to_f64(input: String) -> Result<bool, bool> {
        match input.parse::<f64>() {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn is_binaryexpr(token: TokenType) -> bool {
        return token == TokenType::Plus
            || token == TokenType::Minus
            || token == TokenType::Div
            || token == TokenType::Mul
            || token == TokenType::Mod;
    }

    fn tokenize_content(content: String) -> Vec<Token> {
        tokenize(content)
    }

    fn find_op(op: TokenType) -> Operators {
        match op {
            TokenType::Plus => Operators::Addition,
            TokenType::Minus => Operators::Subtraction,
            TokenType::Div => Operators::Division,
            TokenType::Mul => Operators::Multiplication,
            TokenType::Mod => Operators::Modulus,
            _ => panic!("Unexpected token: {:?}", op),
        }
    }

    pub fn parse(content: String) -> Program {
        let tokens = Parser::tokenize_content(content);
        let mut i = 0;
        let mut body: Vec<Expr> = vec![];

        let expr: Expr = match tokens[i]._type {
            TokenType::IntegerLiteral => {
                if Parser::is_binaryexpr(tokens[i + 1]._type) {
                    if Parser::parsable_to_f64(tokens[i + 2].value.clone()).unwrap() {
                        let left = BinaryExprType::IntegerLiteral {
                            value: tokens[i + 2].value.parse::<f64>().unwrap(),
                        };
                        let right = BinaryExprType::IntegerLiteral {
                            value: tokens[i + 2].value.parse::<f64>().unwrap(),
                        };
                        Expr::BinaryExpr {
                            left: Box::from(left),
                            op: Parser::find_op(tokens[i + 1]._type),
                            right: Box::from(right),
                        }
                    } else {
                        let left = BinaryExprType::IntegerLiteral {
                            value: tokens[i].value.parse::<f64>().unwrap(),
                        };
                        let right = BinaryExprType::IdentifierLiteral {
                            value: tokens[i + 2].value.clone(),
                        };
                        Expr::BinaryExpr {
                            left: Box::from(left),
                            op: Parser::find_op(tokens[i + 1]._type),
                            right: Box::from(right),
                        }
                    }
                } else {
                    Expr::IntegerLiteral {
                        value: tokens[i].value.parse::<f64>().unwrap(),
                    }
                }
            }
            TokenType::Identifier => {
                if Parser::is_binaryexpr(tokens[i + 1]._type) {
                    if Parser::parsable_to_f64(tokens[i + 2].value.clone()).unwrap() {
                        let left = BinaryExprType::IdentifierLiteral {
                            value: tokens[i].value.clone(),
                        };
                        let right = BinaryExprType::IntegerLiteral {
                            value: tokens[i + 2].value.parse::<f64>().unwrap(),
                        };
                        Expr::BinaryExpr {
                            left: Box::from(left),
                            op: Parser::find_op(tokens[i + 1]._type),
                            right: Box::from(right),
                        }
                    } else {
                        let left = BinaryExprType::IdentifierLiteral {
                            value: tokens[i].value.clone(),
                        };
                        let right = BinaryExprType::IdentifierLiteral {
                            value: tokens[i + 2].value.clone(),
                        };
                        Expr::BinaryExpr {
                            left: Box::from(left),
                            op: Parser::find_op(tokens[i + 1]._type),
                            right: Box::from(right),
                        }
                    }
                } else {
                    Expr::IdentifierLiteral {
                        value: tokens[i].value.clone(),
                    }
                }
            }
            _ => panic!("Unexpected token: {:?}", tokens[i]._type),
        };

        while tokens[i]._type != TokenType::EOF {
            i += 1;
        }

        body.push(expr);
        Program { body }
    }
}
