use std::io::{self, Write};

use crate::{lexer::Lexer, parser};

pub struct Repl {}

impl Repl {
    pub fn start() {
        loop {
            print!(">>> ");
            let _ = io::stdout().flush();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading from STDIN");

            let lexer = Lexer::new(input.into_bytes().into_boxed_slice());
            let mut parser = parser::Parser::new(lexer);
            let program = parser.parse_program();

            if parser.errors.len() > 0 {
                for error in parser.errors {
                    println!("{}", error);
                }
            }

            println!("{}", program.to_string());
        }
    }
}
