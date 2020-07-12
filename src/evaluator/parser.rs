use crate::evaluator::lexer::Token;

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Expression {
    Number(f64),
    Binary(Box<Expression>, Operator, Box<Expression>),
}

#[derive(Debug)]
pub enum ParserError {
    InvalidToken(Token),
}

pub fn parse(_tokens: Vec<Token>) -> Result<Expression, ParserError> {
    Ok(Expression::Number(0.0))
}
