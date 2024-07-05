use crate::parser::Parser;
use std::io::{self, Write};
pub fn run() {
    let mut buf = String::new();
    loop {
        print!(">>> ");
        buf = String::new();
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut buf).unwrap();
        if buf.as_str() == "exit" {
            break;
        } else {
            let ast = Parser::parse(buf);
            println!("{:?}", ast);
        }
    }
}