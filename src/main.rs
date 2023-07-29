mod error;

mod compiler;

use compiler::{
    lexer::Lexer,
    parser::{ParseError, Parser},
};

fn main() -> Result<(), ParseError> {
    let input = std::fs::read_to_string("test.ilu").unwrap();
    let mut lexer = Lexer::new(&input);

    let tokens = lexer.lex();

    let mut parser = Parser::new(tokens);
    parser.parse()
}
