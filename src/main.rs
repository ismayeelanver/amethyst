use std::{fs, process::exit};
mod lexer;
mod terminal;
fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    let argc = argv.len();
    if argc == 1 {
        terminal::run()
    } else {
        let contents = fs::read_to_string(argv[1].to_string()).unwrap();
        let tokens = lexer::tokenize(contents);
        println!("{:?}", tokens)
    }
}
