pub mod lexer;
pub mod parser;
pub mod eval;

pub fn calculate(expr: &str) -> Option<parser::Expression> {
    let tokens = lexer::lex(expr.to_string()).unwrap();
    let mut parser = parser::Parser::new(tokens.iter());
    parser.parse().unwrap()
}
