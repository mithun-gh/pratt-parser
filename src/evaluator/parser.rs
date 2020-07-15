use crate::evaluator::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Binary(Box<Expression>, Operator, Box<Expression>),
}

#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn lbp(&self) -> i32 {
        match self {
            Operator::Add => 10,
            Operator::Sub => 10,
            Operator::Mul => 20,
            Operator::Div => 20,
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

    pub fn nud(&self) -> Result<Expression, ParserError> {
        match self {
            Token::Number(n) => Ok(Expression::Number(*n)),
            _ => Err(ParserError::UnexpectedError)
        }
    }

    pub fn led(&self, left: Expression, op: Operator) -> Result<Expression, ParserError> {
        if let Some(right) = parse_expr()? {
            return Ok(Expression::Binary(Box::new(left), op, Box::new(right)));
        } else {
            return Err(ParserError::UnexpectedError);
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedError,
    InvalidToken(Token),
    UnexpectedEndOfInput,
}

pub fn parse_expr() -> Result<Option<Expression>, ParserError> {
    Ok(None)
}

pub fn parse(_tokens: Vec<Token>) -> Result<Option<Expression>, ParserError> {
    Ok(None)
}
