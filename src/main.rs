#![feature(or_patterns)]

use example2::ExampleToken;

pub mod lex;

mod example1;
mod example2;

fn main() {
    let input = "hello  my name is Taz and i am 28 years old";

    // Create the lexer from an str reference
    //let lexer = ExampleLexer::new_from_str(input);
    let lexer = ExampleToken::new_lexer_from_str(input);

    // Consume the lexer to get the tokens
    println!("Lexing \"{}\"", input);
    lexer.for_each(|token| match token {
        Ok(token) => match token.token_type {
            ExampleToken::Whitespace => {}
            _ => println!("{:?}", token),
        },
        Err(err) => println!("{:?}", err),
    });
    println!("Done lexing");
}
