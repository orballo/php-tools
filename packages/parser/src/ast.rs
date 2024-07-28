use lexer::Token;
use lexer_derive::Tokens;

#[derive(Debug, Tokens)]
#[tokens_from(lexer::Token)]
pub enum SyntaxKind {
    Expr,
    Stmt,
    Root,
}

// impl From<Token> for rowan::SyntaxKind {
//     fn from(kind: Token) -> Self {
//         Self(kind as u16)
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// enum Lang {}

// impl rowan::Language for Lang {
//     type Kind = Token;

//     fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
//         assert!(raw.0 <= ROOT as u16);
//         unsafe { std::mem::transmute::<u16, Token>(raw.0) }
//     }

//     fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
//         kind.into()
//     }
// }
