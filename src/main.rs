use std::fs;

mod lexer;
mod ast;

fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    let argc = argv.len();
    if argc == 2 {
        let contents = fs::read_to_string(argv[1].to_string()).unwrap();
        let tokens = lexer::tokenize(contents);
        println!("{:?}", tokens);
    }
}
