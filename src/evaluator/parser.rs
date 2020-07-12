pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Expression {
    Number(f64),
    Binary(Box<Expression>, Operator, Box<Expression>),
}
