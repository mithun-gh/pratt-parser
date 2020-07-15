use crate::interpreter::parser::Expression;

#[derive(Debug, Copy, Clone)]
pub enum ArithmaticError {
    DivideByZero,
}

pub fn evaluate(_expr: Expression) -> Result<f64, ArithmaticError> {
    Ok(0.0)
}
