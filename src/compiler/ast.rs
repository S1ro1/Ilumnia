use super::token::Token;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Statement {
    pub statement_type: StatementType,
}

#[derive(Debug)]
pub enum StatementType {
    Assignment(Assignment),
    IfBlock(Box<Expression>, Box<Vec<Statement>>),
}

#[derive(Debug)]
pub struct Assignment {
    pub identif: String,
    pub value: Expression,
}

#[derive(Debug)]
pub enum ExpressionType {
    Literal(String),
    Binary(Box<Expression>, Token, Box<Expression>),
    Unary(Token, Box<Expression>),
}

#[derive(Debug)]
pub struct Expression {
    pub expression_type: ExpressionType,
}
