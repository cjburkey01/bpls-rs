use crate::lex::{Lexer, Token};
use std::str::CharIndices;

// Token

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExampleToken {
    Whitespace(usize),
    String(String),
    Number(String),
}

impl Token for ExampleToken {}

// Lexer

pub struct ExampleLexer<'top> {
    input: CharIndices<'top>,
    at: usize,
}

impl<'top> ExampleLexer<'top> {
    pub fn new(input: &'top str) -> Self {
        Self {
            input: input.char_indices(),
            at: 0,
        }
    }
}

impl<'top> Iterator for ExampleLexer<'top> {
    type Item = ExampleToken;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<'top> Lexer<ExampleToken> for ExampleLexer<'top> {
    fn get_pos(&self) -> usize {
        self.at
    }
}
