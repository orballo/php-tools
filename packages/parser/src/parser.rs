use lexer::{Lexer, Token};
use nom::error::{Error, ErrorKind};
use nom::{Err, IResult};

use lexer::add_tokens;

// @TODO: Modify add_tokens to parse the derive attributes
// instead of hardcoding them inside the proc macro
#[add_tokens]
pub enum SyntaxKind {
    Expr,
    Stmt,
    Root,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Language {}

impl rowan::Language for Language {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= Self::Kind::Root as u16);
        unsafe { std::mem::transmute::<u16, Self::Kind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<Language>;
pub type SyntaxToken = rowan::SyntaxToken<Language>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

struct SyntaxError {}

impl SyntaxError {
    pub fn new(input: &[SyntaxKind]) -> Err<Error<&[SyntaxKind]>> {
        Err::Error(Error::new(input, ErrorKind::Tag))
    }
}

pub struct Parser<'source, 'cache> {
    tokens: Lexer<'source, Token>,
    builder: rowan::GreenNodeBuilder<'cache>,
}

impl<'source, 'cache> Parser<'source, 'cache> {
    pub fn new(tokens: Lexer<'source, Token>) -> Self {
        Self {
            tokens,
            builder: rowan::GreenNodeBuilder::new(),
        }
    }

    pub fn parse(&self, file_content: &'source str) -> rowan::GreenNode {
        dbg!(file_content);
        self.parse_string();
        todo!()
    }

    fn parse_string(&self) {
        todo!()
    }
}

// fn parse_string(input: &[SyntaxKind], tokens: ) -> IResult<&[SyntaxKind], SyntaxError> {
//     if let Some(SyntaxKind::String) = input.get(0) {
//         let content = s.trim_matches('\'').to_string();
//         let remaining = &input[1..];

//         Ok((remaining, Expr::String(content)))
//     } else {
//         Err(TagError::new(input))
//     }
// }

// fn parse_echo<'a>(input: &[SyntaxKind]) -> IResult<&[SyntaxKind], Expr> {
//     let (input, _) = tag(Token::Echo)(input)?;
//     let (input, _) = multispace0(input)?;
//     let (input, exprs) =
//         many0(|i| parse_string(i).or_else(|_| parse_integer(i)))(input)?;
//     Ok((input, Expr::Echo(exprs)))
// }
