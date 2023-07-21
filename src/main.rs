mod error;

mod compiler;

use std::error::Error;

use compiler::{lexer::Lexer, parser::{Parser, ParseError}};

fn main() -> Result<(), ParseError> {
    let input = std::fs::read_to_string("test.ilu").unwrap();
    let mut lexer = Lexer::new(&input);

    let tokens = lexer.lex();

    for token in &tokens {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(tokens);
    parser.parse()
}
