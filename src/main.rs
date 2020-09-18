use std::io;
use std::env;

use text_io::read;

mod lexer;
mod token;
mod repl;
mod parser;
mod ast;

use crate::repl::repl::repl;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args[1] == String::from("repl") {
        let first_line: String = read!("{}\n");
        repl(first_line)
    }
    Ok(())
}
