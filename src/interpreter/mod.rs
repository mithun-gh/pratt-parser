pub mod lexer;
pub mod parser;
pub mod eval;

pub fn calculate(expr: &str) -> Option<f64> {
    let tokens = lexer::lex(expr.to_string()).unwrap();
    let mut parser = parser::Parser::new(tokens.iter());

    if let Some(expr) = parser.parse().unwrap() {
        return Some(eval::evaluate(expr).unwrap());
    }

    None
}
