#[derive(Debug)]
pub struct Assignment {
    pub identif: String,
    pub value: Expression,
}

#[derive(Debug)]
pub enum ExpressionType {
    Literal(String),
    Binary(Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub struct Expression {
    pub expression_type: ExpressionType,
}
