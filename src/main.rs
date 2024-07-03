use std::{fs, process::exit};
mod lexer;
fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    let argc = argv.len();
    if argc != 2 {
        eprintln!("usage: amethyst [script]");
        exit(67)
    }
    let contents = fs::read_to_string(argv[1].to_string()).unwrap();
    println!("{:?}", contents)
}
