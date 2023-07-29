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
                self.advance_with_type(TokenType::LParen)?;
                let node = self.parse_expression()?;
                self.advance_with_type(TokenType::RParen)?;
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

    fn parse_primary_expression(&mut self) -> Result<ast::Expression, ParseError> {
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

        Ok(node)
    }

    fn parse_expression(&mut self) -> Result<ast::Expression, ParseError> {
        let mut node = self.parse_primary_expression()?;

        while self.current_token().token_type == TokenType::Gt
            || self.current_token().token_type == TokenType::Lt
        {
            let tok = self.current_token();
            self.position += 1;

            node = ast::Expression {
                expression_type: ExpressionType::Binary(
                    Box::new(node),
                    tok,
                    Box::new(self.parse_primary_expression()?),
                ),
            };
        }

        Ok(node)
    }

    fn parse_assignment(&mut self) -> Result<ast::Assignment, ParseError> {
        let _ = self.advance_with_type(TokenType::Let)?;
        let identif_token = self.advance_with_type(TokenType::Identif)?;
        let _ = self.advance_with_type(TokenType::Assign)?;

        let expression = self.parse_expression()?;

        let node = Ok(ast::Assignment {
            identif: identif_token.value,
            value: expression,
        });

        self.advance_with_type(TokenType::Semicolon)?;

        return node;
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, ParseError> {
        let mut statements: Vec<ast::Statement> = Vec::new();

        while self.current_token().token_type != TokenType::EOF {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(ast::Program { statements })
    }

    fn parse_block(&mut self) -> Result<Vec<ast::Statement>, ParseError> {
        let mut statements: Vec<ast::Statement> = Vec::new();
        self.advance_with_type(TokenType::LBrace)?;

        while self.current_token().token_type != TokenType::RBrace {
            let statement = self.parse_statement().unwrap();
            statements.push(statement);
        }

        self.advance_with_type(TokenType::RBrace)?;

        Ok(statements)
    }

    pub fn parse_statement(&mut self) -> Result<ast::Statement, ParseError> {
        let token = self.current_token();

        match token.token_type {
            TokenType::Let => {
                let assignment = self.parse_assignment()?;
                return Ok(ast::Statement {
                    statement_type: ast::StatementType::Assignment(assignment),
                });
            }
            TokenType::If => {
                self.advance_with_type(TokenType::If)?;
                self.advance_with_type(TokenType::LParen)?; // (
                let expression = self.parse_expression()?;
                self.advance_with_type(TokenType::RParen)?; // )

                let statements = self.parse_block()?;

                match self.current_token().token_type {
                    TokenType::Else => {
                        self.advance_with_type(TokenType::Else)?;
                        let else_statements = self.parse_block()?;
                        return Ok(ast::Statement {
                            statement_type: ast::StatementType::IfElseBlock(
                                Box::new(expression),
                                Box::new(statements),
                                Box::new(else_statements),
                            ),
                        });
                    }
                    _ => {
                        return Ok(ast::Statement {
                            statement_type: ast::StatementType::IfBlock(
                                Box::new(expression),
                                Box::new(statements),
                            ),
                        });
                    }
                }
            }
            _ => Err(ParseError::new(TokenType::Invalid, TokenType::Invalid)),
        }
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        let program = self.parse_program()?;

        dbg!(program);

        Ok(())
    }

    fn advance_with_type(&mut self, token_type: TokenType) -> Result<Token, ParseError> {
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
