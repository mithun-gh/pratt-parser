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

#[derive(Debug)]
pub enum ParserError {
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

fn match_operator(token: Token) -> Result<Operator, ParserError> {
    match token {
        Token::Punctuator('+') => Ok(Operator::Add),
        Token::Punctuator('-') => Ok(Operator::Sub),
        Token::Punctuator('*') => Ok(Operator::Mul),
        Token::Punctuator('/') => Ok(Operator::Div),
        _ => Err(ParserError::InvalidToken(token)),
    }
}
