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
    precedence: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
            precedence: 0,
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

    fn is_operator(&self, token: &Token) -> bool {
        match token.token_type {
            TokenType::Plus => true,
            TokenType::Minus => true,
            TokenType::Asterisk => true,
            TokenType::Slash => true,
            _ => false,
        }
    }

    fn is_literal(&self, token: &Token) -> bool {
        match token.token_type {
            TokenType::Number => true,
            TokenType::String => true,
            _ => false,
        }
    }

    fn precedence(&self, token: &Token) -> usize {
        match token.token_type {
            TokenType::Plus => 1,
            TokenType::Minus => 1,
            TokenType::Asterisk => 2,
            TokenType::Slash => 2,
            _ => 0,
        }
    }

    fn token_advance(&mut self) -> Token {
        if self.position >= self.tokens.len() {
            return Token::new(TokenType::EOF, String::new());
        }

        let token = self.tokens[self.position].clone();
        self.position += 1;
        token
    }

    fn current_token(&self) -> Token {
        if self.position >= self.tokens.len() {
            return Token::new(TokenType::EOF, String::new());
        }

        self.tokens[self.position].clone()
    }

    fn parse_factor(&mut self) -> Result<ast::Expression, ParseError> {
        let token = self.current_token();

        match token.token_type {
            TokenType::Number => {
                self.position += 1;
                Ok(ast::Expression {
                    expression_type: ExpressionType::Literal(token.value),
                })
            }
            TokenType::LParen => {
                self.expect_with_type(TokenType::LParen)?;
                let node = self.parse_expression()?;
                self.expect_with_type(TokenType::RParen)?;
                return Ok(node);
            }
            TokenType::Plus | TokenType::Minus => {
                self.position += 1;
                let node = self.parse_factor()?;
                return Ok(ast::Expression {
                    expression_type: ExpressionType::Unary(token, Box::new(node)),
                });
            }
            _ => Err(ParseError::new(TokenType::Invalid, TokenType::Invalid)),
        }
    }

    fn parse_term(&mut self) -> Result<ast::Expression, ParseError> {
        let mut node = self.parse_factor()?;

        while self.current_token().token_type == TokenType::Asterisk
            || self.current_token().token_type == TokenType::Slash
        {
            let tok = self.current_token();
            self.position += 1;
            let right = self.parse_factor()?;
            node = ast::Expression {
                expression_type: ExpressionType::Binary(Box::new(node), tok, Box::new(right)),
            };
        }

        return Ok(node);
    }

    fn parse_expression(&mut self) -> Result<ast::Expression, ParseError> {
        let mut node = self.parse_term()?;

        while self.current_token().token_type == TokenType::Plus
            || self.current_token().token_type == TokenType::Minus
        {
            let tok = self.current_token();
            self.position += 1;

            node = ast::Expression {
                expression_type: ExpressionType::Binary(
                    Box::new(node),
                    tok,
                    Box::new(self.parse_term()?),
                ),
            };
        }

        let _ = self.expect_with_type(TokenType::Semicolon);

        Ok(node)
    }

    fn parse_assignment(&mut self) -> Result<ast::Assignment, ParseError> {
        let _ = self.expect_with_type(TokenType::Let)?;
        let identif_token = self.expect_with_type(TokenType::Identif)?;
        let _ = self.expect_with_type(TokenType::Assign)?;

        let expression = self.parse_expression()?;

        Ok(ast::Assignment {
            identif: identif_token.value,
            value: expression,
        })
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        while self.position < self.tokens.len() {
            let token = self.current_token();
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
