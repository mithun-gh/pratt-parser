pub mod lexer;

pub fn calculate(expr: &str) -> Vec<lexer::Token> {
  lexer::lex(expr.to_string()).unwrap()
}
