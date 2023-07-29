#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // types
    String,
    Number,
    // keywords
    Let,
    If,
    Else,
    // identif
    Identif,
    // symbols
    RParen,
    LParen,
    RBrace,
    LBrace,
    Semicolon,
    Assign,
    // operators
    Minus,
    Plus,
    Asterisk,
    Slash,
    Gt,
    Lt,
    //
    EOF,
    Invalid,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }
}
