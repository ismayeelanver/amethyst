use std::fs;
mod ast;
mod error;
mod lexer;

fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    let argc = argv.len();
    if argc == 2 {
        let contents = fs::read_to_string(argv[1].to_string()).unwrap();
        let ast = ast::parser::Parser::parser(contents);
        println!("{:#?}", ast)
    }
}
