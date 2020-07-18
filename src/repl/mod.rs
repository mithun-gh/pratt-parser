use std::io::{self, Write};
use termion::{clear, cursor/*, event::Key, input::TermRead*/};

use crate::interpreter::calculate;

pub struct Repl {
    pub version: &'static str,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            version: "v0.1",
        }
    }

    pub fn run(&self) {
        println!("Welcome to CalcScript {}.", self.version);
        println!("Type .quit or press CTRL+C to exit.\n");

        let mut input_text = String::new();
        loop {
            print!("> ");
            io::stdout().flush().expect("Stdout error");

            input_text.clear();
            io::stdin().read_line(&mut input_text).expect("Stdin error");

            match input_text.trim() {
                ".quit" | ".exit" => break,
                ".clear" => println!("{}{}", clear::All, cursor::Goto(1, 1)),
                _ => print_result(&input_text),
            }

        }
    }
}

fn print_result(input_text: &str) {
    if let Some(result) = calculate(input_text.trim()) {
        if result.fract() != 0.0 {
            println!("{}", result);
        } else {
            println!("{:.0}", result);
        }
    }
}
