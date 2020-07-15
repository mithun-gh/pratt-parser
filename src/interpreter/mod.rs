pub mod lexer;
pub mod parser;
pub mod eval;

pub fn calculate(expr: &str) -> Result<f64, eval::ArithmaticError> {
    let tokens = lexer::lex(expr.to_string()).unwrap();
    let mut parser = parser::Parser::new(tokens.iter());
    let expr = parser.parse().unwrap().unwrap();
    eval::evaluate(expr)
}
