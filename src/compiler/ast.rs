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
    Declaration(Declaration),
    FunctionCall(String, Box<Vec<Expression>>),
    Assignment(Assignment),
    IfBlock(Box<Expression>, Box<Vec<Statement>>),
    IfElseBlock(Box<Expression>, Box<Vec<Statement>>, Box<Vec<Statement>>),
    FunctionDeclaration(String, Vec<String>, Box<Vec<Statement>>),
    Return(Option<Box<Expression>>),
}

#[derive(Debug)]
pub struct Declaration {
    pub identif: String,
    pub value: Expression,
}

#[derive(Debug)]
pub struct Assignment {
    pub identif: String,
    pub value: Expression,
}

#[derive(Debug)]
pub enum ExpressionType {
    Literal(String),
    Variable(String),
    Binary(Box<Expression>, Token, Box<Expression>),
    Unary(Token, Box<Expression>),
    FunctionCall(String, Box<Vec<Expression>>),
}

#[derive(Debug)]
pub struct Expression {
    pub expression_type: ExpressionType,
}
