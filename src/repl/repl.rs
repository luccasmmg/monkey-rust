use std::process;
use text_io::read;
use crate::lexer::lexer::convert_to_tokens;

pub fn repl(line: String) {
    if line == "exit" {
        process::exit(0);
    } else {
        println!(">> {:?}", convert_to_tokens(&line));
        let new_line: String = read!("{}\n");
        repl(new_line)
    }
}
