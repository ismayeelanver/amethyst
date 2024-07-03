use std::process::exit;

#[derive(Debug, PartialEq)]
enum TokenType {
    Return,
    LParen,
    RParen,
    Exit,
    IntegerLiteral,
    Semi,
    Identifier,
}

#[derive(Debug)]
pub struct Token {
    _type: TokenType,
    value: String,
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

        let token = match current_char {
            '0'..='9' => {
                let mut num_str = String::new();
                while i < vechar.len() && vechar[i].is_digit(10) {
                    num_str.push(vechar[i]);
                    i += 1;
                }
                Token { _type: TokenType::IntegerLiteral, value: num_str }
            },
            '(' => {
                Token { _type: TokenType::RParen, value: String::from("(") }
            },
            ')' => {
                Token { _type: TokenType::LParen, value: String::from(")") }
            }
            c if c.is_alphanumeric() => {
                let mut id_str = String::new();
                while i < vechar.len() && vechar[i].is_alphanumeric() {
                    id_str.push(vechar[i]);
                    i += 1;
                }
                match id_str.as_str() {
                    "return" => Token { _type: TokenType::Return, value: id_str },
                    "exit" => Token { _type: TokenType::Exit, value: id_str },
                    _ => Token { _type: TokenType::Identifier, value: id_str }
                }
            },
            ';' => {
                i += 1;
                Token { _type: TokenType::Semi, value: current_char.to_string() }
            },
            _ => {
                eprintln!("Unexpected token {}", current_char);
                exit(54);
            }
        };

        tokens.push(token);
    }

    tokens
}
