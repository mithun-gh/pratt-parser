use rustyline::{Editor, KeyPress, Cmd, error::ReadlineError};
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
        println!("Type 'q' or press CTRL+C to exit.\n");

        let mut rl = Editor::<()>::new();
        rl.bind_sequence(KeyPress::Char('c'), Cmd::ClearScreen);
        rl.bind_sequence(KeyPress::Char('q'), Cmd::Interrupt);

        loop {
            match rl.readline("> ") {
                Ok(ref input_text) => {
                    rl.add_history_entry(input_text);
                    print_result(input_text);
                },
                Err(ReadlineError::Interrupted) => break,
                Err(err) => {
                    eprintln!("{:?}", err);
                    break;
                },
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
