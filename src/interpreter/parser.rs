use crate::interpreter::lexer::Token;
use std::iter::Peekable;
use std::slice::Iter;

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
    Mod,
    Exp,
}

impl Token {
    pub fn bp(&self) -> Result<i32, ParserError> {
        match self {
            Token::Punctuator('+') => Ok(1),
            Token::Punctuator('-') => Ok(1),
            Token::Punctuator('*') => Ok(2),
            Token::Punctuator('/') => Ok(2),
            Token::Punctuator('%') => Ok(2),
            Token::Punctuator('^') => Ok(3),
            _ => Err(ParserError::UnexpectedError),
        }
    }

    pub fn to_operator(&self) -> Result<Operator, ParserError> {
        match self {
            Token::Punctuator('+') => Ok(Operator::Add),
            Token::Punctuator('-') => Ok(Operator::Sub),
            Token::Punctuator('*') => Ok(Operator::Mul),
            Token::Punctuator('/') => Ok(Operator::Div),
            Token::Punctuator('%') => Ok(Operator::Mod),
            Token::Punctuator('^') => Ok(Operator::Exp),
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
        Parser {
            tokens: tokens.peekable(),
        }
    }

    pub fn nud(&self, token: Token) -> Result<Expression, ParserError> {
        match token {
            Token::Number(n) => Ok(Expression::Number(n)),
            _ => Err(ParserError::UnexpectedError),
        }
    }

    pub fn led(&mut self, left: Expression, op: Token) -> Result<Expression, ParserError> {
        let bp = op.bp()? - if op == Token::Punctuator('^') { 1 } else { 0 };
        if let Some(right) = self.parse_expr(bp)? {
            Ok(Expression::Binary(
                Box::new(left),
                op.to_operator()?,
                Box::new(right),
            ))
        } else {
            Err(ParserError::UnexpectedError)
        }
    }

    pub fn parse(&mut self) -> Result<Option<Expression>, ParserError> {
        self.parse_expr(0)
    }

    fn parse_expr(&mut self, prev_bp: i32) -> Result<Option<Expression>, ParserError> {
        if let Some(&curr) = self.tokens.next() {
            let left = match curr {
                Token::Punctuator('(') => self.parse_expr(0)?,
                Token::Number(_) => Some(self.nud(curr)?),
                _ => None,
            };

            if left.is_none() {
                return Ok(None);
            }

            let mut left = left.unwrap();
            while let Some(&&next) = self.tokens.peek() {
                if next.bp()? <= prev_bp {
                    break;
                }
                self.tokens.next();
                left = self.led(left, next)?;
            }

            return Ok(Some(left));
        }

        Ok(None)
    }
}
