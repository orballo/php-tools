// use crate::lexer::Token;
// use nom::error::{Error, ErrorKind};
// use nom::{Err, IResult};

// // @TODO: Define rowan Kinds to represent the AST

// struct TagError {}

// impl<'a> TagError {
//     pub fn new(input: &'a [Token<'a>]) -> Err<Error<&'a [Token<'a>]>> {
//         Err::Error(Error::new(input, ErrorKind::Tag))
//     }
// }

// fn parse_string<'a>(input: &'a [Token<'a>]) -> IResult<&'a [Token<'a>], Expr> {
//     if let Some(Token::String(s)) = input.get(0) {
//         let content = s.trim_matches('\'').to_string();
//         let remaining = &input[1..];

//         Ok((remaining, Expr::String(content)))
//     } else {
//         Err(TagError::new(input))
//     }
// }

// fn parse_echo<'a>(input: &'a [Token<'a>]) -> IResult<&'a [Token<'a>], Expr> {
//     let (input, _) = tag(Token::Echo)(input)?;
//     let (input, _) = multispace0(input)?;
//     let (input, exprs) =
//         many0(|i| parse_string(i).or_else(|_| parse_integer(i)))(input)?;
//     Ok((input, Expr::Echo(exprs)))
// }
