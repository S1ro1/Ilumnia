use std::fmt;

use super::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub msg: String,
}

impl ParseError {
    pub fn new(expected: TokenType) -> ParseError {
        ParseError {
            msg: format!("Expected token: {:?}", expected),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expected token: {:?}", self.msg)
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
    fn parse_assignment(&mut self) -> Result<(), ParseError> {
        let let_token = self.expect_with_type(TokenType::Let)?;
        let identif_token = self.expect_with_type(TokenType::Identif)?;
        let assign_token = self.expect_with_type(TokenType::Assign)?;
        let number_token = self.expect_with_type(TokenType::Number)?;
        let semicolon_token = self.expect_with_type(TokenType::Semicolon)?;

        println!("let_token: {:?}", let_token);
        println!("identif_token: {:?}", identif_token);

        Ok(())
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        while self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            match token.token_type {
                TokenType::Let => {
                    self.parse_assignment()?;
                }
                _ => Err(ParseError::new(TokenType::RParen))?,
            }
        }

        Ok(())
    }

    fn expect_with_type(&mut self, token_type: TokenType) -> Result<Token, ParseError> {
        if self.position >= self.tokens.len() {
            return Err(ParseError::new(token_type));
        } else {
            let token = self.tokens[self.position].clone();
            if token.token_type == token_type {
                self.position += 1;
                return Ok(token);
            } else {
                return Err(ParseError::new(token_type));
            }
        }
    }
}
