#[derive(Debug)]
pub enum TokenType {
    // types
    String,
    Number,
    // keywords
    Let,
    // identif
    Identif,
    // symbols
    RParen,
    LParen,
    RBrace,
    Lbrace,
    Semicolon,
    Assign,
    // operators
    Minus,
    Plus,
    Asterisk,
    Slash,
}
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }
}
