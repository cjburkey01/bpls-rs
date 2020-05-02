pub trait Token {}

pub trait Lexer<TokenType>: Iterator<Item = TokenType>
where
    TokenType: Token,
{
    fn get_pos(&self) -> usize;
}
