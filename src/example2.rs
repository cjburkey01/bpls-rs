use crate::lex::{Lexer, LexingError, Token, TokenType};

// Token

#[derive(Debug, Clone, PartialEq, Eq, bpls_rs_derive::Lexer)]
#[lexer(RealLexer)]
pub enum ExampleToken {
    #[pattern(' ' | '\t' | '\n' | '\r')]
    Whitespace,

    #[pattern('A'..'Z' | 'a'..'z')]
    String(String),

    #[pattern('0'..'9')]
    Number(u64),

    #[eoi]
    EOI,
}

/*impl TokenType<char> for ExampleToken {
    fn get_eoi() -> Self {
        ExampleToken::EOI
    }
}*/
