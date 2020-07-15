use std::slice::Iter;
use std::iter::Peekable;
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

impl Token {
    pub fn bp(&self) -> i32 {
        match self {
            Token::Punctuator('+') => 10,
            Token::Punctuator('-') => 10,
            Token::Punctuator('*') => 20,
            Token::Punctuator('/') => 20,
            _ => 0,
        }
    }

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

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Iter<'a, Token>) -> Self {
        Parser { tokens: tokens.peekable() }
    }

    pub fn nud(&self, token: Token) -> Result<Expression, ParserError> {
        match token {
            Token::Number(n) => Ok(Expression::Number(n)),
            _ => Err(ParserError::UnexpectedError),
        }
    }

    pub fn led(&mut self, left: Expression, op: Operator) -> Result<Expression, ParserError> {
        if let Some(right) = self.parse()? {
            return Ok(Expression::Binary(Box::new(left), op, Box::new(right)));
        } else {
            return Err(ParserError::UnexpectedError);
        }
    }

    // function expr(rbp = 0) {
    //     let left = nud(next())
    //     while (bp(peek()) > rbp)
    //         left = led(left, next())
    //     return left
    // }
    pub fn parse(&mut self) -> Result<Option<Expression>, ParserError> {
        if let Some(&curr) = self.tokens.next() {
            let mut left = self.nud(curr)?;

            while let Some(&&next) = self.tokens.peek() {
                if next.bp() < curr.bp() {
                    break;
                }
                self.tokens.next();
                left = self.led(left, next.to_operator()?)?;
            }

            return Ok(Some(left));
        }

        Ok(None)
    }
}

pub fn parse_expr() -> Result<Option<Expression>, ParserError> {
    Ok(None)
}

pub fn parse(_tokens: Vec<Token>) -> Result<Option<Expression>, ParserError> {
    Ok(None)
}
