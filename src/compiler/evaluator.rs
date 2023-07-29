use super::{
    ast::{self, Expression, Program, Statement},
    token::TokenType,
};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Value {
    pub value_type: ValueType,
}

#[derive(Debug, Clone)]
pub enum ValueType {
    Integer(i64),
    String(String),
    Bool(bool),
}

impl Value {
    pub fn new(value_type: ValueType) -> Self {
        Self { value_type }
    }
}

pub struct Evaluator {
    pub program: Program,
    pub variable_stack: Vec<HashMap<String, Value>>,
    pub functions: HashMap<String, Statement>,
}

impl Evaluator {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            variable_stack: vec![HashMap::new()],
            functions: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self) {
        let statements = self.program.statements.clone();
        for statement in statements {
            self.evaluate_statement(&statement);
        }
        dbg!(&self.variable_stack);
    }

    fn evaluate_statement(&mut self, statement: &Statement) {
        match statement.statement_type {
            ast::StatementType::Declaration(ref declaration) => {
                let value = self.evaluate_expression(&declaration.value);
                self.variable_stack
                    .last_mut()
                    .unwrap()
                    .insert(declaration.identif.clone(), value);
            }
            ast::StatementType::Assignment(ref assignment) => {
                let value = self.evaluate_expression(&assignment.value);
                self.variable_stack
                    .last_mut()
                    .unwrap()
                    .insert(assignment.identif.clone(), value);
            }
            _ => {}
        }
    }

    fn evaluate_expression(&mut self, expr: &Expression) -> Value {
        match expr.expression_type {
            ast::ExpressionType::Literal(ref literal) => match literal.parse::<i64>() {
                Ok(integer) => Value::new(ValueType::Integer(integer)),
                Err(_) => Value::new(ValueType::String(literal.clone())),
            },
            ast::ExpressionType::Variable(ref variable) => {
                let variable_stack = self.variable_stack.last().unwrap();
                let value = variable_stack.get(variable).unwrap();
                value.clone()
            }
            ast::ExpressionType::Binary(ref left, ref operator, ref right) => {
                let left = self.evaluate_expression(left);
                let right = self.evaluate_expression(right);

                match operator.token_type {
                    TokenType::Plus => match (left.value_type, right.value_type) {
                        (ValueType::Integer(left), ValueType::Integer(right)) => {
                            Value::new(ValueType::Integer(left + right))
                        }
                        (ValueType::String(left), ValueType::String(right)) => {
                            Value::new(ValueType::String(left + &right))
                        }
                        _ => panic!("hihi"),
                    },
                    TokenType::Minus => match (left.value_type, right.value_type) {
                        (ValueType::Integer(left), ValueType::Integer(right)) => {
                            Value::new(ValueType::Integer(left - right))
                        }
                        _ => panic!("hihi"),
                    },
                    TokenType::Asterisk => match (left.value_type, right.value_type) {
                        (ValueType::Integer(left), ValueType::Integer(right)) => {
                            Value::new(ValueType::Integer(left * right))
                        }
                        _ => panic!("hihi"),
                    },
                    TokenType::Slash => match (left.value_type, right.value_type) {
                        (ValueType::Integer(left), ValueType::Integer(right)) => {
                            Value::new(ValueType::Integer(left / right))
                        }
                        _ => panic!("hihi"),
                    },
                    _ => panic!("Cannot evaluate expression"),
                }
            }
            _ => panic!("Cannot evaluate expression"),
        }
    }
}
