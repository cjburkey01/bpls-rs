use std::fmt::Debug;

pub trait TokenType<InputType> {
    fn get_eoi() -> Self;
}

pub trait Lexer<InputType, TT>: Iterator<Item = Result<Token<TT>, LexingError<InputType>>>
where
    TT: TokenType<InputType> + Debug + Clone + Eq,
{
    /// Get the current position of the lexer, in other words, this is the
    /// start index of the next token for this lexer.
    fn get_pos(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexingError<InputType> {
    UnexpectedInput { input: InputType, index: usize },
}

/// A token type and its position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<TT: Debug + Clone + PartialEq + Eq> {
    /// The starting index and exclusive ending index of this token.
    pub span: (usize, usize),

    /// The type of this token with its associated data.
    pub token_type: TT,
}

impl<TT: Debug + Clone + Eq> Token<TT> {
    /// Create a new token instance
    pub fn new(span: (usize, usize), token_type: TT) -> Self {
        Self { span, token_type }
    }

    /// Return the length of this token
    pub fn get_length(&self) -> usize {
        self.span.1 - self.span.0 + 1
    }
}
