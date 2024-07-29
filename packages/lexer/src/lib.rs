pub mod derive;
pub mod tokens;

use logos::Logos;

pub use derive::add_tokens;
pub use logos::Lexer;
pub use tokens::Token;

pub struct PHPLexer {}

impl<'source> PHPLexer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn lex(&self, file_content: &'source str) -> Lexer<'source, Token> {
        Token::lexer(file_content)
    }
}
