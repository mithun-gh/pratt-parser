use crate::evaluator::lexer::Token;
use std::iter::Peekable;
use std::slice::Iter;

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
    fn to_number(&self) -> Result<Expression, ParserError> {
        match self {
            Partial::Expression(number_expr) => Ok(number_expr.clone()),
            _ => Err(ParserError::UnexpectedError),
        }
    }

    fn to_operator(&self) -> Result<Operator, ParserError> {
        match self {
            Partial::Operator(operator) => Ok(*operator),
            _ => Err(ParserError::UnexpectedError),
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedError,
    InvalidToken(Token),
    UnexpectedEndOfInput,
}

// 2 + 3 * 4
// >   >   <
pub fn parse(tokens: Vec<Token>) -> Result<Option<Expression>, ParserError> {
    let mut op = None;
    let mut expr: Option<Expression> = None;
    let mut tokens = tokens.iter().peekable();

    while let Some(token) = tokens.next() {
        match token {
            Token::Number(n) => {
                if let Some(ref left) = expr {
                    if let Some(root) = op {
                        let left = Box::new(left.clone());
                        let right = Box::new(Expression::Number(*n));
                        expr = Some(Expression::Binary(left, root, right));
                    }
                } else {
                    expr = Some(Expression::Number(*n))
                }
            }
            Token::Punctuator(_) => {
                op = Some(match_operator(*token)?)
            }
        }
    }

    Ok(expr)
}

// 2
// 2 + 3 * 4
// 2 * 3 + 4
// 2, +, (3 * 4)
// (2 * 3), +, 4
fn parse_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Option<Expression>, ParserError> {
    let mut stack = Vec::<Partial>::new();

    while let Some(&token) = tokens.next() {
        match token {
            Token::Number(n) => {
                if let Some(_) = tokens.peek() {
                    let op = match_operator(*tokens.next().unwrap())?;

                    if let Some(Partial::Operator(prev_op)) = stack.last() {
                        if op.lbp() > prev_op.lbp() {
                            if let Some(Token::Number(n2)) = tokens.next() {
                                let left = Box::new(Expression::Number(n));
                                let right = Box::new(Expression::Number(*n2));
                                stack.push(Partial::Expression(Expression::Binary(left, op, right)));
                            } else {
                                return Err(ParserError::UnexpectedEndOfInput);
                            }
                        } else {
                            let op = stack.pop().unwrap().to_operator()?;
                            let left = Box::new(stack.pop().unwrap().to_number()?);
                            let right = Box::new(Expression::Number(n));
                            stack.push(Partial::Expression(Expression::Binary(left, op, right)));
                            stack.push(Partial::Operator(op));
                        }
                    } else {
                        stack.push(Partial::Expression(Expression::Number(n)));
                        stack.push(Partial::Operator(op));
                    }

                } else {
                    stack.push(Partial::Expression(Expression::Number(n)));
                }
            }
            Token::Punctuator(_) => {
                stack.push(Partial::Operator(match_operator(token)?));
            }
        }
    }

    Err(ParserError::UnexpectedEndOfInput)
}

fn match_operator(token: Token) -> Result<Operator, ParserError> {
    match token {
        Token::Punctuator('+') => Ok(Operator::Add),
        Token::Punctuator('-') => Ok(Operator::Sub),
        Token::Punctuator('*') => Ok(Operator::Mul),
        Token::Punctuator('/') => Ok(Operator::Div),
        _ => Err(ParserError::InvalidToken(token)),
    }
}
