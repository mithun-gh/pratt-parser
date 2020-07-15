use std::io::{self, Write};

use crate::interpreter::calculate;

pub struct Repl {
    pub version: &'static str,
    pub lang_version: &'static str,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            version: "v0.1",
            lang_version: "v0.1",
        }
    }

    pub fn run(&self) {
        println!("Welcome to CalcScript!");
        println!("Language: {} REPL: {}", self.lang_version, self.version);
        println!("Type quit() or press CTRL + C to exit.\n");

        let mut input_text = String::new();
        loop {
            print!("> ");
            io::stdout().flush().expect("Stdout error");

            input_text.clear();
            io::stdin().read_line(&mut input_text).expect("Stdin error");

            if input_text.contains("quit()") {
                break;
            }

            if let Ok(result) = calculate(input_text.trim()) {
                println!("{:?}", result);
            }
        }
    }
}
