use crate::example::ExampleLexer;

pub mod lex;

mod example;

fn main() {
    // Create the lexer from an str reference
    let lexer = ExampleLexer::new("hello my name is taz and i am 28 years old");

    // Consume the lexer
    println!("Lexing");
    lexer.for_each(|token| println!("Token: {:?}", token));
    println!("Done lexing");
}
