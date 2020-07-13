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

pub fn parse(tokens: Vec<Token>) -> Result<Option<Expression>, ParserError> {
    let mut expr = None;
    let mut tokens = tokens.iter().peekable();

    while let Some(token) = tokens.next() {
        match token {
            Token::Number(n) => {
                expr = Some(Expression::Number(*n));
            }
            _ => {}
        }
    }

    Ok(expr)
}
