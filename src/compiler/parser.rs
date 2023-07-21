use std::fmt;

use super::ast::{self, Expression, ExpressionType};
use super::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub msg: String,
}

#[derive(Debug, Clone)]
pub struct HmmgeError;

impl ParseError {
    pub fn new(expected: TokenType, gotten: TokenType) -> ParseError {
        ParseError {
            msg: format!("Expected token: {:?} -> Gotten: {:?}", expected, gotten),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.msg)
    }
}

impl fmt::Display for HmmgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HmmgeError")
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
        }
    }
}

impl Parser {
    fn peek_token(&self) -> Token {
        if self.position + 1 >= self.tokens.len() {
            return Token::new(TokenType::EOF, String::new());
        }
        self.tokens[self.position + 1].clone()
    }

    fn is_operator(&self, token: Token) -> bool {
        match token.token_type {
            TokenType::Plus => true,
            TokenType::Minus => true,
            TokenType::Asterisk => true,
            TokenType::Slash => true,
            _ => false,
        }
    }

    fn is_literal(&self, token: Token) -> bool {
        match token.token_type {
            TokenType::Number => true,
            TokenType::String => true,
            _ => false,
        }
    }

    fn parse_expression(&mut self) -> Option<ast::Expression> {
        let mut current_token = self.tokens[self.position].clone();

        if self.is_operator(self.peek_token()) {
            return Some(Expression {
                expression_type: ExpressionType::Binary(
                    Box::new(Expression {
                        expression_type: ExpressionType::Literal(current_token.value),
                    }),
                    Box::new(self.parse_expression()?),
                ),
            });
        } else {
            return Some(Expression {
                expression_type: ExpressionType::Literal(current_token.value),
            });
        }
    }

    fn parse_assignment(&mut self) -> Result<ast::Assignment, ParseError> {
        let _ = self.expect_with_type(TokenType::Let)?;
        let identif_token = self.expect_with_type(TokenType::Identif)?;
        let _ = self.expect_with_type(TokenType::Assign)?;

        let expression = self.parse_expression();

        match expression {
            Some(expression) => Ok(ast::Assignment {
                identif: identif_token.value,
                value: expression,
            }),
            None => Err(ParseError::new(TokenType::Invalid, TokenType::Invalid))?,
        }
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        while self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            match token.token_type {
                TokenType::Let => {
                    dbg!(self.parse_assignment()?);
                }
                _ => Err(ParseError::new(TokenType::Invalid, TokenType::Invalid))?,
            }
        }

        Ok(())
    }

    fn expect_with_type(&mut self, token_type: TokenType) -> Result<Token, ParseError> {
        if self.position >= self.tokens.len() {
            return Err(ParseError::new(token_type, TokenType::EOF));
        } else {
            let token = self.tokens[self.position].clone();
            if token.token_type == token_type {
                self.position += 1;
                return Ok(token);
            } else {
                return Err(ParseError::new(token_type, token.token_type));
            }
        }
    }
}
