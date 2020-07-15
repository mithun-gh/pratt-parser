use crate::interpreter::parser::{Expression, Operator};

impl Operator {
    fn eval(&self, left: f64, right: f64) -> f64 {
        match self {
            Operator::Add => left + right,
            Operator::Sub => left - right,
            Operator::Mul => left * right,
            Operator::Div => left / right,
            Operator::Mod => left % right,
            Operator::Exp => left.powf(right),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ArithmaticError {
    DivideByZero,
}

pub fn evaluate(expr: Expression) -> Result<f64, ArithmaticError> {
    match expr {
        Expression::Number(n) => Ok(n),
        Expression::Binary(left, op, right) => {
            Ok(op.eval(evaluate(*left)?, evaluate(*right)?))
        }
    }
}
