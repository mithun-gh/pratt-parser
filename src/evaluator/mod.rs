pub mod lexer;
pub mod parser;

pub fn calculate(expr: &str) -> Option<parser::Expression> {
    let tokens = lexer::lex(expr.to_string()).unwrap();
    parser::parse(tokens).unwrap()
}
