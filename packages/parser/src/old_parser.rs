// use chumsky::Parser;

// #[repr(u16)]
// pub enum Token {
//     OpenTag,
//     CloseTag,
//     Newline,
//     Whitespace,
//     LParen,
//     RParen,
//     Comma,
//     Semicolon,
//     Plus,
//     Minus,
//     Star,
//     Slash,
//     Assign,
//     Integer,
//     String,
//     True,
//     False,
//     Null,
//     Echo,
//     Variable,
//     Identifier,
// }

// #[repr(u16)]
// pub enum Expressions {}

// pub enum Statements {}

// pub enum Script {
//     OpenTag,
//     CloseTag,
// }

// impl From<Token> for SyntaxKind {
//     fn from(token: Token) -> Self {
//         let token_u16 = token as u16;
//         assert!((token_u16 as u16) < Self::Expr as u16);
//         unsafe { std::mem::transmute(token_u16) }
//     }
// }

// impl From<SyntaxKind> for rowan::SyntaxKind {
//     fn from(kind: SyntaxKind) -> Self {
//         Self(kind as u16)
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum Language {}

// impl rowan::Language for Language {
//     type Kind = SyntaxKind;

//     fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
//         assert!(raw.0 <= Self::Kind::Script as u16);
//         unsafe { std::mem::transmute::<u16, Self::Kind>(raw.0) }
//     }

//     fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
//         kind.into()
//     }
// }

// pub type SyntaxNode = rowan::SyntaxNode<Language>;
// pub type SyntaxToken = rowan::SyntaxToken<Language>;
// pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

// #[derive(Debug)]
// pub struct PHPParser<'source, 'cache> {
//     source: &'source str,
//     tokens: Lexer<'source, Token>,
//     builder: rowan::GreenNodeBuilder<'cache>,
// }

// impl<'source, 'cache> PHPParser<'source, 'cache> {
//     pub fn new(source: &'source str, lexer: PHPLexer) -> Self {
//         let tokens = lexer.lex(source);

//         Self {
//             source,
//             tokens,
//             builder: rowan::GreenNodeBuilder::new(),
//         }
//     }

//     pub fn finish(self) -> SyntaxNode {
//         SyntaxNode::new_root(self.builder.finish())
//     }

//     pub fn parse(&'source mut self) {
//         let result = Self::parse_open_tag(
//             &mut self.builder,
//             &mut self.tokens,
//             &self.source,
//         )
//         .expect("Failed to parse open tag");

//         dbg!(result);
//     }

//     fn parse_open_tag(
//         builder: &mut rowan::GreenNodeBuilder,
//         tokens: &'source mut Lexer<'source, Token>,
//         source: &'source str,
//     ) -> IResult<&'source mut Lexer<'source, Token>, SyntaxKind> {
//         if let Some(token) = tokens.next() {
//             match token {
//                 Ok(token) => {
//                     let kind: SyntaxKind = token.into();

//                     if SyntaxKind::OpenTag == kind {
//                         let span = tokens.span();
//                         let text = &source[span];

//                         builder.token(kind.into(), text);
//                     }

//                     return Ok((tokens, kind));
//                 }
//                 Err(_) => {
//                     return Err(SyntaxError::new(tokens));
//                 }
//             }
//         } else {
//             return Err(SyntaxError::new(tokens));
//         }
//     }
// }

// // fn parse_string(input: &[SyntaxKind], tokens: ) -> IResult<&[SyntaxKind], SyntaxError> {
// //     if let Some(SyntaxKind::String) = input.get(0) {
// //         let content = s.trim_matches('\'').to_string();
// //         let remaining = &input[1..];

// //         Ok((remaining, Expr::String(content)))
// //     } else {
// //         Err(TagError::new(input))
// //     }
// // }

// // fn parse_echo<'a>(input: &[SyntaxKind]) -> IResult<&[SyntaxKind], Expr> {
// //     let (input, _) = tag(Token::Echo)(input)?;
// //     let (input, _) = multispace0(input)?;
// //     let (input, exprs) =
// //         many0(|i| parse_string(i).or_else(|_| parse_integer(i)))(input)?;
// //     Ok((input, Expr::Echo(exprs)))
// // }

// struct SyntaxError {}

// impl<'source> SyntaxError {
//     pub fn new(
//         input: &'source mut Lexer<'source, Token>,
//     ) -> nom::Err<Error<&mut Lexer<'source, Token>>> {
//         nom::Err::Error(Error::new(input, ErrorKind::Tag))
//     }
// }

// /** Parsers **/
// trait SubParser {
//     type Kind;

//     fn parser<'source>() -> impl Parser<&'source str, Self::Kind>;
// }

// fn open_tag() {}
