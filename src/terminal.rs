use crate::lexer::{self, tokenize};
use std::{borrow::Borrow, io::{self, Write}};
pub fn run() {
    let mut buf = String::new();
    loop {
        print!(">>> ");
        io::stdout().flush();
        io::stdin().read_line(&mut buf).unwrap();
        let tokens = lexer::tokenize(buf.to_string());
        println!("{:?}", tokens)
    }
}