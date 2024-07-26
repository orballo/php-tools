use crate::lexer::Token;

impl<'source> From<Token<'source>> for rowan::SyntaxKind {
    fn from(kind: Token<'source>) -> Self {
        Self(kind.into())
    }
}
