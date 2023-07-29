mod error;

mod compiler;

use compiler::{
    evaluator::Evaluator,
    lexer::Lexer,
    parser::{ParseError, Parser},
};

fn main() -> Result<(), ParseError> {
    let args = std::env::args().collect::<Vec<String>>();

    let input = std::fs::read_to_string(&args[1]).unwrap();
    let mut lexer = Lexer::new(&input);

    let tokens = lexer.lex();

    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    let mut evaluator = Evaluator::new(program);

    evaluator.evaluate();

    Ok(())
}
