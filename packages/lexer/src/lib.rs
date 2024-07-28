pub mod derive;
pub mod tokens;

use logos::Logos;

pub use derive::add_tokens;
pub use logos::Lexer;
pub use tokens::Token;

pub fn lex<'source>(file_content: &'source str) -> Lexer<'source, Token> {
    Token::lexer(file_content)
}
