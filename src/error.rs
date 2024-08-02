use std::process::exit;

pub(crate) use crate::lexer::Token;

pub fn UnexpectedTokenError(token: Token) {
    eprintln!(
        "\x1B[38;2;255;0;0mError ✗\x1B[0m\n|\n\t\x1B[38;2;55;200;155mUnexpected Token\x1B[0m -> \x1B[38;2;100;200;255m{}\x1B[0m",
        token
    );
    exit(34);
}

pub fn OutOfRange() {
    eprintln!(
        "\x1B[38;2;255;0;0mError ✗\x1B[0m\n|\n\t\x1B[38;2;55;200;155mUIndex Out of range!\x1B[0m No more characters can be displayed!\x1B[38;2;100;200;255m\x1B[0m");
    exit(34);
}

pub fn TrailingCharacter() {
    eprintln!(
        "\x1B[38;2;255;0;0mError ✗\x1B[0m\n|\n\t\x1B[38;2;55;200;155mTrailing character!\x1B[0m Unknown Character\x1B[38;2;100;200;255m\x1B[0m");
    exit(34);
}
