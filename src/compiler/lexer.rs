#[allow(dead_code)]
pub use super::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    position: usize,
    tmp_buffer: String,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            source: input,
            position: 0,
            tmp_buffer: String::new(),
        }
    }
}

impl<'a> Lexer<'a> {
    fn peek(&self) -> Option<char> {
        if self.position + 1 >= self.source.len() {
            return None;
        }
        return self.source.chars().nth(self.position + 1);
    }

    fn parse_keyword(&self) -> TokenType {
        match self.tmp_buffer.as_str() {
            "let" => TokenType::Let,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            _ => TokenType::Identif,
        }
    }

    fn parse_number(&mut self) -> Token {
        let mut current_char = self.source.chars().nth(self.position).unwrap();

        while self.position < self.source.len() && current_char.is_numeric() {
            self.tmp_buffer.push(current_char);
            self.position += 1;
            current_char = self.source.chars().nth(self.position).unwrap();

            if !current_char.is_alphanumeric() {
                self.position -= 1;
            }
        }
        let token = Token::new(TokenType::Number, self.tmp_buffer.clone());
        self.tmp_buffer.clear();

        return token;
    }

    fn parse_string(&mut self) -> Token {
        self.position += 1;

        let mut current_char = self.source.chars().nth(self.position).unwrap();

        while self.position < self.source.len() && current_char != '\"' {
            self.tmp_buffer.push(current_char);
            self.position += 1;
            current_char = self.source.chars().nth(self.position).unwrap();
        }

        let token = Token::new(TokenType::String, self.tmp_buffer.clone());
        self.tmp_buffer.clear();
        return token;
    }

    fn parse_identif(&mut self) -> Token {
        let mut current_char = self.source.chars().nth(self.position).unwrap();

        while self.position < self.source.len() && current_char.is_alphanumeric() {
            self.tmp_buffer.push(current_char);
            self.position += 1;
            current_char = self.source.chars().nth(self.position).unwrap();
        }

        let token_type = self.parse_keyword();

        let token = Token::new(token_type, self.tmp_buffer.clone());
        self.tmp_buffer.clear();
        return token;
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut current_char: char;
        let mut tokens: Vec<Token> = Vec::new();

        while self.position < self.source.len() {
            current_char = self.source.chars().nth(self.position).unwrap();

            let token: Token = match current_char {
                '\"' => self.parse_string(),
                '0'..='9' => self.parse_number(),
                'a'..='z' | 'A'..='Z' => self.parse_identif(),
                ')' => Token::new(TokenType::RParen, current_char.to_string()),
                '(' => Token::new(TokenType::LParen, current_char.to_string()),
                '{' => Token::new(TokenType::LBrace, current_char.to_string()),
                '}' => Token::new(TokenType::RBrace, current_char.to_string()),
                '=' => Token::new(TokenType::Assign, current_char.to_string()),
                ';' => Token::new(TokenType::Semicolon, current_char.to_string()),
                '+' => Token::new(TokenType::Plus, current_char.to_string()),
                '-' => Token::new(TokenType::Minus, current_char.to_string()),
                '*' => Token::new(TokenType::Asterisk, current_char.to_string()),
                '/' => Token::new(TokenType::Slash, current_char.to_string()),
                '>' => Token::new(TokenType::Gt, current_char.to_string()),
                '<' => Token::new(TokenType::Lt, current_char.to_string()),
                _ => {
                    self.position += 1;
                    continue;
                }
            };
            tokens.push(token);

            self.position += 1;
        }
        return tokens;
    }
}
