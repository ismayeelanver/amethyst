use std::{fmt, process::exit};
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Assingment,
    Return,
    LParen,
    RParen,
    Exit,
    IntegerLiteral,
    Identifier,
    Plus,
    Minus,
    Div,
    Mul,
    Mod,
    Repr,
    Comma,
    Greaterthan,
    Lesserthan,
    Leftblock,
    Rightblock,
    Equals,
    Stringliteral,
    In,
    Function,
    Variable,
    Print,
    EOF,
    Dot,
    Float,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub _type: TokenType,
    pub value: String,
}

pub trait TokenTraits {
    fn to_string(&self) -> String;
    fn fmt(&self, f: fmt::Formatter<'_>) -> fmt::Result;
}

impl TokenTraits for Token {
    fn fmt(&self, mut f: fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> ({})", self._type, self.value)
    }
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

pub fn tokenize(content: String) -> Vec<Token> {
    let vechar = content.chars().collect::<Vec<char>>();
    let mut tokens: Vec<Token> = vec![];
    let mut i: usize = 0;

    while i < vechar.len() {
        let current_char = vechar[i];

        if current_char.is_whitespace() {
            i += 1;
            continue;
        }

        // Skip single-line comments
        if current_char == '/' && i + 1 < vechar.len() && vechar[i + 1] == '/' {
            while i < vechar.len() && vechar[i] != '\n' {
                i += 1;
            }
            continue;
        }

        // Skip multi-line comments
        if current_char == '/' && i + 1 < vechar.len() && vechar[i + 1] == '*' {
            i += 2;
            while i + 1 < vechar.len() && !(vechar[i] == '*' && vechar[i + 1] == '/') {
                i += 1;
            }
            i += 2;
            continue;
        }

        let token = match current_char {
            '0'..='9' => {
                let mut num_str = String::new();
                let mut is_float = false;
                while i < vechar.len() && (vechar[i].is_digit(10) || vechar[i] == '.') {
                    if vechar[i] == '.' {
                        if is_float {
                            eprintln!("Unexpected token {}", current_char);
                            exit(54);
                        }
                        is_float = true;
                    }
                    num_str.push(vechar[i]);
                    i += 1;
                }
                if is_float {
                    Token {
                        _type: TokenType::Float,
                        value: num_str,
                    }
                } else {
                    Token {
                        _type: TokenType::IntegerLiteral,
                        value: num_str,
                    }
                }
            }
            '.' => {
                i += 1;
                Token {
                    _type: TokenType::Dot,
                    value: String::from("."),
                }
            }
            '(' => {
                i += 1;
                Token {
                    _type: TokenType::LParen,
                    value: current_char.to_string(),
                }
            }
            ')' => {
                i += 1;
                Token {
                    _type: TokenType::RParen,
                    value: current_char.to_string(),
                }
            }
            c if c.is_alphanumeric() => {
                let mut id_str = String::new();
                while i < vechar.len() && vechar[i].is_alphanumeric() {
                    id_str.push(vechar[i]);
                    i += 1;
                }
                match id_str.as_str() {
                    "return" => Token {
                        _type: TokenType::Return,
                        value: id_str,
                    },
                    "exit" => Token {
                        _type: TokenType::Exit,
                        value: id_str,
                    },
                    "in" => Token {
                        _type: TokenType::In,
                        value: id_str,
                    },
                    "func" => Token {
                        _type: TokenType::Function,
                        value: id_str,
                    },
                    "var" => Token {
                        _type: TokenType::Variable,
                        value: id_str,
                    },
                    "print" => Token {
                        _type: TokenType::Print,
                        value: id_str,
                    },
                    _ => Token {
                        _type: TokenType::Identifier,
                        value: id_str,
                    },
                }
            }
            '+' => {
                i += 1;
                Token {
                    _type: TokenType::Plus,
                    value: current_char.to_string(),
                }
            }
            '-' => {
                i += 1;
                Token {
                    _type: TokenType::Minus,
                    value: current_char.to_string(),
                }
            }
            '*' => {
                i += 1;
                Token {
                    _type: TokenType::Mul,
                    value: current_char.to_string(),
                }
            }
            '/' => {
                i += 1;
                Token {
                    _type: TokenType::Div,
                    value: current_char.to_string(),
                }
            }
            ':' => {
                let start = i;
                i += 1;
                let mut tk: Token = Token {
                    _type: TokenType::Repr,
                    value: current_char.to_string(),
                };
                if vechar[i] == '=' {
                    i += 1;
                    tk = Token {
                        _type: TokenType::Assingment,
                        value: vechar[start..i].into_iter().collect(),
                    };
                }
                tk
            }
            '%' => {
                i += 1;
                Token {
                    _type: TokenType::Mod,
                    value: current_char.to_string(),
                }
            }
            ',' => {
                i += 1;
                Token {
                    _type: TokenType::Comma,
                    value: current_char.to_string(),
                }
            }
            '>' => {
                i += 1;
                Token {
                    _type: TokenType::Greaterthan,
                    value: current_char.to_string(),
                }
            }
            '<' => {
                i += 1;
                Token {
                    _type: TokenType::Lesserthan,
                    value: current_char.to_string(),
                }
            }
            '{' => {
                i += 1;
                Token {
                    _type: TokenType::Leftblock,
                    value: current_char.to_string(),
                }
            }
            '}' => {
                i += 1;
                Token {
                    _type: TokenType::Rightblock,
                    value: current_char.to_string(),
                }
            }
            '=' => {
                i += 1;
                Token {
                    _type: TokenType::Equals,
                    value: current_char.to_string(),
                }
            }
            '"' => {
                i += 1;
                let start = i;
                while i < vechar.len() && vechar[i] != '"' {
                    i += 1;
                }
                let chars = vechar[start..i].iter().collect::<String>();
                i += 1;
                Token {
                    _type: TokenType::Stringliteral,
                    value: chars,
                }
            }
            _ => {
                eprintln!("Unexpected token {}", current_char);
                exit(54);
            }
        };
        tokens.push(token);
    }
    tokens.push(Token {
        _type: TokenType::EOF,
        value: '\0'.to_string(),
    });

    tokens
}
