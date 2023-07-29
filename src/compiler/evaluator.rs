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

    pub fn into_bool(&self) -> bool {
        match &self.value_type {
            ValueType::Bool(value) => value.clone(),
            ValueType::Integer(value) => value.clone() != 0,
            ValueType::String(value) => !value.is_empty(),
        }
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
    }

    fn evaluate_statement(&mut self, statement: &Statement) {
        match &statement.statement_type {
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
            ast::StatementType::Print(ref expr) => {
                let value = self.evaluate_expression(expr);
                match value.value_type {
                    ValueType::Integer(value) => println!("{}", value),
                    ValueType::String(value) => println!("{}", value),
                    ValueType::Bool(value) => println!("{}", value),
                }
            }
            ast::StatementType::FunctionDeclaration(name, _, _) => {
                self.functions.insert(name.clone(), statement.clone());
            }
            ast::StatementType::Return(_) => {}
            ast::StatementType::IfBlock(ref expr, body) => {
                let value = self.evaluate_expression(expr).into_bool();
                if value {
                    for statement in body.clone().into_iter() {
                        self.evaluate_statement(&statement);
                    }
                }
            }
            ast::StatementType::IfElseBlock(expr, if_body, else_body) => {
                let value = self.evaluate_expression(expr).into_bool();

                if value {
                    for statement in if_body.clone().into_iter() {
                        self.evaluate_statement(&statement);
                    }
                } else {
                    for statement in else_body.clone().into_iter() {
                        self.evaluate_statement(&statement);
                    }
                }
            }
            _ => {}
        }
    }

    fn evaluate_expression(&mut self, expr: &Expression) -> Value {
        match &expr.expression_type {
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
                    TokenType::Gt => match (left.value_type, right.value_type) {
                        (ValueType::Integer(left), ValueType::Integer(right)) => {
                            Value::new(ValueType::Bool(left > right))
                        }
                        _ => panic!("hihi"),
                    },
                    TokenType::Lt => match (left.value_type, right.value_type) {
                        (ValueType::Integer(left), ValueType::Integer(right)) => {
                            Value::new(ValueType::Bool(left < right))
                        }
                        _ => panic!("hihi"),
                    },
                    _ => panic!("Cannot evaluate expression"),
                }
            }
            ast::ExpressionType::FunctionCall(name, args) => {
                let function = self.functions.get(name).unwrap();
                let params = match function.statement_type {
                    ast::StatementType::FunctionDeclaration(_, ref params, _) => params.clone(),
                    _ => panic!("Cannot evaluate expression"),
                };

                self.evaluate_function_call(name.clone(), params, *args.clone())
            }
            _ => panic!("Cannot evaluate expression"),
        }
    }

    fn evaluate_function_call(
        &mut self,
        name: String,
        params: Vec<String>,
        args: Vec<Expression>,
    ) -> Value {
        let body = match self.functions.get(&name).unwrap().statement_type {
            ast::StatementType::FunctionDeclaration(_, _, ref body) => body.clone(),
            _ => panic!("Cannot evaluate expression"),
        };
        let mut variable_stack = HashMap::new();
        for (param, arg) in params.iter().zip(args.iter()) {
            let value = self.evaluate_expression(arg);
            variable_stack.insert(param.clone(), value);
        }
        self.variable_stack.push(variable_stack);

        let mut return_value = Value::new(ValueType::Integer(0));
        for statement in body.into_iter() {
            match statement.statement_type {
                ast::StatementType::Return(ref expr) => match expr {
                    Some(expr) => {
                        return_value = self.evaluate_expression(expr);
                    }
                    None => {
                        return_value = Value::new(ValueType::Integer(0));
                    }
                },
                _ => self.evaluate_statement(&statement),
            }
        }

        self.variable_stack.pop();

        return_value
    }
}
