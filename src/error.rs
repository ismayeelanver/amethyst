use std::process::exit;

use crate::lexer;
#[derive(Debug)]
pub(crate) struct Error {}

pub fn lenof(msg: String, character: char) -> Vec<char> {
    let mut vechar: Vec<char> = Vec::new();
    for i in 0..msg.len() {
        vechar.push(character)
    }
    vechar
}

fn colored(r: i32, g: i32, b: i32, text: String) -> String {
    return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text);
}

fn print_error() {
    eprint!("{} ", colored(255, 0, 0, "Error ✗".to_string()))
}

// fn print_warning() {
// eprint!("{} ", colored(255, 255, 100, "Warning".to_string()))
// }

fn underline(text: String) -> String {
    // ANSI escape codes for underlining
    const UNDERLINE_START: &str = "\x1b[4m";
    const RESET: &str = "\x1b[0m";
    format!("{}{}{}", UNDERLINE_START, text, RESET)
}

impl Error {
    pub fn unexpected_token_error(token: lexer::TokenType, pos: usize) {
        let msg = format!("Unexpected Token Error: {:?}", token);
        let code = 34;
        print_error();
        eprintln!(
            "{}:",
            String::from(underline("parser::error::Error".to_string()))
        );
        eprint!("|\n⬡-» {} {}", pos, colored(0, 100, 230, "|".to_string()));
        eprintln!("\t{}", (msg.clone()));
        println!("\t{}", (lenof(msg, '^').into_iter().collect::<String>()));
        exit(code);
    }
}
