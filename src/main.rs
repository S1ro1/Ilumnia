mod error;

mod compiler;

use std::error::Error;

use compiler::lexer::Lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("test.ilu")?;
    let mut lexer = Lexer::new(&input);

    let tokens = lexer.lex();

    println!("{:?}", tokens);

    Ok(())
}
