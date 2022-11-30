use crate::lex::{Lexer, LexingError, Token, TokenType};
use bpls_rs_macro::try_consume_token;
use std::str::CharIndices;

// Token

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExampleToken {
    Whitespace,
    String(String),
    Number(u64),
    EOI,
}

impl TokenType<char> for ExampleToken {
    fn get_eoi() -> Self {
        ExampleToken::EOI
    }
}

// Lexer

pub struct ExampleLexer<'top> {
    input: CharIndices<'top>,
    at: usize,
    c1: Option<(usize, char)>,
    c2: Option<(usize, char)>,
    done: bool,
}

impl<'top> ExampleLexer<'top> {
    pub fn new_from_ci(input: CharIndices<'top>) -> Self {
        // Initialize a new lexer
        let mut new = Self {
            input,
            at: 0,
            c1: None,
            c2: None,
            done: false,
        };

        // Fill `current1` and `current2`
        new.next_char();
        new.next_char();

        // Return
        new
    }

    pub fn new_from_str(input: &'top str) -> Self {
        Self::new_from_ci(input.char_indices())
    }

    /// Increment this lexer by one character
    fn next_char(&mut self) {
        // Move the first to the second
        self.c2 = self.c1;

        // Update lexer position
        if let Some((at, _)) = self.c2 {
            self.at = at;
        }

        // Get the next character
        self.c1 = self.input.next();
    }
}

impl<'top> Iterator for ExampleLexer<'top> {
    type Item = Result<Token<ExampleToken>, LexingError<char>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c2) = self.c2 {
            if let t_exp @ Some(_) = match c2.1 {
                'A'..='Z' | 'a'..='z' => {
                    let start = self.at;
                    let mut token = c2.1.to_string();

                    try_consume_token!(self, c1, ('A'..='Z' | 'a'..='z') => token.push(c1.1));

                    Some(Ok(Token::new(
                        (start, start + token.len()),
                        ExampleToken::String(token),
                    )))
                }

                ' ' | '\t' | '\n' | '\r' => {
                    let start = self.at;
                    let mut end = start + 1;

                    try_consume_token!(self, c1, (' ' | '\t' | '\n' | '\r') => end += 1);

                    Some(Ok(Token::new((start, end), ExampleToken::Whitespace)))
                }
                '1'..='9' => {
                    let start = self.at;
                    let mut end = start + 1;
                    let mut val = c2.1.to_digit(10).unwrap() as u64;

                    try_consume_token!(self, c1, '0'..='9' => {
                        end += 1;
                        val *= 10;
                        val += c1.1.to_digit(10).unwrap() as u64;
                    });

                    Some(Ok(Token::new((start, end), ExampleToken::Number(val))))
                }
                _ => Some(Err(LexingError::UnexpectedInput {
                    input: c2.1,
                    index: c2.0,
                })),
            } {
                self.next_char();
                return t_exp;
            }
        }
        if !self.done {
            self.done = true;
            return Some(Ok(Token::new(
                (self.at + 1, self.at + 1),
                ExampleToken::EOI,
            )));
        }
        None
    }
}

impl<'top> Lexer<char, ExampleToken> for ExampleLexer<'top> {
    fn get_pos(&self) -> usize {
        self.at
    }
}
