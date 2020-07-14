use crate::evaluator::lexer::Token;

#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    #[allow(dead_code)]
    fn lbp(&self) -> i32 {
        match self {
            Operator::Add => 10,
            Operator::Sub => 10,
            Operator::Mul => 20,
            Operator::Div => 20,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Binary(Box<Expression>, Operator, Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Partial {
    Operator(Operator),
    Expression(Expression),
}

impl Partial {
    pub fn to_number(&self) -> Result<Expression, ParserError> {
        match self {
            Partial::Expression(number_expr) => Ok(number_expr.clone()),
            _ => Err(ParserError::UnexpectedError),
        }
    }

    pub fn to_operator(&self) -> Result<Operator, ParserError> {
        match self {
            Partial::Operator(operator) => Ok(*operator),
            _ => Err(ParserError::UnexpectedError),
        }
    }
}

impl Token {
    pub fn to_operator(&self) -> Result<Operator, ParserError> {
        match self {
            Token::Punctuator('+') => Ok(Operator::Add),
            Token::Punctuator('-') => Ok(Operator::Sub),
            Token::Punctuator('*') => Ok(Operator::Mul),
            Token::Punctuator('/') => Ok(Operator::Div),
            _ => Err(ParserError::InvalidToken(*self)),
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedError,
    InvalidToken(Token),
    UnexpectedEndOfInput,
}

pub fn parse(_tokens: Vec<Token>) -> Result<Option<Expression>, ParserError> {
    Ok(None)
}
