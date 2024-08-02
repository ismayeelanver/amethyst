use std::{
    fmt::{self},
    fs,
};

use lexer::TokenType;
use parser::parser::Parser;

mod ast;
mod error;
mod lexer;
mod parser;

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for lexer::Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} `{}`", self._type, self.value)
    }
}

fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    let argc = argv.len();
    if argc == 2 {
        let contents = fs::read_to_string(argv[1].to_string()).unwrap();
        let tokens = lexer::tokenize(contents);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        println!("{:#?}", ast)
    }
}
